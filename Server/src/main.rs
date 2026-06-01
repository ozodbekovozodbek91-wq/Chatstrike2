use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::sync::{RwLock, broadcast};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::collections::HashMap;
use uuid::Uuid;
use chrono::Utc;

mod models;
mod handlers;
mod database;
mod anticheats;
mod networking;

use models::*;
use handlers::*;
use database::*;
use anticheats::*;
use networking::*;

pub type PlayerMap = Arc<RwLock<HashMap<String, Arc<RwLock<PlayerData>>>>>;
pub type GameStateMap = Arc<RwLock<HashMap<String, Arc<RwLock<GameRoom>>>>>;

#[derive(Clone)]
pub struct ServerState {
    pub players: PlayerMap,
    pub game_rooms: GameStateMap,
    pub db: Arc<Database>,
    pub anticheat: Arc<AntiCheatSystem>,
    pub tx: broadcast::Sender<NetworkMessage>,
}

#[tokio::main]
async fn main() {
    env_logger::init();
    
    let listener = TcpListener::bind("0.0.0.0:9000")
        .await
        .expect("Failed to bind to port 9000");
    
    log::info!("🎮 ===== CHATSTRIKE2 SERVER STARTED =====");
    log::info!("🚀 Server listening on 0.0.0.0:9000");
    log::info!("🔐 Anti-Cheat System: ACTIVE");
    log::info!("💾 Database: INITIALIZED");
    log::info!("🌐 Network Protocol: TCP+JSON");
    log::info!("======================================");
    
    let players: PlayerMap = Arc::new(RwLock::new(HashMap::new()));
    let game_rooms: GameStateMap = Arc::new(RwLock::new(HashMap::new()));
    let db = Arc::new(Database::new().await);
    let anticheat = Arc::new(AntiCheatSystem::new());
    let (tx, _) = broadcast::channel(1000);
    
    let state = ServerState {
        players: Arc::clone(&players),
        game_rooms: Arc::clone(&game_rooms),
        db,
        anticheat,
        tx,
    };
    
    // Запускаем фоновые задачи
    let state_tick = state.clone();
    tokio::spawn(async move {
        game_tick_loop(state_tick).await;
    });
    
    let state_stats = state.clone();
    tokio::spawn(async move {
        stats_loop(state_stats).await;
    });
    
    loop {
        let (socket, addr) = listener.accept().await.expect("Failed to accept connection");
        log::info!("✅ New connection: {}", addr);
        
        let state = state.clone();
        
        tokio::spawn(async move {
            if let Err(e) = handle_client(socket, addr, state).await {
                log::error!("❌ Client error [{}]: {}", addr, e);
            }
        });
    }
}

async fn handle_client(
    socket: TcpStream,
    addr: std::net::SocketAddr,
    state: ServerState,
) -> Result<(), Box<dyn std::error::Error>> {
    let (mut reader, mut writer) = socket.into_split();
    let mut buffer = [0u8; 4096];
    let player_id = Uuid::new_v4().to_string();
    let client = Arc::new(ClientConnection::new(player_id.clone(), addr));
    
    let mut rx = state.tx.subscribe();
    
    loop {
        tokio::select! {
            n = reader.read(&mut buffer) => {
                let n = n?;
                if n == 0 {
                    log::info!("🔌 Player {} disconnected", player_id);
                    cleanup_player(&player_id, &state).await;
                    return Ok(());
                }
                
                let request = String::from_utf8_lossy(&buffer[..n]);
                
                for line in request.lines() {
                    if line.is_empty() { continue; }
                    
                    match serde_json::from_str::<ServerCommand>(line) {
                        Ok(cmd) => {
                            let response = handle_command(
                                cmd,
                                &player_id,
                                &state
                            ).await;
                            
                            let json_response = serde_json::to_string(&response)?;
                            writer.write_all(json_response.as_bytes()).await?;
                            writer.write_all(b"\n").await?;
                        },
                        Err(e) => {
                            log::warn!("❌ JSON Parse error: {}", e);
                            let err = ErrorResponse {
                                status: "error".to_string(),
                                message: format!("Invalid JSON: {}", e),
                                code: 400,
                            };
                            let json = serde_json::to_string(&err)?;
                            writer.write_all(json.as_bytes()).await?;
                            writer.write_all(b"\n").await?;
                        }
                    }
                }
            }
            
            msg = rx.recv() => {
                if let Ok(net_msg) = msg {
                    let json = serde_json::to_string(&net_msg)?;
                    writer.write_all(json.as_bytes()).await?;
                    writer.write_all(b"\n").await?;
                }
            }
        }
    }
}

