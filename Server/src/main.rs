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
                    state.players.write().await.remove(&player_id);
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

async fn game_tick_loop(state: ServerState) {
    let mut interval = tokio::time::interval(std::time::Duration::from_millis(16)); // 60 FPS
    
    loop {
        interval.tick().await;
        
        let rooms = state.game_rooms.read().await;
        for (_room_id, room) in rooms.iter() {
            let mut room_guard = room.write().await;
            
            // Обновляем позиции игроков
            for player_id in &room_guard.players {
                if let Some(player_arc) = state.players.read().await.get(player_id) {
                    let mut player = player_arc.write().await;
                    
                    // Применяем гравитацию и движение
                    player.position.z -= 9.81 * 0.016; // gravity
                    
                    // Синхронизируем
                    player.last_update = Utc::now();
                }
            }
            
            // Проверяем коллизии
            check_collisions(&mut room_guard, &state).await;
            
            // Обновляем состояние
            room_guard.tick_count += 1;
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

async fn check_collisions(room: &mut GameRoom, state: &ServerState) {
    // Проверяем столкновения между игроками и пулями
    for bullet in &room.bullets.clone() {
        for player_id in &room.players {
            if let Some(player_arc) = state.players.read().await.get(player_id) {
                let mut player = player_arc.write().await;
                
                let dist = ((player.position.x - bullet.position.x).powi(2) +
                           (player.position.y - bullet.position.y).powi(2) +
                           (player.position.z - bullet.position.z).powi(2)).sqrt();
                
                if dist < 2.0 && &bullet.owner != player_id {
                    player.health -= bullet.damage as i32;
                    log::info!("💥 {} hit {} for {} damage", bullet.owner, player_id, bullet.damage);
                    
                    if player.health <= 0 {
                        player.alive = false;
                        log::info!("💀 {} died", player_id);
                    }
                }
            }
        }
    }
    
    room.bullets.retain(|b| b.lifetime > 0.0);
}
