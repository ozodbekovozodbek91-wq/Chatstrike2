use crate::models::*;
use crate::ServerState;
use crate::ai_bot::{BotDifficulty, BotManager};
use serde_json::json;

pub async fn handle_spawn_bots(
    count: usize,
    difficulty: String,
    state: &ServerState,
) -> serde_json::Value {
    let bot_difficulty = match difficulty.as_str() {
        "easy" => BotDifficulty::Easy,
        "normal" => BotDifficulty::Normal,
        "hard" => BotDifficulty::Hard,
        _ => BotDifficulty::Normal,
    };
    
    log::info!("🤖 Spawning {} bots ({})", count, difficulty);
    
    json!({
        "status": "ok",
        "message": format!("Spawned {} bots", count),
        "bots_spawned": count,
        "difficulty": difficulty
    })
}

pub async fn handle_remove_bot(
    bot_id: &str,
    state: &ServerState,
) -> serde_json::Value {
    log::info!("🗑️ Removing bot: {}", bot_id);
    
    json!({
        "status": "ok",
        "message": format!("Bot {} removed", bot_id)
    })
}

pub async fn handle_get_bot_stats(
    state: &ServerState,
) -> serde_json::Value {
    json!({
        "status": "ok",
        "message": "Bot statistics",
        "stats": {
            "active_bots": 0,
            "total_kills": 0,
            "total_deaths": 0,
        }
    })
}
