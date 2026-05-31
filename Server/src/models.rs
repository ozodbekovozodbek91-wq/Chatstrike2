use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vector3 { x, y, z }
    }
    
    pub fn distance(&self, other: &Vector3) -> f32 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2) + (self.z - other.z).powi(2)).sqrt()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerData {
    pub id: String,
    pub username: String,
    pub health: i32,
    pub max_health: i32,
    pub alive: bool,
    pub position: Vector3,
    pub rotation: Vector3,
    pub velocity: Vector3,
    pub ammo: WeaponAmmo,
    pub current_weapon: WeaponType,
    pub score: i32,
    pub kills: i32,
    pub deaths: i32,
    pub assists: i32,
    pub last_update: DateTime<Utc>,
    pub connected_at: DateTime<Utc>,
    pub ping: u32,
}

impl PlayerData {
    pub fn new(id: String, username: String) -> Self {
        PlayerData {
            id,
            username,
            health: 100,
            max_health: 100,
            alive: true,
            position: Vector3::new(0.0, 0.0, 0.0),
            rotation: Vector3::new(0.0, 0.0, 0.0),
            velocity: Vector3::new(0.0, 0.0, 0.0),
            ammo: WeaponAmmo::default(),
            current_weapon: WeaponType::M4A1,
            score: 0,
            kills: 0,
            deaths: 0,
            assists: 0,
            last_update: Utc::now(),
            connected_at: Utc::now(),
            ping: 0,
        }
    }
    
    pub fn take_damage(&mut self, damage: i32) {
        self.health = (self.health - damage).max(0);
        if self.health == 0 {
            self.alive = false;
            self.deaths += 1;
        }
    }
    
    pub fn heal(&mut self, amount: i32) {
        self.health = (self.health + amount).min(self.max_health);
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum WeaponType {
    M4A1,
    AK47,
    AWP,
    UMP45,
    M249,
    Knife,
    Pistol,
}

impl WeaponType {
    pub fn damage(&self) -> f32 {
        match self {
            WeaponType::M4A1 => 25.0,
            WeaponType::AK47 => 30.0,
            WeaponType::AWP => 115.0,
            WeaponType::UMP45 => 20.0,
            WeaponType::M249 => 15.0,
            WeaponType::Knife => 60.0,
            WeaponType::Pistol => 10.0,
        }
    }
    
    pub fn fire_rate(&self) -> f32 {
        match self {
            WeaponType::M4A1 => 0.1,
            WeaponType::AK47 => 0.1,
            WeaponType::AWP => 1.47,
            WeaponType::UMP45 => 0.08,
            WeaponType::M249 => 0.04,
            WeaponType::Knife => 0.5,
            WeaponType::Pistol => 0.15,
        }
    }
    
    pub fn magazine_size(&self) -> i32 {
        match self {
            WeaponType::M4A1 => 30,
            WeaponType::AK47 => 30,
            WeaponType::AWP => 10,
            WeaponType::UMP45 => 25,
            WeaponType::M249 => 200,
            WeaponType::Knife => -1,
            WeaponType::Pistol => 12,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeaponAmmo {
    pub m4a1: i32,
    pub ak47: i32,
    pub awp: i32,
    pub ump45: i32,
    pub m249: i32,
    pub pistol: i32,
}

impl Default for WeaponAmmo {
    fn default() -> Self {
        WeaponAmmo {
            m4a1: 120,
            ak47: 120,
            awp: 30,
            ump45: 100,
            m249: 500,
            pistol: 120,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bullet {
    pub id: String,
    pub position: Vector3,
    pub velocity: Vector3,
    pub damage: f32,
    pub owner: String,
    pub lifetime: f32,
}

#[derive(Debug, Clone)]
pub struct GameRoom {
    pub id: String,
    pub name: String,
    pub map_name: String,
    pub max_players: usize,
    pub players: Vec<String>,
    pub bullets: Vec<Bullet>,
    pub state: GameState,
    pub tick_count: u64,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GameState {
    Waiting,
    Starting,
    InProgress,
    Ended,
}

impl GameRoom {
    pub fn new(id: String, name: String, map_name: String, max_players: usize) -> Self {
        GameRoom {
            id,
            name,
            map_name,
            max_players,
            players: Vec::new(),
            bullets: Vec::new(),
            state: GameState::Waiting,
            tick_count: 0,
            created_at: Utc::now(),
        }
    }
    
    pub fn can_join(&self) -> bool {
        self.players.len() < self.max_players && self.state == GameState::Waiting
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ServerCommand {
    Login { username: String, password: String },
    Register { username: String, password: String, email: String },
    CreateRoom { map_name: String, max_players: usize },
    JoinRoom { room_id: String },
    LeaveRoom,
    Move { direction: Vector3 },
    Shoot { target: Vector3, weapon: WeaponType },
    Reload,
    ChangeWeapon { weapon: WeaponType },
    GetPlayers,
    GetRooms,
    GetStats,
    GetLeaderboard { top: i64 },
    GetMatchHistory { limit: i64 },
    GetLeaderboardByType { board_type: String, limit: i64 },
    Ping,
    UpdatePosition { position: Vector3, rotation: Vector3 },
    Chat { message: String },
    EndMatch { kills: i32, deaths: i32, assists: i32, score: i32, won: bool, duration: i64 },
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServerResponse {
    pub status: String,
    pub data: serde_json::Value,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub status: String,
    pub message: String,
    pub code: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkMessage {
    pub message_type: String,
    pub payload: serde_json::Value,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct ClientConnection {
    pub id: String,
    pub addr: std::net::SocketAddr,
    pub connected_at: DateTime<Utc>,
}

impl ClientConnection {
    pub fn new(id: String, addr: std::net::SocketAddr) -> Self {
        ClientConnection {
            id,
            addr,
            connected_at: Utc::now(),
        }
    }
}
