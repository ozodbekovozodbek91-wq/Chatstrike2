use crate::weapon::weapon_types::*;
use std::collections::HashMap;

pub struct WeaponManager {
    pub equipped_weapons: HashMap<usize, Weapon>,  // slot -> weapon
    pub current_weapon_slot: usize,
    pub max_weapon_slots: usize,
}

impl WeaponManager {
    pub fn new() -> Self {
        WeaponManager {
            equipped_weapons: HashMap::new(),
            current_weapon_slot: 0,
            max_weapon_slots: 2,  // Primary + Secondary
        }
    }
    
    pub fn equip_weapon(&mut self, slot: usize, weapon: Weapon) -> bool {
        if slot >= self.max_weapon_slots {
            log::error!("❌ Slot {} out of bounds", slot);
            return false;
        }
        
        self.equipped_weapons.insert(slot, weapon.clone());
        log::info!("✅ Weapon {} equipped in slot {}", weapon.name, slot);
        true
    }
    
    pub fn switch_weapon(&mut self, slot: usize) -> bool {
        if slot >= self.max_weapon_slots || !self.equipped_weapons.contains_key(&slot) {
            log::error!("❌ Cannot switch to slot {}", slot);
            return false;
        }
        
        if self.current_weapon_slot != slot {
            if let Some(prev_weapon) = self.equipped_weapons.get(&self.current_weapon_slot) {
                log::info!("🔄 Switching from {} to slot {}", prev_weapon.name, slot);
            }
            self.current_weapon_slot = slot;
            return true;
        }
        false
    }
    
    pub fn get_current_weapon(&self) -> Option<&Weapon> {
        self.equipped_weapons.get(&self.current_weapon_slot)
    }
    
    pub fn get_current_weapon_mut(&mut self) -> Option<&mut Weapon> {
        self.equipped_weapons.get_mut(&self.current_weapon_slot)
    }
    
    pub fn fire(&mut self, delta_time: f32) -> bool {
        if let Some(weapon) = self.get_current_weapon_mut() {
            weapon.fire(delta_time)
        } else {
            false
        }
    }
    
    pub fn reload(&mut self) {
        if let Some(weapon) = self.get_current_weapon_mut() {
            if !weapon.is_reloading && weapon.magazine_ammo < weapon.current_stats.magazine_capacity {
                weapon.is_reloading = true;
                log::info!("🔄 Reloading {}", weapon.name);
            }
        }
    }
    
    pub fn update(&mut self, delta_time: f32) {
        for weapon in self.equipped_weapons.values_mut() {
            weapon.update(delta_time);
        }
    }
    
    pub fn get_all_weapons_json(&self) -> serde_json::Value {
        let weapons_json: Vec<serde_json::Value> = self.equipped_weapons
            .iter()
            .map(|(slot, weapon)| {
                let mut w = weapon.render_json();
                if let Some(obj) = w.as_object_mut() {
                    obj.insert("slot".to_string(), serde_json::json!(slot));
                    obj.insert("equipped".to_string(), serde_json::json!(slot == &self.current_weapon_slot));
                }
                w
            })
            .collect();
        
        serde_json::json!({
            "weapons": weapons_json,
            "current_slot": self.current_weapon_slot,
            "current_weapon": self.get_current_weapon().map(|w| w.render_json()),
            "total_weapons": self.equipped_weapons.len()
        })
    }
}
