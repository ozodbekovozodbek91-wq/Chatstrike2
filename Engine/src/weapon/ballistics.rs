use serde::{Deserialize, Serialize};
use crate::weapon::weapon_types::Weapon;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vector3 { x, y, z }
    }
    
    pub fn distance_to(&self, other: &Vector3) -> f32 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;
        (dx * dx + dy * dy + dz * dz).sqrt()
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum HitType {
    Head,
    Chest,
    Stomach,
    LeftArm,
    RightArm,
    LeftLeg,
    RightLeg,
}

impl HitType {
    pub fn damage_multiplier(&self) -> f32 {
        match self {
            HitType::Head => 2.5,      // Голова = x2.5 урона
            HitType::Chest => 1.0,     // Грудь = x1.0 урона
            HitType::Stomach => 1.25,  // Живот = x1.25 урона
            HitType::LeftArm => 0.75,  // Рука = x0.75 урона
            HitType::RightArm => 0.75,
            HitType::LeftLeg => 0.75,  // Нога = x0.75 урона
            HitType::RightLeg => 0.75,
        }
    }
    
    pub fn to_string(&self) -> String {
        match self {
            HitType::Head => "HEAD 💀".to_string(),
            HitType::Chest => "CHEST 🎯".to_string(),
            HitType::Stomach => "STOMACH 💢".to_string(),
            HitType::LeftArm => "LEFT ARM".to_string(),
            HitType::RightArm => "RIGHT ARM".to_string(),
            HitType::LeftLeg => "LEFT LEG".to_string(),
            HitType::RightLeg => "RIGHT LEG".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HitInfo {
    pub shooter_id: String,
    pub target_id: String,
    pub weapon_name: String,
    pub hit_type: HitType,
    pub damage: f32,
    pub distance: f32,
    pub armor_penetrated: bool,
    pub critical_hit: bool,
    pub timestamp: f32,
}

impl HitInfo {
    pub fn render_json(&self) -> serde_json::Value {
        serde_json::json!({
            "shooter": self.shooter_id,
            "target": self.target_id,
            "weapon": self.weapon_name,
            "hit_location": self.hit_type.to_string(),
            "damage": format!("{:.1}", self.damage),
            "distance": format!("{}m", self.distance as i32),
            "armor_penetrated": self.armor_penetrated,
            "critical_hit": self.critical_hit,
            "icon": if self.critical_hit { "💥" } else { "💢" }
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BallisticsEngine {
    pub hits: Vec<HitInfo>,
}

impl BallisticsEngine {
    pub fn new() -> Self {
        BallisticsEngine {
            hits: Vec::new(),
        }
    }
    
    pub fn calculate_damage(
        weapon: &Weapon,
        shooter_pos: Vector3,
        target_pos: Vector3,
        hit_type: HitType,
        armor: i32,
    ) -> HitInfo {
        let distance = shooter_pos.distance_to(&target_pos);
        let base_damage = weapon.get_damage_at_distance(distance);
        
        // Применяем множитель за место попадания
        let mut final_damage = base_damage * hit_type.damage_multiplier();
        
        // Броня
        let mut armor_penetrated = false;
        if armor > 0 && weapon.current_stats.armor_penetration < 1.0 {
            let armor_reduction = armor as f32 * (1.0 - weapon.current_stats.armor_penetration);
            final_damage = (final_damage - armor_reduction).max(0.0);
            armor_penetrated = armor_reduction > 0.0;
        } else if armor > 0 {
            armor_penetrated = true;
        }
        
        // Критический урон (только в голову)
        let critical_hit = matches!(hit_type, HitType::Head) && final_damage > 100.0;
        if critical_hit {
            final_damage *= 1.5;  // +50% за крит
        }
        
        HitInfo {
            shooter_id: "Player1".to_string(),
            target_id: "Player2".to_string(),
            weapon_name: weapon.name.clone(),
            hit_type,
            damage: final_damage,
            distance,
            armor_penetrated,
            critical_hit,
            timestamp: 0.0,
        }
    }
    
    pub fn simulate_shot(
        &mut self,
        weapon: &Weapon,
        shooter_pos: Vector3,
        target_pos: Vector3,
        target_armor: i32,
    ) -> HitInfo {
        let hit_type = HitType::Chest; // Для симуляции предположим что в груди
        let hit = Self::calculate_damage(weapon, shooter_pos, target_pos, hit_type, target_armor);
        self.hits.push(hit.clone());
        
        log::info!(
            "🎯 HIT! {} -> {} with {} | Damage: {:.1}",
            hit.shooter_id, hit.target_id, hit.weapon_name, hit.damage
        );
        
        hit
    }
    
    pub fn get_total_damage_dealt(&self, shooter_id: &str, target_id: &str) -> f32 {
        self.hits
            .iter()
            .filter(|h| h.shooter_id == shooter_id && h.target_id == target_id)
            .map(|h| h.damage)
            .sum()
    }
}
