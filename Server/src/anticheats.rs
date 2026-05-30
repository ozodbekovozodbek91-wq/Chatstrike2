use std::sync::atomic::{AtomicU64, Ordering};
use chrono::Utc;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheatEvent {
    pub player_id: String,
    pub event_type: String,
    pub severity: u8,
    pub details: String,
    pub timestamp: String,
}

pub struct AntiCheatSystem {
    events: tokio::sync::RwLock<Vec<CheatEvent>>,
    total_events: AtomicU64,
}

impl AntiCheatSystem {
    pub fn new() -> Self {
        log::info!("🔐 Anti-Cheat System initialized");
        
        AntiCheatSystem {
            events: tokio::sync::RwLock::new(Vec::new()),
            total_events: AtomicU64::new(0),
        }
    }
    
    pub async fn check_position_anomaly(
        &self,
        player_id: &str,
        prev_pos: crate::models::Vector3,
        current_pos: crate::models::Vector3,
        delta_time: f32,
    ) -> bool {
        let distance = prev_pos.distance(&current_pos);
        let max_distance = 100.0 * delta_time; // Max speed
        
        if distance > max_distance {
            self.log_event(
                player_id.to_string(),
                "teleport_detected".to_string(),
                3,
                format!("Teleported {} units in {} seconds", distance, delta_time),
            ).await;
            return false;
        }
        
        true
    }
    
    pub async fn check_aimbot(
        &self,
        player_id: &str,
        accuracy: f32,
    ) -> bool {
        if accuracy > 0.95 {
            self.log_event(
                player_id.to_string(),
                "suspicious_accuracy".to_string(),
                2,
                format!("Accuracy: {}%", accuracy * 100.0),
            ).await;
            return false;
        }
        
        true
    }
    
    pub async fn check_speed_hack(
        &self,
        player_id: &str,
        speed: f32,
    ) -> bool {
        let max_speed = 350.0; // UE units per second
        
        if speed > max_speed {
            self.log_event(
                player_id.to_string(),
                "speed_hack_detected".to_string(),
                3,
                format!("Speed: {} units/s (max: {})", speed, max_speed),
            ).await;
            return false;
        }
        
        true
    }
    
    pub async fn log_event(
        &self,
        player_id: String,
        event_type: String,
        severity: u8,
        details: String,
    ) {
        let event = CheatEvent {
            player_id,
            event_type,
            severity,
            details,
            timestamp: Utc::now().to_rfc3339(),
        };
        
        log::warn!("⚠️ Anti-Cheat Event: {:?}", event);
        
        self.events.write().await.push(event);
        self.total_events.fetch_add(1, Ordering::SeqCst);
    }
    
    pub fn get_total_events(&self) -> u64 {
        self.total_events.load(Ordering::SeqCst)
    }
    
    pub async fn get_events(&self, player_id: &str) -> Vec<CheatEvent> {
        self.events
            .read()
            .await
            .iter()
            .filter(|e| e.player_id == player_id)
            .cloned()
            .collect()
    }
}
