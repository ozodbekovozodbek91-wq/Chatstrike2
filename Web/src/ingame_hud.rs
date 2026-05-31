use serde::{Deserialize, Serialize};
use crate::ui_types::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InGameHUD {
    pub health: i32,
    pub max_health: i32,
    pub armor: i32,
    pub max_armor: i32,
    pub magazine_ammo: i32,
    pub reserve_ammo: i32,
    pub weapon_name: String,
    pub money: i32,
    pub score: i32,
    pub kills: i32,
    pub deaths: i32,
    pub assists: i32,
}

impl InGameHUD {
    pub fn new() -> Self {
        InGameHUD {
            health: 100,
            max_health: 100,
            armor: 0,
            max_armor: 100,
            magazine_ammo: 30,
            reserve_ammo: 120,
            weapon_name: "M4A1".to_string(),
            money: 2500,
            score: 0,
            kills: 0,
            deaths: 0,
            assists: 0,
        }
    }
    
    pub fn get_health_bar_color(&self) -> String {
        let health_percent = self.health as f32 / self.max_health as f32;
        
        match health_percent {
            x if x >= 0.75 => "#00CC66".to_string(),  // Green
            x if x >= 0.5 => "#FFB627".to_string(),   // Yellow
            x if x >= 0.25 => "#FF9900".to_string(),  // Orange
            _ => "#FF3333".to_string(),                // Red
        }
    }
    
    pub fn render_json(&self) -> serde_json::Value {
        serde_json::json!({
            "hud_state": "InGame",
            "top_bar": {
                "team_a_score": 8,
                "team_b_score": 5,
                "round": 3,
                "total_rounds": 5,
                "round_timer": "02:00",
                "economy": self.money
            },
            "bottom_left": {
                "health": {
                    "current": self.health,
                    "max": self.max_health,
                    "bar_color": self.get_health_bar_color(),
                    "bar_length": 150.0
                },
                "armor": {
                    "current": self.armor,
                    "max": self.max_armor,
                    "bar_color": "#4da6ff",
                    "bar_length": 100.0
                },
                "damage_indicator": {
                    "show": false,
                    "direction": "top",
                    "intensity": 0.0
                }
            },
            "bottom_center": {
                "weapon": {
                    "name": self.weapon_name,
                    "magazine_ammo": self.magazine_ammo,
                    "reserve_ammo": self.reserve_ammo,
                    "icon": "🔫"
                },
                "reload_bar": {
                    "show": false,
                    "progress": 0.0
                }
            },
            "bottom_right": {
                "minimap": {
                    "width": 200.0,
                    "height": 200.0,
                    "player_pos": {"x": 100.0, "y": 100.0},
                    "rotation": 0.0,
                    "zoom": 1.0
                },
                "team_info": {
                    "alive_teammates": 4,
                    "total_teammates": 5
                }
            },
            "center": {
                "crosshair": {
                    "style": "Modern",
                    "color": "#00FF00",
                    "scale": 1.0,
                    "spread": 0.0
                }
            },
            "scoreboard": {
                "show": false
            }
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Scoreboard {
    pub team_a_players: Vec<ScoreboardEntry>,
    pub team_b_players: Vec<ScoreboardEntry>,
    pub team_a_score: i32,
    pub team_b_score: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScoreboardEntry {
    pub username: String,
    pub rank: String,
    pub ping: u32,
    pub kills: i32,
    pub deaths: i32,
    pub assists: i32,
    pub is_self: bool,
    pub alive: bool,
}

impl Scoreboard {
    pub fn new() -> Self {
        Scoreboard {
            team_a_players: Vec::new(),
            team_b_players: Vec::new(),
            team_a_score: 0,
            team_b_score: 0,
        }
    }
    
    pub fn render_json(&self) -> serde_json::Value {
        let render_team = |players: &[ScoreboardEntry], color: &str| -> Vec<serde_json::Value> {
            players
                .iter()
                .map(|p| {
                    let ping_color = if p.ping < 50 {
                        "#00CC66"
                    } else if p.ping < 100 {
                        "#FFB627"
                    } else {
                        "#FF3333"
                    };
                    
                    serde_json::json!({
                        "username": p.username,
                        "rank": p.rank,
                        "ping": p.ping,
                        "ping_color": ping_color,
                        "kills": p.kills,
                        "deaths": p.deaths,
                        "assists": p.assists,
                        "is_self": p.is_self,
                        "self_color": if p.is_self { "#FFD700" } else { color },
                        "alive": p.alive,
                        "alive_indicator": if p.alive { "●" } else { "✝" }
                    })
                })
                .collect()
        };
        
        serde_json::json!({
            "scoreboard_state": "Visible",
            "team_a": {
                "name": "TEAM A",
                "color": "#4da6ff",
                "score": self.team_a_score,
                "players": render_team(&self.team_a_players, "#4da6ff")
            },
            "team_b": {
                "name": "TEAM B",
                "color": "#ff6b6b",
                "score": self.team_b_score,
                "players": render_team(&self.team_b_players, "#ff6b6b")
            }
        })
    }
}
