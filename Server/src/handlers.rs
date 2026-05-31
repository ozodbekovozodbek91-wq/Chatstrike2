use crate::database::DatabasePool;
use crate::models::*;
use crate::ServerState;
use serde_json::{json, Value};
use uuid::Uuid;
use chrono::Utc;

pub async fn handle_command(
    cmd: ServerCommand,
    player_id: &str,
    state: &ServerState,
) -> Value {
    match cmd {
        ServerCommand::Login { username, password } => {
            handle_login(username, password, player_id, state).await
        }
        ServerCommand::Register { username, password, email } => {
            handle_register(username, password, email, state).await
        }
        ServerCommand::CreateRoom { map_name, max_players } => {
            handle_create_room(map_name, max_players, player_id, state).await
        }
        ServerCommand::JoinRoom { room_id } => {
            handle_join_room(&room_id, player_id, state).await
        }
        ServerCommand::LeaveRoom => {
            handle_leave_room(player_id, state).await
        }
        ServerCommand::Move { direction } => {
            handle_move(direction, player_id, state).await
        }
        ServerCommand::Shoot { target, weapon } => {
            handle_shoot(target, weapon, player_id, state).await
        }
        ServerCommand::Reload => {
            handle_reload(player_id, state).await
        }
        ServerCommand::ChangeWeapon { weapon } => {
            handle_change_weapon(weapon, player_id, state).await
        }
        ServerCommand::GetPlayers => {
            handle_get_players(player_id, state).await
        }
        ServerCommand::GetRooms => {
            handle_get_rooms(state).await
        }
        ServerCommand::GetStats => {
            handle_get_stats(player_id, state).await
        }
        ServerCommand::GetLeaderboard { top } => {
            handle_get_leaderboard(top, state).await
        }
        ServerCommand::GetMatchHistory { limit } => {
            handle_get_match_history(player_id, limit, state).await
        }
        ServerCommand::Ping => {
            json!({
                "status": "ok",
                "pong": true,
                "timestamp": Utc::now()
            })
        }
        ServerCommand::UpdatePosition { position, rotation } => {
            handle_update_position(position, rotation, player_id, state).await
        }
        ServerCommand::Chat { message } => {
            handle_chat(&message, player_id, state).await
        }
        ServerCommand::EndMatch { kills, deaths, assists, score, won, duration } => {
            handle_end_match(kills, deaths, assists, score, won, duration, player_id, state).await
        }
        ServerCommand::GetLeaderboardByType { board_type, limit } => {
            handle_get_leaderboard_by_type(&board_type, limit, state).await
        }
    }
}

async fn handle_login(
    username: String,
    password: String,
    player_id: &str,
    state: &ServerState,
) -> Value {
    log::info!("🔐 Login attempt: {}", username);
    
    match state.db.authenticate_user(&username, &password).await {
        Ok(user) => {
            let player = PlayerData::new(player_id.to_string(), username.clone());
            state.players.write().await.insert(
                player_id.to_string(),
                std::sync::Arc::new(tokio::sync::RwLock::new(player)),
            );
            
            log::info!("✅ {} logged in successfully (ID: {})", username, user.id);
            
            json!({
                "status": "ok",
                "message": "Login successful",
                "player_id": player_id,
                "user_id": user.id,
                "user": {
                    "username": user.username,
                    "level": user.level,
                    "rank": user.rank,
                    "experience": user.experience,
                    "rank_points": user.rank_points,
                }
            })
        }
        Err(e) => {
            log::warn!("❌ Login failed for {}: {}", username, e);
            json!({
                "status": "error",
                "message": format!("Login failed: {}", e),
                "code": 401
            })
        }
    }
}

async fn handle_register(
    username: String,
    password: String,
    email: String,
    state: &ServerState,
) -> Value {
    log::info!("📝 Register attempt: {} ({})", username, email);
    
    match state.db.create_user(&username, &password, &email).await {
        Ok(user) => {
            log::info!("✅ User {} registered successfully", username);
            json!({
                "status": "ok",
                "message": "Registration successful",
                "user_id": user.id,
                "username": username,
                "email": email
            })
        }
        Err(e) => {
            log::warn!("❌ Registration failed: {}", e);
            json!({
                "status": "error",
                "message": format!("Registration failed: {}", e),
                "code": 400
            })
        }
    }
}

