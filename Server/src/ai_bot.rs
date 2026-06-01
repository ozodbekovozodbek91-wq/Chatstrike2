use serde::{Deserialize, Serialize};
use chrono::Utc;
use uuid::Uuid;
use crate::models::{Vector3, WeaponType, PlayerData};
use crate::pathfinding::{PathNode, Pathfinder, VisionSystem};
use std::collections::VecDeque;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum BotDifficulty {
    Easy,
    Normal,
    Hard,
}

impl BotDifficulty {
    pub fn reaction_time(&self) -> u64 {
        match self {
            BotDifficulty::Easy => 500,      // 500ms
            BotDifficulty::Normal => 200,    // 200ms
            BotDifficulty::Hard => 50,       // 50ms
        }
    }
    
    pub fn accuracy(&self) -> f32 {
        match self {
            BotDifficulty::Easy => 0.4,      // 40%
            BotDifficulty::Normal => 0.65,   // 65%
            BotDifficulty::Hard => 0.9,      // 90%
        }
    }
    
    pub fn vision_range(&self) -> f32 {
        match self {
            BotDifficulty::Easy => 1000.0,
            BotDifficulty::Normal => 1500.0,
            BotDifficulty::Hard => 2000.0,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum BotState {
    Idle,
    Patrolling,
    ChaseEnemy,
    Combat,
    Dead,
}

#[derive(Debug, Clone)]
pub struct BotBrain {
    pub state: BotState,
    pub last_seen_enemy: Option<Vector3>,
    pub search_timer: f32,
    pub patrol_waypoints: VecDeque<PathNode>,
    pub current_waypoint_idx: usize,
    pub decision_cooldown: f32,
}

impl BotBrain {
    pub fn new() -> Self {
        BotBrain {
            state: BotState::Idle,
            last_seen_enemy: None,
            search_timer: 0.0,
            patrol_waypoints: VecDeque::new(),
            current_waypoint_idx: 0,
            decision_cooldown: 0.0,
        }
    }
}

pub struct Bot {
    pub id: String,
    pub player_data: PlayerData,
    pub difficulty: BotDifficulty,
    pub brain: BotBrain,
    pub vision_system: VisionSystem,
    pub pathfinder: Option<Pathfinder>,
    pub last_decision_time: f32,
    pub shoot_cooldown: f32,
    pub reload_timer: f32,
}

impl Bot {
    pub fn new(username: String, difficulty: BotDifficulty, pathfinder: Option<Pathfinder>) -> Self {
        let id = Uuid::new_v4().to_string();
        let mut player_data = PlayerData::new(id.clone(), username);
        player_data.position = Vector3::new(0.0, 0.0, 0.0);
        
        let vision_range = difficulty.vision_range();
        
        log::info!("🤖 Bot created: {} ({})", player_data.username, id);
        
        Bot {
            id,
            player_data,
            difficulty,
            brain: BotBrain::new(),
            vision_system: VisionSystem::new(vision_range, 120.0),
            pathfinder,
            last_decision_time: 0.0,
            shoot_cooldown: 0.0,
            reload_timer: 0.0,
        }
    }
    
    /// Main update loop для бота
    pub fn update(&mut self, delta_time: f32, enemies: &[PlayerData]) {
        if !self.player_data.alive {
            self.brain.state = BotState::Dead;
            return;
        }
        
        // Обновляем таймеры
        self.shoot_cooldown = (self.shoot_cooldown - delta_time).max(0.0);
        self.reload_timer = (self.reload_timer - delta_time).max(0.0);
        self.brain.search_timer = (self.brain.search_timer - delta_time).max(0.0);
        self.last_decision_time += delta_time;
        
        // Ищем видимых врагов
        let visible_enemy = self.find_visible_enemy(enemies);
        
        if let Some(enemy) = visible_enemy {
            // ✅ ИСПРАВЛЕНИЕ: Проверяем что враг живой перед обновлением
            if enemy.alive {
                self.brain.last_seen_enemy = Some(enemy.position.clone());
                self.brain.search_timer = 5.0; // Ищем 5 секунд
                self.handle_combat_state(&enemy, delta_time);
            }
        } else if self.brain.search_timer > 0.0 {
            // Ищем врага по последней известной позиции
            self.handle_search_state(delta_time);
        } else {
            // Патрулируем
            self.handle_patrol_state(delta_time);
        }
    }
    
    fn find_visible_enemy(&self, enemies: &[PlayerData]) -> Option<PlayerData> {
        for enemy in enemies {
            if !enemy.alive || enemy.id == self.player_data.id {
                continue;
            }
            
            if self.vision_system.can_see_target(
                &self.player_data.position,
                &self.player_data.rotation,
                &enemy.position,
            ) {
                return Some(enemy.clone());
            }
        }
        None
    }
    
    fn handle_combat_state(&mut self, enemy: &PlayerData, delta_time: f32) {
        self.brain.state = BotState::Combat;
        
        // Поворачиваемся к врагу
        let dx = enemy.position.x - self.player_data.position.x;
        let dy = enemy.position.y - self.player_data.position.y;
        let dz = enemy.position.z - self.player_data.position.z;
        
        let distance = (dx.powi(2) + dy.powi(2) + dz.powi(2)).sqrt();
        
        if distance > 0.1 {
            self.player_data.rotation = Vector3 {
                x: dx.atan2(dy),
                y: dz / distance,
                z: 0.0,
            };
        }
        
        // ✅ ИСПРАВЛЕНИЕ: Стреляем с учётом сложности
        if self.shoot_cooldown <= 0.0 && self.reload_timer <= 0.0 {
            let accuracy = self.difficulty.accuracy();
            
            // Добавляем неточность в зависимости от сложности
            if rand::random::<f32>() < accuracy {
                log::info!("💥 Bot {} shooting at enemy (accuracy: {:.0}%)", self.id, accuracy * 100.0);
                self.player_data.kills += 1;
                self.shoot_cooldown = 1.0 / self.player_data.current_weapon.fire_rate();
            }
        }
        
        // ✅ ИСПРАВЛЕНИЕ: Логика перезарядки
        if self.player_data.ammo.current <= 0 && self.reload_timer <= 0.0 {
            log::info!("🔄 Bot {} reloading", self.id);
            self.reload_timer = 2.0; // Время перезарядки
        }
        
        // Перезарядка завершена
        if self.reload_timer <= 0.0 && self.player_data.ammo.current <= 0 {
            self.player_data.ammo.current = self.player_data.ammo.max;
            log::info!("✅ Bot {} reload complete", self.id);
        }
        
        // ✅ ИСПРАВЛЕНИЕ: Движение - обходим врага с разумной скоростью
        const COMBAT_STRAFE_SPEED: f32 = 80.0;
        self.player_data.velocity = Vector3 {
            x: (rand::random::<f32>() - 0.5) * COMBAT_STRAFE_SPEED,
            y: (rand::random::<f32>() - 0.5) * COMBAT_STRAFE_SPEED,
            z: 0.0,
        };
    }
    
    fn handle_search_state(&mut self, delta_time: f32) {
        self.brain.state = BotState::ChaseEnemy;
        
        if let Some(last_pos) = &self.brain.last_seen_enemy.clone() {
            // Двигаемся к последней известной позиции
            let dx = last_pos.x - self.player_data.position.x;
            let dy = last_pos.y - self.player_data.position.y;
            let dist = (dx.powi(2) + dy.powi(2)).sqrt();
            
            const SEARCH_RADIUS: f32 = 10.0;
            const CHASE_SPEED: f32 = 120.0;
            
            if dist > SEARCH_RADIUS {
                self.player_data.velocity = Vector3 {
                    x: (dx / dist) * CHASE_SPEED,
                    y: (dy / dist) * CHASE_SPEED,
                    z: 0.0,
                };
            } else {
                self.player_data.velocity = Vector3::new(0.0, 0.0, 0.0);
            }
        }
    }
    
    fn handle_patrol_state(&mut self, delta_time: f32) {
        self.brain.state = BotState::Patrolling;
        
        // ✅ ИСПРАВЛЕНИЕ: Случайное движение с разумной скоростью
        const PATROL_DECISION_TIME: f32 = 2.0;
        const PATROL_SPEED: f32 = 50.0;
        
        if self.last_decision_time > PATROL_DECISION_TIME {
            self.player_data.velocity = Vector3 {
                x: (rand::random::<f32>() - 0.5) * PATROL_SPEED,
                y: (rand::random::<f32>() - 0.5) * PATROL_SPEED,
                z: 0.0,
            };
            self.last_decision_time = 0.0;
            log::debug!("🚶 Bot {} new patrol direction", self.id);
        }
    }
}

pub struct BotManager {
    pub bots: Vec<Bot>,
    pub max_bots: usize,
}

impl BotManager {
    pub fn new(max_bots: usize) -> Self {
        log::info!("🤖 Bot Manager initialized (max bots: {})", max_bots);
        
        BotManager {
            bots: Vec::new(),
            max_bots,
        }
    }
    
    pub fn spawn_bot(
        &mut self,
        username: String,
        difficulty: BotDifficulty,
        pathfinder: Option<Pathfinder>,
    ) -> Option<Bot> {
        if self.bots.len() >= self.max_bots {
            log::warn!("⚠️ Max bots limit reached");
            return None;
        }
        
        let bot = Bot::new(username, difficulty, pathfinder);
        log::info!("✅ Bot spawned: {} ({:?})", bot.player_data.username, bot.difficulty);
        
        self.bots.push(bot.clone());
        Some(bot)
    }
    
    pub fn remove_bot(&mut self, bot_id: &str) -> bool {
        if let Some(pos) = self.bots.iter().position(|b| b.id == bot_id) {
            self.bots.remove(pos);
            log::info!("🗑️ Bot removed: {}", bot_id);
            true
        } else {
            false
        }
    }
    
    pub fn update_all_bots(&mut self, delta_time: f32, players: &[PlayerData]) {
        for bot in &mut self.bots {
            bot.update(delta_time, players);
        }
    }
    
    pub fn get_bot(&mut self, bot_id: &str) -> Option<&mut Bot> {
        self.bots.iter_mut().find(|b| b.id == bot_id)
    }
    
    pub fn get_bot_stats(&self) -> serde_json::Value {
        let total_bots = self.bots.len();
        let total_kills: i32 = self.bots.iter().map(|b| b.player_data.kills).sum();
        let total_deaths: i32 = self.bots.iter().map(|b| b.player_data.deaths).sum();
        let alive_bots = self.bots.iter().filter(|b| b.player_data.alive).count();
        
        serde_json::json!({
            "total_bots": total_bots,
            "max_bots": self.max_bots,
            "active_bots": alive_bots,
            "total_kills": total_kills,
            "total_deaths": total_deaths,
            "avg_kd": if total_deaths > 0 {
                total_kills as f32 / total_deaths as f32
            } else {
                total_kills as f32
            }
        })
    }
}
