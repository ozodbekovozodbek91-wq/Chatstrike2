use serde::{Deserialize, Serialize};
use crate::ui_types::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LobbyScreen {
    pub rooms: Vec<RoomCard>,
    pub selected_room: Option<String>,
    pub player_name: String,
    pub player_level: u32,
    pub player_rank: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoomCard {
    pub id: String,
    pub name: String,
    pub map: String,
    pub creator: String,
    pub current_players: usize,
    pub max_players: usize,
    pub ping: u32,
    pub map_preview: String, // URL
}

impl LobbyScreen {
    pub fn new(player_name: String) -> Self {
        LobbyScreen {
            rooms: Vec::new(),
            selected_room: None,
            player_name,
            player_level: 1,
            player_rank: "Unranked".to_string(),
        }
    }
    
    pub fn render_json(&self) -> serde_json::Value {
        let rooms_json: Vec<serde_json::Value> = self.rooms
            .iter()
            .map(|room| {
                let status = if room.current_players >= room.max_players {
                    "FULL"
                } else if room.current_players > 0 {
                    "PLAYERS"
                } else {
                    "EMPTY"
                };
                
                let ping_color = if room.ping < 50 {
                    "#00CC66"
                } else if room.ping < 100 {
                    "#FFB627"
                } else {
                    "#FF3333"
                };
                
                serde_json::json!({
                    "id": room.id,
                    "name": room.name,
                    "map": room.map,
                    "creator": room.creator,
                    "players": format!("{}/{}", room.current_players, room.max_players),
                    "ping": room.ping,
                    "ping_color": ping_color,
                    "status": status,
                    "map_preview": room.map_preview,
                    "selected": self.selected_room == Some(room.id.clone())
                })
            })
            .collect();
        
        serde_json::json!({
            "menu_state": "Lobby",
            "layout": "three-column",
            "left_panel": {
                "title": "Available Rooms",
                "rooms": rooms_json,
                "total": self.rooms.len()
            },
            "center_panel": {
                "title": "Room Details",
                "selected_room": self.selected_room,
                "sections": [
                    {"name": "Map Info"},
                    {"name": "Players"},
                    {"name": "Chat"}
                ]
            },
            "right_panel": {
                "title": "Player Info",
                "player": {
                    "name": self.player_name,
                    "level": self.player_level,
                    "rank": self.player_rank,
                    "avatar": "url_to_avatar"
                }
            },
            "buttons": [
                {"label": "JOIN GAME", "style": "primary", "enabled": self.selected_room.is_some()},
                {"label": "CREATE ROOM", "style": "secondary", "enabled": true},
                {"label": "REFRESH", "style": "tertiary", "enabled": true}
            ]
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeaderboardScreen {
    pub entries: Vec<LeaderboardEntry>,
    pub current_tab: String, // "Global", "Kills", "Accuracy", "Season"
    pub current_page: usize,
    pub entries_per_page: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeaderboardEntry {
    pub position: usize,
    pub username: String,
    pub level: u32,
    pub rank: String,
    pub kills: i32,
    pub deaths: i32,
    pub wins: i32,
    pub rating: i32,
    pub accuracy: f32,
}

impl LeaderboardScreen {
    pub fn new() -> Self {
        LeaderboardScreen {
            entries: Vec::new(),
            current_tab: "Global".to_string(),
            current_page: 0,
            entries_per_page: 20,
        }
    }
    
    pub fn render_json(&self) -> serde_json::Value {
        let entries_json: Vec<serde_json::Value> = self.entries
            .iter()
            .map(|entry| {
                let medal = match entry.position {
                    1 => "🥇",
                    2 => "🥈",
                    3 => "🥉",
                    _ => " ",
                };
                
                let rank_color = match entry.rank.as_str() {
                    "Bronze" => "#CD7F32",
                    "Silver" => "#C0C0C0",
                    "Gold" => "#FFD700",
                    "Platinum" => "#E5E4E2",
                    "Diamond" => "#B9F2FF",
                    "Master" => "#FF6B9D",
                    "Grandmaster" => "#FFD700",
                    _ => "#FFFFFF",
                };
                
                serde_json::json!({
                    "position": entry.position,
                    "medal": medal,
                    "username": entry.username,
                    "level": entry.level,
                    "rank": entry.rank,
                    "rank_color": rank_color,
                    "kills": entry.kills,
                    "deaths": entry.deaths,
                    "wins": entry.wins,
                    "rating": entry.rating,
                    "accuracy": format!("{}%", (entry.accuracy * 100.0) as i32),
                    "kd_ratio": if entry.deaths > 0 {
                        format!("{:.2}", entry.kills as f32 / entry.deaths as f32)
                    } else {
                        entry.kills.to_string()
                    }
                })
            })
            .collect();
        
        serde_json::json!({
            "menu_state": "Leaderboards",
            "tabs": [
                {"name": "Global Ranking", "active": self.current_tab == "Global"},
                {"name": "Kills Leaderboard", "active": self.current_tab == "Kills"},
                {"name": "Accuracy Leaderboard", "active": self.current_tab == "Accuracy"},
                {"name": "This Season", "active": self.current_tab == "Season"}
            ],
            "entries": entries_json,
            "pagination": {
                "current_page": self.current_page,
                "total_entries": self.entries.len(),
                "entries_per_page": self.entries_per_page,
                "total_pages": (self.entries.len() + self.entries_per_page - 1) / self.entries_per_page
            }
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileScreen {
    pub username: String,
    pub level: u32,
    pub rank: String,
    pub rating: i32,
    pub experience: i32,
    pub avatar_url: String,
    pub stats: PlayerProfileStats,
    pub achievements: Vec<Achievement>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerProfileStats {
    pub total_kills: i32,
    pub total_deaths: i32,
    pub total_wins: i32,
    pub total_matches: i32,
    pub playtime_hours: i32,
    pub average_accuracy: f32,
    pub kd_ratio: f32,
    pub win_rate: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Achievement {
    pub id: String,
    pub title: String,
    pub description: String,
    pub icon: String,
    pub rarity: String, // "Common", "Rare", "Epic", "Legendary"
    pub unlocked: bool,
    pub progress: f32, // 0.0 - 1.0
}

impl ProfileScreen {
    pub fn new(username: String) -> Self {
        ProfileScreen {
            username,
            level: 1,
            rank: "Unranked".to_string(),
            rating: 0,
            experience: 0,
            avatar_url: "".to_string(),
            stats: PlayerProfileStats {
                total_kills: 0,
                total_deaths: 0,
                total_wins: 0,
                total_matches: 0,
                playtime_hours: 0,
                average_accuracy: 0.0,
                kd_ratio: 0.0,
                win_rate: 0.0,
            },
            achievements: Vec::new(),
        }
    }
    
    pub fn render_json(&self) -> serde_json::Value {
        let achievements_json: Vec<serde_json::Value> = self.achievements
            .iter()
            .map(|ach| {
                let rarity_color = match ach.rarity.as_str() {
                    "Common" => "#CCCCCC",
                    "Rare" => "#0099FF",
                    "Epic" => "#A335EE",
                    "Legendary" => "#FFD700",
                    _ => "#FFFFFF",
                };
                
                serde_json::json!({
                    "id": ach.id,
                    "title": ach.title,
                    "description": ach.description,
                    "icon": ach.icon,
                    "rarity": ach.rarity,
                    "rarity_color": rarity_color,
                    "unlocked": ach.unlocked,
                    "progress": ach.progress
                })
            })
            .collect();
        
        serde_json::json!({
            "menu_state": "Profile",
            "header": {
                "avatar": self.avatar_url,
                "username": self.username,
                "level": self.level,
                "rank": self.rank,
                "rating": self.rating,
                "experience": self.experience,
                "next_level_exp": (self.level * 1000) as i32
            },
            "stats": {
                "total_kills": self.stats.total_kills,
                "total_deaths": self.stats.total_deaths,
                "total_wins": self.stats.total_wins,
                "total_matches": self.stats.total_matches,
                "playtime_hours": self.stats.playtime_hours,
                "average_accuracy": format!("{}%", (self.stats.average_accuracy * 100.0) as i32),
                "kd_ratio": format!("{:.2}", self.stats.kd_ratio),
                "win_rate": format!("{}%", (self.stats.win_rate * 100.0) as i32)
            },
            "achievements": {
                "total": self.achievements.len(),
                "unlocked": self.achievements.iter().filter(|a| a.unlocked).count(),
                "list": achievements_json
            }
        })
    }
}
