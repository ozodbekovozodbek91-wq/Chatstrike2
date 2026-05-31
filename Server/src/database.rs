use sqlx::sqlite::{SqlitePool, SqlitePoolOptions, SqliteConnectOptions};
use sqlx::Row;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use std::str::FromStr;
use bcrypt::{hash, verify, DEFAULT_COST};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub level: i32,
    pub experience: i32,
    pub rank: String,
    pub rank_points: i32,
    pub created_at: String,
    pub updated_at: String,
    pub last_login: Option<String>,
    pub is_banned: bool,
    pub ban_reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct PlayerStats {
    pub id: i64,
    pub user_id: i64,
    pub kills: i32,
    pub deaths: i32,
    pub assists: i32,
    pub score: i32,
    pub matches_played: i32,
    pub matches_won: i32,
    pub accuracy: f32,
    pub headshots: i32,
    pub playtime_seconds: i64,
    pub favorite_weapon: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct MatchRecord {
    pub id: i64,
    pub user_id: i64,
    pub room_id: String,
    pub map_name: String,
    pub kills: i32,
    pub deaths: i32,
    pub assists: i32,
    pub score: i32,
    pub won: bool,
    pub duration_seconds: i64,
    pub started_at: String,
    pub ended_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct RankingSeason {
    pub id: i64,
    pub season_number: i32,
    pub user_id: i64,
    pub username: String,
    pub rating: i32,
    pub wins: i32,
    pub losses: i32,
    pub position: i32,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Achievement {
    pub id: i64,
    pub user_id: i64,
    pub achievement_type: String,
    pub title: String,
    pub description: String,
    pub reward_points: i32,
    pub unlocked_at: String,
}

pub struct DatabasePool {
    pool: SqlitePool,
}

impl DatabasePool {
    pub async fn new(database_url: &str) -> Result<Self, sqlx::Error> {
        let connect_options = SqliteConnectOptions::from_str(database_url)?
            .create_if_missing(true);
        
        let pool = SqlitePoolOptions::new()
            .max_connections(20)
            .connect_with(connect_options)
            .await?;
        
        log::info!("📊 Creating database tables...");
        
        sqlx::query(
            r#"
CREATE TABLE IF NOT EXISTS users (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    username TEXT UNIQUE NOT NULL,
    email TEXT UNIQUE NOT NULL,
    password_hash TEXT NOT NULL,
    level INTEGER NOT NULL DEFAULT 1,
    experience INTEGER NOT NULL DEFAULT 0,
    rank TEXT NOT NULL DEFAULT 'Unranked',
    rank_points INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    last_login TEXT,
    is_banned BOOLEAN NOT NULL DEFAULT 0,
    ban_reason TEXT
);
            "#
        )
        .execute(&pool)
        .await?;
        
        log::info!("✅ users table created");
        
        sqlx::query(
            r#"
CREATE TABLE IF NOT EXISTS player_stats (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER UNIQUE NOT NULL,
    kills INTEGER NOT NULL DEFAULT 0,
    deaths INTEGER NOT NULL DEFAULT 0,
    assists INTEGER NOT NULL DEFAULT 0,
    score INTEGER NOT NULL DEFAULT 0,
    matches_played INTEGER NOT NULL DEFAULT 0,
    matches_won INTEGER NOT NULL DEFAULT 0,
    accuracy REAL NOT NULL DEFAULT 0.0,
    headshots INTEGER NOT NULL DEFAULT 0,
    playtime_seconds INTEGER NOT NULL DEFAULT 0,
    favorite_weapon TEXT NOT NULL DEFAULT 'M4A1',
    updated_at TEXT NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users(id)
);
            "#
        )
        .execute(&pool)
        .await?;
        
        log::info!("✅ player_stats table created");
        
        sqlx::query(
            r#"
CREATE TABLE IF NOT EXISTS match_records (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL,
    room_id TEXT NOT NULL,
    map_name TEXT NOT NULL,
    kills INTEGER NOT NULL,
    deaths INTEGER NOT NULL,
    assists INTEGER NOT NULL,
    score INTEGER NOT NULL,
    won BOOLEAN NOT NULL DEFAULT 0,
    duration_seconds INTEGER NOT NULL,
    started_at TEXT NOT NULL,
    ended_at TEXT NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users(id)
);
            "#
        )
        .execute(&pool)
        .await?;
        
        log::info!("✅ match_records table created");
        
        sqlx::query(
            r#"
CREATE TABLE IF NOT EXISTS ranking_season (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    season_number INTEGER NOT NULL,
    user_id INTEGER NOT NULL,
    username TEXT NOT NULL,
    rating INTEGER NOT NULL,
    wins INTEGER NOT NULL DEFAULT 0,
    losses INTEGER NOT NULL DEFAULT 0,
    position INTEGER NOT NULL,
    updated_at TEXT NOT NULL,
    UNIQUE(season_number, user_id)
);
            "#
        )
        .execute(&pool)
        .await?;
        
        log::info!("✅ ranking_season table created");
        
        sqlx::query(
            r#"
CREATE TABLE IF NOT EXISTS achievements (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL,
    achievement_type TEXT NOT NULL,
    title TEXT NOT NULL,
    description TEXT NOT NULL,
    reward_points INTEGER NOT NULL,
    unlocked_at TEXT NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users(id)
);
            "#
        )
        .execute(&pool)
        .await?;
        
        log::info!("✅ achievements table created");
        
        // Создаём индексы для оптимизации
        sqlx::query("CREATE INDEX IF NOT EXISTS idx_users_username ON users(username)")
            .execute(&pool)
            .await?;
        
        sqlx::query("CREATE INDEX IF NOT EXISTS idx_users_email ON users(email)")
            .execute(&pool)
            .await?;
        
        sqlx::query("CREATE INDEX IF NOT EXISTS idx_match_records_user ON match_records(user_id)")
            .execute(&pool)
            .await?;
        
        sqlx::query("CREATE INDEX IF NOT EXISTS idx_ranking_season_rating ON ranking_season(rating DESC)")
            .execute(&pool)
            .await?;
        
        log::info!("✅ Database indexes created");
        
        Ok(DatabasePool { pool })
    }
    
    // ===== USER MANAGEMENT =====
    
    pub async fn create_user(
        &self,
        username: &str,
        email: &str,
        password: &str,
    ) -> Result<User, String> {
        // Проверяем существование пользователя
        let existing = sqlx::query("SELECT id FROM users WHERE username = ? OR email = ?")
            .bind(username)
            .bind(email)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| format!("Database error: {}", e))?
            .is_some();
        
        if existing {
            return Err("User already exists".to_string());
        }
        
        // Хешируем пароль
        let password_hash = hash(password, DEFAULT_COST)
            .map_err(|e| format!("Password hashing error: {}", e))?
            .to_string();
        
        let now = Utc::now().to_rfc3339();
        
        let result = sqlx::query(
            r#"
INSERT INTO users (username, email, password_hash, created_at, updated_at)
VALUES (?, ?, ?, ?, ?)
            "#
        )
        .bind(username)
        .bind(email)
        .bind(&password_hash)
        .bind(&now)
        .bind(&now)
        .execute(&self.pool)
        .await
        .map_err(|e| format!("Database error: {}", e))?
        .last_insert_rowid();
        
        // Создаём запись в player_stats
        sqlx::query(
            "INSERT INTO player_stats (user_id, updated_at) VALUES (?, ?)"
        )
        .bind(result)
        .bind(&now)
        .execute(&self.pool)
        .await
        .map_err(|e| format!("Failed to create player stats: {}", e))?
        .last_insert_rowid();
        
        log::info!("✅ User {} created successfully", username);
        
        Ok(User {
            id: result,
            username: username.to_string(),
            email: email.to_string(),
            password_hash,
            level: 1,
            experience: 0,
            rank: "Unranked".to_string(),
            rank_points: 0,
            created_at: now.clone(),
            updated_at: now,
            last_login: None,
            is_banned: false,
            ban_reason: None,
        })
    }
    
    pub async fn authenticate_user(
        &self,
        username: &str,
        password: &str,
    ) -> Result<User, String> {
        let user = sqlx::query_as::<_, User>(
            "SELECT * FROM users WHERE username = ?"
        )
        .bind(username)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| format!("Database error: {}", e))?
        .ok_or("User not found".to_string())?;
        
        if user.is_banned {
            return Err(format!("User is banned: {}", user.ban_reason.unwrap_or_default()));
        }
        
        verify(password, &user.password_hash)
            .map_err(|e| format!("Authentication error: {}", e))?
            .then_some(())
            .ok_or("Invalid password".to_string())?;
        
        // Обновляем last_login
        let now = Utc::now().to_rfc3339();
        sqlx::query("UPDATE users SET last_login = ? WHERE id = ?")
            .bind(&now)
            .bind(user.id)
            .execute(&self.pool)
            .await
            .map_err(|e| format!("Failed to update login: {}", e))?
            .last_insert_rowid();
        
        log::info!("✅ {} logged in", username);
        
        Ok(user)
    }
    
    pub async fn get_user_by_id(&self, user_id: i64) -> Result<User, String> {
        sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = ?")
            .bind(user_id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| format!("Database error: {}", e))?
            .ok_or("User not found".to_string())
    }
    
    pub async fn get_user_by_username(&self, username: &str) -> Result<User, String> {
        sqlx::query_as::<_, User>("SELECT * FROM users WHERE username = ?")
            .bind(username)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| format!("Database error: {}", e))?
            .ok_or("User not found".to_string())
    }
    
    pub async fn update_user_level(
        &self,
        user_id: i64,
        exp_gained: i32,
    ) -> Result<(), String> {
        let user = self.get_user_by_id(user_id).await?;
        let new_exp = user.experience + exp_gained;
        let new_level = (new_exp / 1000) + 1; // 1000 exp per level
        
        sqlx::query(
            "UPDATE users SET experience = ?, level = ?, updated_at = ? WHERE id = ?"
        )
        .bind(new_exp)
        .bind(new_level)
        .bind(Utc::now().to_rfc3339())
        .bind(user_id)
        .execute(&self.pool)
        .await
        .map_err(|e| format!("Database error: {}", e))?
        .last_insert_rowid();
        
        log::info!("✅ User {} leveled up to {}", user_id, new_level);
        
        Ok(())
    }
    
    pub async fn ban_user(
        &self,
        user_id: i64,
        reason: &str,
    ) -> Result<(), String> {
        sqlx::query(
            "UPDATE users SET is_banned = 1, ban_reason = ?, updated_at = ? WHERE id = ?"
        )
        .bind(reason)
        .bind(Utc::now().to_rfc3339())
        .bind(user_id)
        .execute(&self.pool)
        .await
        .map_err(|e| format!("Database error: {}", e))?
        .last_insert_rowid();
        
        log::warn!("🚫 User {} banned: {}", user_id, reason);
        
        Ok(())
    }
    
    // ===== PLAYER STATS =====
    
    pub async fn get_player_stats(&self, user_id: i64) -> Result<PlayerStats, String> {
        sqlx::query_as::<_, PlayerStats>(
            "SELECT * FROM player_stats WHERE user_id = ?"
        )
        .bind(user_id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| format!("Database error: {}", e))
    }
    
    pub async fn update_player_stats(
        &self,
        user_id: i64,
        kills: i32,
        deaths: i32,
        assists: i32,
        score: i32,
        accuracy: f32,
        headshots: i32,
    ) -> Result<(), String> {
        sqlx::query(
            r#"
UPDATE player_stats
SET kills = kills + ?,
    deaths = deaths + ?,
    assists = assists + ?,
    score = score + ?,
    accuracy = ?,
    headshots = headshots + ?,
    matches_played = matches_played + 1,
    updated_at = ?
WHERE user_id = ?
            "#
        )
        .bind(kills)
        .bind(deaths)
        .bind(assists)
        .bind(score)
        .bind(accuracy)
        .bind(headshots)
        .bind(Utc::now().to_rfc3339())
        .bind(user_id)
        .execute(&self.pool)
        .await
        .map_err(|e| format!("Database error: {}", e))?
        .last_insert_rowid();
        
        log::info!("✅ Stats updated for user {}", user_id);
        
        Ok(())
    }
    
    // ===== MATCH RECORDS =====
    
    pub async fn record_match(
        &self,
        user_id: i64,
        room_id: &str,
        map_name: &str,
        kills: i32,
        deaths: i32,
        assists: i32,
        score: i32,
        won: bool,
        duration_seconds: i64,
    ) -> Result<MatchRecord, String> {
        let now = Utc::now().to_rfc3339();
        
        let match_id = sqlx::query(
            r#"
INSERT INTO match_records
(user_id, room_id, map_name, kills, deaths, assists, score, won, duration_seconds, started_at, ended_at)
VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#
        )
        .bind(user_id)
        .bind(room_id)
        .bind(map_name)
        .bind(kills)
        .bind(deaths)
        .bind(assists)
        .bind(score)
        .bind(won)
        .bind(duration_seconds)
        .bind(&now)
        .bind(&now)
        .execute(&self.pool)
        .await
        .map_err(|e| format!("Database error: {}", e))?
        .last_insert_rowid();
        
        // Обновляем wins если победа
        if won {
            sqlx::query(
                "UPDATE player_stats SET matches_won = matches_won + 1 WHERE user_id = ?"
            )
            .bind(user_id)
            .execute(&self.pool)
            .await
            .map_err(|e| format!("Database error: {}", e))?
            .last_insert_rowid();
        }
        
        log::info!("📝 Match recorded for user {}: {} points", user_id, score);
        
        Ok(MatchRecord {
            id: match_id,
            user_id,
            room_id: room_id.to_string(),
            map_name: map_name.to_string(),
            kills,
            deaths,
            assists,
            score,
            won,
            duration_seconds,
            started_at: now.clone(),
            ended_at: now,
        })
    }
    
    pub async fn get_match_history(
        &self,
        user_id: i64,
        limit: i64,
    ) -> Result<Vec<MatchRecord>, String> {
        sqlx::query_as::<_, MatchRecord>(
            "SELECT * FROM match_records WHERE user_id = ? ORDER BY ended_at DESC LIMIT ?"
        )
        .bind(user_id)
        .bind(limit)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| format!("Database error: {}", e))
    }
    
    // ===== LEADERBOARD & RANKING =====
    
    pub async fn get_global_leaderboard(&self, limit: i64) -> Result<Vec<PlayerStats>, String> {
        sqlx::query_as::<_, PlayerStats>(
            "SELECT * FROM player_stats ORDER BY score DESC LIMIT ?"
        )
        .bind(limit)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| format!("Database error: {}", e))
    }
    
    pub async fn get_kills_leaderboard(&self, limit: i64) -> Result<Vec<PlayerStats>, String> {
        sqlx::query_as::<_, PlayerStats>(
            "SELECT * FROM player_stats ORDER BY kills DESC LIMIT ?"
        )
        .bind(limit)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| format!("Database error: {}", e))
    }
    
    pub async fn get_accuracy_leaderboard(&self, limit: i64) -> Result<Vec<PlayerStats>, String> {
        sqlx::query_as::<_, PlayerStats>(
            "SELECT * FROM player_stats WHERE matches_played > 10 ORDER BY accuracy DESC LIMIT ?"
        )
        .bind(limit)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| format!("Database error: {}", e))
    }
    
    pub async fn update_rank(
        &self,
        user_id: i64,
        rank_points: i32,
    ) -> Result<(), String> {
        let rank_name = match rank_points {
            0..=500 => "Bronze",
            501..=1000 => "Silver",
            1001..=1500 => "Gold",
            1501..=2000 => "Platinum",
            2001..=2500 => "Diamond",
            2501..=3000 => "Master",
            _ => "Grandmaster",
        };
        
        sqlx::query(
            "UPDATE users SET rank = ?, rank_points = ?, updated_at = ? WHERE id = ?"
        )
        .bind(rank_name)
        .bind(rank_points)
        .bind(Utc::now().to_rfc3339())
        .bind(user_id)
        .execute(&self.pool)
        .await
        .map_err(|e| format!("Database error: {}", e))?
        .last_insert_rowid();
        
        log::info!("🏆 {} promoted to {}", user_id, rank_name);
        
        Ok(())
    }
    
    pub async fn get_seasonal_ranking(
        &self,
        season: i32,
        limit: i64,
    ) -> Result<Vec<RankingSeason>, String> {
        sqlx::query_as::<_, RankingSeason>(
            "SELECT * FROM ranking_season WHERE season_number = ? ORDER BY rating DESC LIMIT ?"
        )
        .bind(season)
        .bind(limit)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| format!("Database error: {}", e))
    }
    
    pub async fn record_seasonal_rating(
        &self,
        season: i32,
        user_id: i64,
        username: &str,
        rating: i32,
    ) -> Result<(), String> {
        let position = sqlx::query("SELECT COUNT(*) as cnt FROM ranking_season WHERE season_number = ? AND rating > ?")
            .bind(season)
            .bind(rating)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| format!("Database error: {}", e))?
            .get::<i64, _>(0) + 1;
        
        sqlx::query(
            r#"
INSERT INTO ranking_season (season_number, user_id, username, rating, position, updated_at)
VALUES (?, ?, ?, ?, ?, ?)
ON CONFLICT(season_number, user_id) DO UPDATE SET
    rating = excluded.rating,
    position = excluded.position,
    updated_at = excluded.updated_at
            "#
        )
        .bind(season)
        .bind(user_id)
        .bind(username)
        .bind(rating)
        .bind(position)
        .bind(Utc::now().to_rfc3339())
        .execute(&self.pool)
        .await
        .map_err(|e| format!("Database error: {}", e))?
        .last_insert_rowid();
        
        Ok(())
    }
    
    // ===== ACHIEVEMENTS =====
    
    pub async fn unlock_achievement(
        &self,
        user_id: i64,
        achievement_type: &str,
        title: &str,
        description: &str,
        reward_points: i32,
    ) -> Result<(), String> {
        // Проверяем, не разблокирована ли уже
        let exists = sqlx::query(
            "SELECT id FROM achievements WHERE user_id = ? AND achievement_type = ?"
        )
        .bind(user_id)
        .bind(achievement_type)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| format!("Database error: {}", e))?
        .is_some();
        
        if exists {
            return Ok(());
        }
        
        sqlx::query(
            r#"
INSERT INTO achievements
(user_id, achievement_type, title, description, reward_points, unlocked_at)
VALUES (?, ?, ?, ?, ?, ?)
            "#
        )
        .bind(user_id)
        .bind(achievement_type)
        .bind(title)
        .bind(description)
        .bind(reward_points)
        .bind(Utc::now().to_rfc3339())
        .execute(&self.pool)
        .await
        .map_err(|e| format!("Database error: {}", e))?
        .last_insert_rowid();
        
        log::info!("🏅 Achievement unlocked for {}: {}", user_id, title);
        
        Ok(())
    }
    
    pub async fn get_achievements(&self, user_id: i64) -> Result<Vec<Achievement>, String> {
        sqlx::query_as::<_, Achievement>(
            "SELECT * FROM achievements WHERE user_id = ? ORDER BY unlocked_at DESC"
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| format!("Database error: {}", e))
    }
    
    // ===== STATISTICS =====
    
    pub async fn get_database_stats(&self) -> Result<serde_json::Value, String> {
        let total_users = sqlx::query("SELECT COUNT(*) as cnt FROM users")
            .fetch_one(&self.pool)
            .await
            .map_err(|e| format!("Database error: {}", e))?
            .get::<i64, _>(0);
        
        let total_matches = sqlx::query("SELECT COUNT(*) as cnt FROM match_records")
            .fetch_one(&self.pool)
            .await
            .map_err(|e| format!("Database error: {}", e))?
            .get::<i64, _>(0);
        
        let total_kills = sqlx::query("SELECT SUM(kills) as total FROM player_stats")
            .fetch_one(&self.pool)
            .await
            .map_err(|e| format!("Database error: {}", e))?
            .get::<Option<i64>, _>(0)
            .unwrap_or(0);
        
        let banned_users = sqlx::query("SELECT COUNT(*) as cnt FROM users WHERE is_banned = 1")
            .fetch_one(&self.pool)
            .await
            .map_err(|e| format!("Database error: {}", e))?
            .get::<i64, _>(0);
        
        Ok(serde_json::json!({
            "total_users": total_users,
            "total_matches": total_matches,
            "total_kills": total_kills,
            "banned_users": banned_users,
        }))
    }
}
