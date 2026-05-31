use serde::{Deserialize, Serialize};
use crate::weapon::weapon_types::Weapon;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct SprayPoint {
    pub shot_number: i32,
    pub x_offset: f32,
    pub y_offset: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SprayPattern {
    pub weapon_name: String,
    pub points: Vec<SprayPoint>,
    pub difficulty: f32,  // 0.0 - 1.0 (1.0 = очень сложно управлять)
}

impl SprayPattern {
    pub fn new(weapon_name: String, difficulty: f32) -> Self {
        SprayPattern {
            weapon_name,
            points: Vec::new(),
            difficulty,
        }
    }
    
    pub fn add_point(&mut self, shot: i32, x: f32, y: f32) {
        self.points.push(SprayPoint {
            shot_number: shot,
            x_offset: x,
            y_offset: y,
        });
    }
    
    pub fn get_spray_offset(&self, shot_number: i32) -> (f32, f32) {
        if let Some(point) = self.points.iter().find(|p| p.shot_number == shot_number as i32) {
            (point.x_offset, point.y_offset)
        } else if let Some(last_point) = self.points.last() {
            (last_point.x_offset, last_point.y_offset)
        } else {
            (0.0, 0.0)
        }
    }
    
    pub fn render_json(&self) -> serde_json::Value {
        let points_json: Vec<serde_json::Value> = self.points
            .iter()
            .map(|p| {
                serde_json::json!({
                    "shot": p.shot_number,
                    "x": format!("{:.2}", p.x_offset),
                    "y": format!("{:.2}", p.y_offset)
                })
            })
            .collect();
        
        serde_json::json!({
            "weapon": self.weapon_name,
            "difficulty": format!("{:.0}%", self.difficulty * 100.0),
            "total_shots": self.points.len(),
            "spray_pattern": points_json
        })
    }
}

pub struct SprayPatternLibrary;

impl SprayPatternLibrary {
    /// M4A1 спрей паттерн
    pub fn create_m4a1_spray() -> SprayPattern {
        let mut spray = SprayPattern::new("M4A1".to_string(), 0.65);
        
        // Первые 7 выстрелов идут вверх
        spray.add_point(1, 0.0, 0.0);
        spray.add_point(2, 0.1, -0.4);
        spray.add_point(3, -0.2, -0.8);
        spray.add_point(4, 0.2, -1.2);
        spray.add_point(5, -0.1, -1.5);
        spray.add_point(6, 0.3, -1.7);
        spray.add_point(7, -0.2, -1.8);
        
        // Потом вниз и вправо
        spray.add_point(8, 0.5, -1.5);
        spray.add_point(9, 0.8, -1.0);
        spray.add_point(10, 1.0, -0.5);
        spray.add_point(11, 1.2, 0.0);
        spray.add_point(12, 1.3, 0.5);
        spray.add_point(13, 1.2, 1.0);
        spray.add_point(14, 1.0, 1.4);
        spray.add_point(15, 0.8, 1.7);
        
        spray
    }
    
    /// AK-47 спрей паттерн (сложнее чем M4A1)
    pub fn create_ak47_spray() -> SprayPattern {
        let mut spray = SprayPattern::new("AK-47".to_string(), 0.8);
        
        // Большой первый recoil
        spray.add_point(1, 0.0, 0.0);
        spray.add_point(2, -0.3, -0.6);
        spray.add_point(3, 0.1, -1.2);
        spray.add_point(4, -0.4, -1.8);
        spray.add_point(5, 0.2, -2.2);
        spray.add_point(6, -0.3, -2.5);
        spray.add_point(7, 0.4, -2.6);
        spray.add_point(8, -0.5, -2.5);
        
        // Боковой дрейф
        spray.add_point(9, -0.8, -2.2);
        spray.add_point(10, -1.2, -1.8);
        spray.add_point(11, -1.5, -1.2);
        spray.add_point(12, -1.3, -0.5);
        spray.add_point(13, -0.8, 0.2);
        spray.add_point(14, 0.0, 0.8);
        spray.add_point(15, 0.8, 1.2);
        
        spray
    }
    
    /// AWP спрей паттерн (очень точный, большой recoil)
    pub fn create_awp_spray() -> SprayPattern {
        let mut spray = SprayPattern::new("AWP".to_string(), 0.3);
        
        // AWP имеет только несколько выстрелов в очереди
        spray.add_point(1, 0.0, 0.0);
        spray.add_point(2, 0.0, -2.5);    // Огромный vertical recoil
        spray.add_point(3, 0.1, -4.8);
        
        spray
    }
    
    /// MP9 спрей паттерн (очень быстро разлетается)
    pub fn create_mp9_spray() -> SprayPattern {
        let mut spray = SprayPattern::new("MP9".to_string(), 0.75);
        
        // Быстрый разброс вправо-вверх
        spray.add_point(1, 0.0, 0.0);
        spray.add_point(2, 0.3, -0.2);
        spray.add_point(3, 0.6, -0.3);
        spray.add_point(4, 1.0, -0.2);
        spray.add_point(5, 1.4, 0.0);
        spray.add_point(6, 1.7, 0.3);
        spray.add_point(7, 1.9, 0.8);
        spray.add_point(8, 2.0, 1.3);
        spray.add_point(9, 1.9, 1.8);
        spray.add_point(10, 1.7, 2.2);
        
        spray
    }
    
    pub fn get_spray_pattern(weapon_name: &str) -> Option<SprayPattern> {
        match weapon_name.to_lowercase().as_str() {
            "m4a1" => Some(Self::create_m4a1_spray()),
            "ak-47" | "ak47" => Some(Self::create_ak47_spray()),
            "awp" => Some(Self::create_awp_spray()),
            "mp9" => Some(Self::create_mp9_spray()),
            _ => None,
        }
    }
}