async fn handle_create_room(
    map_name: String,
    max_players: usize,
    player_id: &str,
    state: &ServerState,
) -> Value {
    let room_id = Uuid::new_v4().to_string();
    let room_name = format!("{}'s Room", player_id);
    
    let mut room = GameRoom::new(
        room_id.clone(),
        room_name.clone(),
        map_name.clone(),
        max_players,
    );
    room.players.push(player_id.to_string());
    room.state = GameState::Waiting;
    
    state.game_rooms.write().await.insert(
        room_id.clone(),
        std::sync::Arc::new(tokio::sync::RwLock::new(room)),
    );
    
    log::info!("🎮 Room created: {} ({}), map: {}, max: {}", room_name, room_id, map_name, max_players);
    
    json!({
        "status": "ok",
        "message": "Room created",
        "room_id": room_id,
        "map": map_name,
        "max_players": max_players
    })
}

async fn handle_join_room(
    room_id: &str,
    player_id: &str,
    state: &ServerState,
) -> Value {
    let rooms = state.game_rooms.read().await;
    
    if let Some(room_arc) = rooms.get(room_id) {
        let mut room = room_arc.write().await;
        
        if room.can_join() {
            room.players.push(player_id.to_string());
            log::info!("✅ Player {} joined room {}", player_id, room_id);
            
            json!({
                "status": "ok",
                "message": "Joined room",
                "room_id": room_id,
                "players_count": room.players.len(),
                "map": room.map_name
            })
        } else {
            log::warn!("❌ Cannot join room {}: full or not waiting", room_id);
            json!({
                "status": "error",
                "message": "Cannot join room: full or not waiting",
                "code": 409
            })
        }
    } else {
        json!({
            "status": "error",
            "message": "Room not found",
            "code": 404
        })
    }
}

async fn handle_leave_room(player_id: &str, state: &ServerState) -> Value {
    let mut rooms = state.game_rooms.write().await;
    
    for (room_id, room_arc) in rooms.iter_mut() {
        let mut room = room_arc.write().await;
        if let Some(pos) = room.players.iter().position(|p| p == player_id) {
            room.players.remove(pos);
            log::info!("✅ Player {} left room {}", player_id, room_id);
            
            if room.players.is_empty() {
                log::info!("🗑️ Room {} is now empty", room_id);
            }
            
            return json!({
                "status": "ok",
                "message": "Left room"
            });
        }
    }
    
    json!({
        "status": "error",
        "message": "Not in any room",
        "code": 404
    })
}

async fn handle_move(direction: Vector3, player_id: &str, state: &ServerState) -> Value {
    if let Some(player_arc) = state.players.read().await.get(player_id) {
        let mut player = player_arc.write().await;
        player.velocity = direction;
        
        log::debug!("🚶 Player {} moving: {:?}", player_id, direction);
        
        json!({
            "status": "ok",
            "message": "Position updated",
            "velocity": {
                "x": direction.x,
                "y": direction.y,
                "z": direction.z
            }
        })
    } else {
        json!({
            "status": "error",
            "message": "Player not found",
            "code": 404
        })
    }
}

async fn handle_shoot(
    target: Vector3,
    weapon: WeaponType,
    player_id: &str,
    state: &ServerState,
) -> Value {
    if let Some(player_arc) = state.players.read().await.get(player_id) {
        let mut player = player_arc.write().await;
        
        if !player.alive {
            return json!({
                "status": "error",
                "message": "You are dead",
                "code": 409
            });
        }
        
        let damage = weapon.damage();
        let bullet_id = Uuid::new_v4().to_string();
        
        let bullet = Bullet {
            id: bullet_id,
            position: player.position.clone(),
            velocity: Vector3 {
                x: target.x * 50.0,
                y: target.y * 50.0,
                z: target.z * 50.0,
            },
            damage,
            owner: player_id.to_string(),
            lifetime: 10.0,
        };
        
        log::info!("💥 {} shot with {:?} at {:?}", player_id, weapon, target);
        
        json!({
            "status": "ok",
            "message": "Shot fired",
            "damage": damage,
            "weapon": format!("{:?}", weapon)
        })
    } else {
        json!({
            "status": "error",
            "message": "Player not found",
            "code": 404
        })
    }
}

