use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use chrono::Utc;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub username: String,
    pub password_hash: String,
    pub email: String,
    pub level: u32,
    pub rank: String,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerStats {
    pub username: String,
    pub kills: i32,
    pub deaths: i32,
    pub assists: i32,
    pub score: i32,
    pub matches_played: i32,
    pub wins: i32,
}

pub struct Database {
    users: tokio::sync::RwLock<HashMap<String, User>>,
    stats: tokio::sync::RwLock<HashMap<String, PlayerStats>>,
    db_file: String,
}

impl Database {
    pub async fn new() -> Self {
        let mut db = Database {
            users: tokio::sync::RwLock::new(HashMap::new()),
            stats: tokio::sync::RwLock::new(HashMap::new()),
            db_file: "database.json".to_string(),
        };
        
        db.load().await;
        log::info!("💾 Database initialized");
        
        db
    }
    
    async fn load(&self) {
        if let Ok(content) = fs::read_to_string(&self.db_file) {
            if let Ok(data) = serde_json::from_str::<serde_json::Value>(&content) {
                if let Some(users_obj) = data.get("users").and_then(|v| v.as_object()) {
                    let mut users = self.users.write().await;
                    for (key, value) in users_obj {
                        if let Ok(user) = serde_json::from_value::<User>(value.clone()) {
                            users.insert(key.clone(), user);
                        }
                    }
                }
                
                if let Some(stats_obj) = data.get("stats").and_then(|v| v.as_object()) {
                    let mut stats = self.stats.write().await;
                    for (key, value) in stats_obj {
                        if let Ok(stat) = serde_json::from_value::<PlayerStats>(value.clone()) {
                            stats.insert(key.clone(), stat);
                        }
                    }
                }
                
                log::info!("📂 Database loaded from {}", self.db_file);
            }
        } else {
            log::info!("📝 Creating new database file");
            let _ = self.save().await;
        }
    }
    
    async fn save(&self) -> Result<(), String> {
        let users = self.users.read().await;
        let stats = self.stats.read().await;
        
        let data = serde_json::json!({
            "users": *users,
            "stats": *stats,
            "timestamp": Utc::now(),
        });
        
        match fs::write(&self.db_file, data.to_string()) {
            Ok(_) => {
                log::debug!("💾 Database saved");
                Ok(())
            }
            Err(e) => {
                log::error!("❌ Failed to save database: {}", e);
                Err(format!("Failed to save database: {}", e))
            }
        }
    }
    
    pub async fn authenticate_user(
        &self,
        username: &str,
        password: &str,
    ) -> Result<User, String> {
        let users = self.users.read().await;
        
        if let Some(user) = users.get(username) {
            if self.verify_password(password, &user.password_hash) {
                return Ok(user.clone());
            }
        }
        
        Err("Invalid credentials".to_string())
    }
    
    pub async fn create_user(
        &self,
        username: &str,
        password: &str,
        email: &str,
    ) -> Result<(), String> {
        let mut users = self.users.write().await;
        
        if users.contains_key(username) {
            return Err("User already exists".to_string());
        }
        
        let user = User {
            username: username.to_string(),
            password_hash: self.hash_password(password),
            email: email.to_string(),
            level: 1,
            rank: "Unranked".to_string(),
            created_at: Utc::now().to_rfc3339(),
        };
        
        users.insert(username.to_string(), user.clone());
        
        let mut stats = self.stats.write().await;
        stats.insert(
            username.to_string(),
            PlayerStats {
                username: username.to_string(),
                kills: 0,
                deaths: 0,
                assists: 0,
                score: 0,
                matches_played: 0,
                wins: 0,
            },
        );
        
        drop(users);
        drop(stats);
        
        let _ = self.save().await;
        log::info!("✅ User {} created", username);
        
        Ok(())
    }
    
    pub async fn get_user(&self, username: &str) -> Option<User> {
        self.users.read().await.get(username).cloned()
    }
    
    pub async fn get_stats(&self, username: &str) -> Option<PlayerStats> {
        self.stats.read().await.get(username).cloned()
    }
    
    pub async fn update_stats(&self, username: &str, stats: PlayerStats) -> Result<(), String> {
        let mut stats_map = self.stats.write().await;
        stats_map.insert(username.to_string(), stats);
        drop(stats_map);
        
        self.save().await
    }
    
    pub async fn get_leaderboard(&self, limit: usize) -> Vec<PlayerStats> {
        let mut stats: Vec<PlayerStats> = self.stats.read().await.values().cloned().collect();
        stats.sort_by(|a, b| b.score.cmp(&a.score));
        stats.truncate(limit);
        stats
    }
    
    fn hash_password(&self, password: &str) -> String {
        // В продакшене используй bcrypt или аналог!
        format!("hash_{}", password)
    }
    
    fn verify_password(&self, password: &str, hash: &str) -> bool {
        // В продакшене используй bcrypt или аналог!
        format!("hash_{}", password) == hash
    }
}
