use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkMessage {
    pub message_type: String,
    pub payload: serde_json::Value,
    pub timestamp: DateTime<Utc>,
}

impl NetworkMessage {
    pub fn new(message_type: String, payload: serde_json::Value) -> Self {
        NetworkMessage {
            message_type,
            payload,
            timestamp: Utc::now(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct PacketMetrics {
    pub sent: u64,
    pub received: u64,
    pub lost: u64,
    pub ping: u32,
}

impl Default for PacketMetrics {
    fn default() -> Self {
        PacketMetrics {
            sent: 0,
            received: 0,
            lost: 0,
            ping: 0,
        }
    }
}