// ✅ ИСПРАВЛЕНИЕ: Очистка игрока при отключении
async fn cleanup_player(player_id: &str, state: &ServerState) {
    // Удаляем игрока из всех комнат
    let mut rooms = state.game_rooms.write().await;
    let mut empty_rooms = Vec::new();
    
    for (room_id, room_arc) in rooms.iter_mut() {
        let mut room = room_arc.write().await;
        room.players.retain(|p| p != player_id);
        
        // Отмечаем пустые комнаты для удаления
        if room.players.is_empty() {
            empty_rooms.push(room_id.clone());
            log::info!("🗑️ Clearing empty room: {}", room_id);
        }
    }
    
    // Удаляем пустые комнаты
    for room_id in empty_rooms {
        rooms.remove(&room_id);
    }
    drop(rooms);
    
    // Удаляем самого игрока
    state.players.write().await.remove(player_id);
    log::info!("✅ Cleanup completed for player: {}", player_id);
}

async fn game_tick_loop(state: ServerState) {
    let mut interval = tokio::time::interval(std::time::Duration::from_millis(16)); // 60 FPS
    
    loop {
        interval.tick().await;
        
        // ✅ ИСПРАВЛЕНИЕ: Получаем список комнат один раз
        let room_ids: Vec<String> = {
            let rooms = state.game_rooms.read().await;
            rooms.keys().cloned().collect()
        };
        
        for room_id in room_ids {
            if let Some(room_arc) = state.game_rooms.read().await.get(&room_id) {
                let mut room_guard = room_arc.write().await;
                
                // Обновляем позиции игроков
                let player_ids: Vec<String> = room_guard.players.iter().cloned().collect();
                drop(room_guard); // Освобождаем lock комнаты
                
                for player_id in player_ids {
                    if let Some(player_arc) = state.players.read().await.get(&player_id) {
                        let mut player = player_arc.write().await;
                        
                        // ✅ ИСПРАВЛЕНИЕ: Гравитация с проверкой
                        if player.position.z > 0.0 {
                            player.position.z = (player.position.z - 9.81 * 0.016).max(0.0);
                        } else {
                            player.position.z = 0.0; // На земле
                        }
                        
                        // Синхронизируем
                        player.last_update = Utc::now();
                    }
                }
                
                // Проверяем коллизии после обновления позиций
                if let Some(room_arc) = state.game_rooms.read().await.get(&room_id) {
                    let mut room_guard = room_arc.write().await;
                    check_collisions(&mut room_guard, &state).await;
                    
                    // Обновляем состояние
                    room_guard.tick_count += 1;
                }
            }
        }
    }
}

async fn stats_loop(state: ServerState) {
    let mut interval = tokio::time::interval(std::time::Duration::from_secs(30));
    
    loop {
        interval.tick().await;
        
        let players = state.players.read().await;
        let rooms = state.game_rooms.read().await;
        
        log::info!("📊 ===== SERVER STATS =====");
        log::info!("👥 Connected Players: {}", players.len());
        log::info!("🎮 Active Game Rooms: {}", rooms.len());
        log::info!("🔐 Anti-Cheat Events: {}", state.anticheat.get_total_events());
        log::info!("=========================");
    }
}

// ✅ ИСПРАВЛЕНИЕ: Безопасная проверка колизий
async fn check_collisions(room: &mut GameRoom, state: &ServerState) {
    // Проверяем столкновения между игроками и пулями
    let bullets = room.bullets.clone();
    
    for bullet in &bullets {
        for player_id in &room.players {
            if let Some(player_arc) = state.players.read().await.get(player_id) {
                let mut player = player_arc.write().await;
                
                // ✅ ИСПРАВЛЕНИЕ: Используем squared distance для избежания sqrt
                let dist_sq = (player.position.x - bullet.position.x).powi(2) +
                             (player.position.y - bullet.position.y).powi(2) +
                             (player.position.z - bullet.position.z).powi(2);
                
                const COLLISION_RADIUS_SQ: f32 = 4.0; // 2.0^2
                
                // ✅ ИСПРАВЛЕНИЕ: Проверяем что это не собственная пуля и расстояние в пределах
                if dist_sq < COLLISION_RADIUS_SQ && dist_sq > 0.01 && &bullet.owner != player_id {
                    let dist = dist_sq.sqrt();
                    player.health -= bullet.damage as i32;
                    log::info!("💥 {} hit {} for {} damage at distance {:.2}", 
                               bullet.owner, player_id, bullet.damage, dist);
                    
                    if player.health <= 0 {
                        player.alive = false;
                        log::info!("💀 {} died", player_id);
                    }
                }
            }
        }
    }
    
    // Очищаем мертвые пули
    room.bullets.retain(|b| b.lifetime > 0.0);
}