async fn handle_reload(player_id: &str, state: &ServerState) -> Value {
    if let Some(player_arc) = state.players.read().await.get(player_id) {
        let mut player = player_arc.write().await;
        let magazine_size = player.current_weapon.magazine_size();
        
        log::info!("🔄 {} reloading {:?}", player_id, player.current_weapon);
        
        json!({
            "status": "ok",
            "message": "Reloading",
            "weapon": format!("{:?}", player.current_weapon),
            "magazine_size": magazine_size
        })
    } else {
        json!({
            "status": "error",
            "message": "Player not found",
            "code": 404
        })
    }
}

async fn handle_change_weapon(weapon: WeaponType, player_id: &str, state: &ServerState) -> Value {
    if let Some(player_arc) = state.players.read().await.get(player_id) {
        let mut player = player_arc.write().await;
        player.current_weapon = weapon.clone();
        
        log::info!("🔫 {} switched to {:?}", player_id, weapon);
        
        json!({
            "status": "ok",
            "message": "Weapon changed",
            "weapon": format!("{:?}", weapon)
        })
    } else {
        json!({
            "status": "error",
            "message": "Player not found",
            "code": 404
        })
    }
}

async fn handle_get_players(player_id: &str, state: &ServerState) -> Value {
    let players = state.players.read().await;
    let player_list: Vec<Value> = players
        .iter()
        .map(|(id, player_arc)| {
            json!({
                "id": id,
                "username": player_arc.blocking_read().username,
                "alive": player_arc.blocking_read().alive,
                "health": player_arc.blocking_read().health,
                "ping": player_arc.blocking_read().ping,
            })
        })
        .collect();
    
    json!({
        "status": "ok",
        "message": "Players list",
        "count": player_list.len(),
        "players": player_list
    })
}

async fn handle_get_rooms(state: &ServerState) -> Value {
    let rooms = state.game_rooms.read().await;
    let room_list: Vec<Value> = rooms
        .iter()
        .map(|(id, room_arc)| {
            let room = room_arc.blocking_read();
            json!({
                "id": id,
                "name": room.name,
                "map": room.map_name,
                "players": room.players.len(),
                "max_players": room.max_players,
                "state": format!("{:?}", room.state)
            })
        })
        .collect();
    
    json!({
        "status": "ok",
        "message": "Rooms list",
        "count": room_list.len(),
        "rooms": room_list
    })
}

async fn handle_get_stats(player_id: &str, state: &ServerState) -> Value {
    if let Some(player_arc) = state.players.read().await.get(player_id) {
        let player = player_arc.read().await;
        
        json!({
            "status": "ok",
            "message": "Player stats",
            "stats": {
                "kills": player.kills,
                "deaths": player.deaths,
                "assists": player.assists,
                "score": player.score,
                "kd_ratio": if player.deaths > 0 {
                    player.kills as f32 / player.deaths as f32
                } else {
                    player.kills as f32
                }
            }
        })
    } else {
        json!({
            "status": "error",
            "message": "Player not found",
            "code": 404
        })
    }
}

async fn handle_get_leaderboard(top: i64, state: &ServerState) -> Value {
    match state.db.get_global_leaderboard(top).await {
        Ok(stats) => {
            let leaderboard: Vec<Value> = stats
                .iter()
                .enumerate()
                .map(|(idx, stat)| {
                    json!({
                        "position": idx + 1,
                        "user_id": stat.user_id,
                        "kills": stat.kills,
                        "deaths": stat.deaths,
                        "score": stat.score,
                        "kd_ratio": if stat.deaths > 0 {
                            stat.kills as f32 / stat.deaths as f32
                        } else {
                            stat.kills as f32
                        },
                        "matches": stat.matches_played
                    })
                })
                .collect();
            
            json!({
                "status": "ok",
                "message": "Global leaderboard",
                "leaderboard": leaderboard
            })
        }
        Err(e) => {
            json!({
                "status": "error",
                "message": format!("Failed to fetch leaderboard: {}", e),
                "code": 500
            })
        }
    }
}

async fn handle_get_leaderboard_by_type(
    board_type: &str,
    limit: i64,
    state: &ServerState,
) -> Value {
    match board_type {
        "kills" => {
            match state.db.get_kills_leaderboard(limit).await {
                Ok(stats) => {
                    let leaderboard: Vec<Value> = stats
                        .iter()
                        .enumerate()
                        .map(|(idx, stat)| {
                            json!({
                                "position": idx + 1,
                                "user_id": stat.user_id,
                                "kills": stat.kills,
                            })
                        })
                        .collect();
                    
                    json!({
                        "status": "ok",
                        "message": "Kills leaderboard",
                        "leaderboard": leaderboard
                    })
                }
                Err(e) => json!({
                    "status": "error",
                    "message": format!("Failed: {}", e),
                    "code": 500
                })
            }
        }
        "accuracy" => {
            match state.db.get_accuracy_leaderboard(limit).await {
                Ok(stats) => {
                    let leaderboard: Vec<Value> = stats
                        .iter()
                        .enumerate()
                        .map(|(idx, stat)| {
                            json!({
                                "position": idx + 1,
                                "user_id": stat.user_id,
                                "accuracy": format!("{}%", (stat.accuracy * 100.0) as i32),
                                "matches": stat.matches_played
                            })
                        })
                        .collect();
                    
                    json!({
                        "status": "ok",
                        "message": "Accuracy leaderboard",
                        "leaderboard": leaderboard
                    })
                }
                Err(e) => json!({
                    "status": "error",
                    "message": format!("Failed: {}", e),
                    "code": 500
                })
            }
        }
        _ => json!({
            "status": "error",
            "message": "Invalid leaderboard type",
            "code": 400
        })
    }
}

async fn handle_get_match_history(
    player_id: &str,
    limit: i64,
    state: &ServerState,
) -> Value {
    // Получаем user_id из сессии
    if let Some(player_arc) = state.players.read().await.get(player_id) {
        // В реальной системе здесь должен быть user_id из базы
        // Для примера используем player_id как индексатор
        match state.db.get_match_history(1, limit).await {
            Ok(matches) => {
                let history: Vec<Value> = matches
                    .iter()
                    .map(|m| {
                        json!({
                            "map": m.map_name,
                            "kills": m.kills,
                            "deaths": m.deaths,
                            "score": m.score,
                            "won": m.won,
                            "date": m.ended_at
                        })
                    })
                    .collect();
                
                json!({
                    "status": "ok",
                    "message": "Match history",
                    "matches": history
                })
            }
            Err(e) => json!({
                "status": "error",
                "message": format!("Failed: {}", e),
                "code": 500
            })
        }
    } else {
        json!({
            "status": "error",
            "message": "Player not found",
            "code": 404
        })
    }
}

async fn handle_update_position(
    position: Vector3,
    rotation: Vector3,
    player_id: &str,
    state: &ServerState,
) -> Value {
    if let Some(player_arc) = state.players.read().await.get(player_id) {
        let mut player = player_arc.write().await;
        player.position = position.clone();
        player.rotation = rotation.clone();
        player.last_update = Utc::now();
        
        json!({
            "status": "ok",
            "message": "Position synchronized"
        })
    } else {
        json!({
            "status": "error",
            "message": "Player not found",
            "code": 404
        })
    }
}

async fn handle_chat(message: &str, player_id: &str, state: &ServerState) -> Value {
    log::info!("💬 [{}]: {}", player_id, message);
    
    let net_msg = crate::NetworkMessage {
        message_type: "chat".to_string(),
        payload: json!({
            "sender": player_id,
            "message": message,
        }),
        timestamp: Utc::now(),
    };
    
    let _ = state.tx.send(net_msg);
    
    json!({
        "status": "ok",
        "message": "Message sent"
    })
}

async fn handle_end_match(
    kills: i32,
    deaths: i32,
    assists: i32,
    score: i32,
    won: bool,
    duration: i64,
    player_id: &str,
    state: &ServerState,
) -> Value {
    if let Some(player_arc) = state.players.read().await.get(player_id) {
        let player = player_arc.read().await;
        
        // Записываем матч в БД
        match state.db.record_match(
            1, // user_id (в реальной системе должен быть из сессии)
            "room_1",
            "Dust2",
            kills,
            deaths,
            assists,
            score,
            won,
            duration,
        ).await {
            Ok(match_record) => {
                log::info!("📊 Match recorded: {} kills, {} deaths", kills, deaths);
                
                json!({
                    "status": "ok",
                    "message": "Match recorded",
                    "match_id": match_record.id,
                    "xp_gained": score / 10,
                    "rank_points_gained": if won { 25 } else { 5 }
                })
            }
            Err(e) => json!({
                "status": "error",
                "message": format!("Failed to record match: {}", e),
                "code": 500
            })
        }
    } else {
        json!({
            "status": "error",
            "message": "Player not found",
            "code": 404
        })
    }
}
