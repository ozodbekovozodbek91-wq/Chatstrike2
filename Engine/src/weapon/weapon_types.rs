use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WeaponType {
    AssaultRifle,    // AR (M4A1, AK-47)
    Pistol,          // 9mm, Deagle
    SMG,             // MP9, UMP45
    Sniper,          // AWP, SSG08
    LMG,             // M249, MAG-7
    Shotgun,         // XM1014, Nova
    Knife,           // Melee
}

impl WeaponType {
    pub fn to_string(&self) -> String {
        match self {
            WeaponType::AssaultRifle => "Assault Rifle".to_string(),
            WeaponType::Pistol => "Pistol".to_string(),
            WeaponType::SMG => "SMG".to_string(),
            WeaponType::Sniper => "Sniper Rifle".to_string(),
            WeaponType::LMG => "Light Machine Gun".to_string(),
            WeaponType::Shotgun => "Shotgun".to_string(),
            WeaponType::Knife => "Knife".to_string(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FireMode {
    SemiAutomatic,   // Один выстрел = один нажим
    FullAutomatic,   // Полуавтомат при удержании кнопки
    BurstFire,       // 3-х выстрелов за один тап
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeaponStats {
    pub damage: f32,              // Урон per bullet
    pub fire_rate: f32,           // Выстрелов в секунду (RPS)
    pub accuracy: f32,            // Точность на дальней дистанции (0.0 - 1.0)
    pub recoil: RecoilPattern,    // Паттерн отдачи
    pub magazine_capacity: i32,   // Кол-во патронов в магазине
    pub reserve_ammo: i32,        // Запасные патроны
    pub reload_time: f32,         // Время перезарядки в секундах
    pub weapon_weight: f32,       // Вес (влияет на скорость движения)
    pub range_effective: f32,     // Эффективная дальность в метрах
    pub armor_penetration: f32,   // Пробиваемость брони (0.0 - 1.0)
    pub spread: f32,              // Начальный разброс
    pub spread_increase: f32,     // Увеличение разброса при стрельбе
    pub spread_recovery: f32,     // Восстановление разброса после стрельбы
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecoilPattern {
    pub horizontal_kicks: Vec<f32>,  // Горизонтальные отскоки
    pub vertical_kicks: Vec<f32>,    // Вертикальные отскоки
    pub recovery_speed: f32,         // Скорость восстановления
    pub total_recoil: f32,           // Общий реколь за очередь
}

impl RecoilPattern {
    pub fn new(horizontal: Vec<f32>, vertical: Vec<f32>, recovery: f32) -> Self {
        let total = horizontal.iter().map(|h| h.abs()).sum::<f32>()
            + vertical.iter().map(|v| v.abs()).sum::<f32>();
        
        RecoilPattern {
            horizontal_kicks: horizontal,
            vertical_kicks: vertical,
            recovery_speed: recovery,
            total_recoil: total,
        }
    }
    
    pub fn get_kick_at_index(&self, index: usize) -> (f32, f32) {
        let h = self.horizontal_kicks.get(index).copied().unwrap_or(0.0);
        let v = self.vertical_kicks.get(index).copied().unwrap_or(0.0);
        (h, v)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AttachmentSlot {
    Scope,        // Оптический прицел
    Silencer,     // Глушитель
    Magazine,     // Расширенный магазин
    Grip,         // Рукоять для уменьшения отдачи
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Attachment {
    pub id: String,
    pub name: String,
    pub slot: AttachmentSlot,
    pub damage_modifier: f32,      // 1.0 = без изменений
    pub fire_rate_modifier: f32,
    pub accuracy_modifier: f32,
    pub recoil_modifier: f32,
    pub weight_modifier: f32,
    pub zoom_level: f32,           // Для оптических прицелов
}

impl Attachment {
    pub fn new_scope(zoom: f32) -> Self {
        Attachment {
            id: format!("scope_{}", zoom),
            name: format!("{}x Scope", zoom),
            slot: AttachmentSlot::Scope,
            damage_modifier: 1.0,
            fire_rate_modifier: 1.0,
            accuracy_modifier: 1.3,  // +30% точность
            recoil_modifier: 1.0,
            weight_modifier: 1.1,     // Немного тяжелее
            zoom_level: zoom,
        }
    }
    
    pub fn new_silencer() -> Self {
        Attachment {
            id: "silencer_9mm".to_string(),
            name: "9mm Silencer".to_string(),
            slot: AttachmentSlot::Silencer,
            damage_modifier: 0.95,    // -5% урона
            fire_rate_modifier: 1.0,
            accuracy_modifier: 0.98,
            recoil_modifier: 0.85,    // -15% отдачи
            weight_modifier: 1.05,
            zoom_level: 1.0,
        }
    }
    
    pub fn new_extended_mag() -> Self {
        Attachment {
            id: "mag_extended".to_string(),
            name: "Extended Magazine".to_string(),
            slot: AttachmentSlot::Magazine,
            damage_modifier: 1.0,
            fire_rate_modifier: 1.0,
            accuracy_modifier: 1.0,
            recoil_modifier: 1.0,
            weight_modifier: 1.15,    // +15% вес
            zoom_level: 1.0,
        }
    }
    
    pub fn new_grip() -> Self {
        Attachment {
            id: "grip_tactical".to_string(),
            name: "Tactical Grip".to_string(),
            slot: AttachmentSlot::Grip,
            damage_modifier: 1.0,
            fire_rate_modifier: 1.0,
            accuracy_modifier: 1.05,  // +5% точность
            recoil_modifier: 0.75,    // -25% отдачи
            weight_modifier: 1.08,
            zoom_level: 1.0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Weapon {
    pub id: String,
    pub name: String,                 // M4A1, AK-47, USP, AWP
    pub weapon_type: WeaponType,
    pub fire_mode: FireMode,
    pub base_stats: WeaponStats,
    pub current_stats: WeaponStats,   // С модификаторами от attachments
    pub magazine_ammo: i32,           // Патроны в магазине
    pub reserve_ammo: i32,            // Запасные патроны
    pub attachments: HashMap<AttachmentSlot, Attachment>,
    pub is_reloading: bool,
    pub reload_progress: f32,         // 0.0 - 1.0
    pub last_shot_time: f32,          // Время последнего выстрела
    pub current_spread: f32,          // Текущий разброс
    pub shots_fired: i32,             // Кол-во выстрелов подряд
}

impl Weapon {
    pub fn new(name: String, weapon_type: WeaponType, stats: WeaponStats) -> Self {
        let mag_ammo = stats.magazine_capacity;
        let reserve = stats.reserve_ammo;
        
        Weapon {
            id: format!("weapon_{}", name.to_lowercase().replace(" ", "_")),
            name,
            weapon_type,
            fire_mode: FireMode::FullAutomatic,
            base_stats: stats.clone(),
            current_stats: stats,
            magazine_ammo: mag_ammo,
            reserve_ammo: reserve,
            attachments: HashMap::new(),
            is_reloading: false,
            reload_progress: 0.0,
            last_shot_time: 0.0,
            current_spread: stats.spread,
            shots_fired: 0,
        }
    }
    
    pub fn can_fire(&self) -> bool {
        self.magazine_ammo > 0 && !self.is_reloading
    }
    
    pub fn fire(&mut self, delta_time: f32) -> bool {
        if !self.can_fire() {
            return false;
        }
        
        // Проверяем fire rate (нужно ли ждать перед следующим выстрелом)
        let time_between_shots = 1.0 / self.current_stats.fire_rate;
        if self.last_shot_time + time_between_shots > delta_time {
            return false;
        }
        
        self.magazine_ammo -= 1;
        self.last_shot_time = delta_time;
        self.shots_fired += 1;
        
        // Увеличиваем разброс
        let spread_increase = self.current_stats.spread_increase * self.shots_fired as f32;
        self.current_spread = (self.current_stats.spread + spread_increase).min(1.0);
        
        log::info!(
            "🔫 {} fired! Ammo: {}/{}, Spread: {:.2}",
            self.name, self.magazine_ammo, self.reserve_ammo, self.current_spread
        );
        
        true
    }
    
    pub fn reload(&mut self, delta_time: f32) {
        if self.is_reloading {
            self.reload_progress += delta_time / self.current_stats.reload_time;
            
            if self.reload_progress >= 1.0 {
                self.reload_progress = 0.0;
                self.is_reloading = false;
                
                // Трансфер патронов
                let ammo_needed = self.current_stats.magazine_capacity - self.magazine_ammo;
                let ammo_taken = ammo_needed.min(self.reserve_ammo);
                
                self.magazine_ammo += ammo_taken;
                self.reserve_ammo -= ammo_taken;
                
                log::info!(
                    "✅ {} reloaded! Ammo: {}/{}",
                    self.name, self.magazine_ammo, self.reserve_ammo
                );
            }
        } else if self.magazine_ammo < self.current_stats.magazine_capacity && self.reserve_ammo > 0 {
            self.is_reloading = true;
            self.reload_progress = 0.0;
            log::info!("🔄 {} reloading...", self.name);
        }
    }
    
    pub fn update(&mut self, delta_time: f32) {
        // Восстанавливаем разброс
        if self.shots_fired > 0 && self.magazine_ammo > 0 {
            self.current_spread = (self.current_spread
                - self.current_stats.spread_recovery * delta_time)
                .max(self.current_stats.spread);
        }
        
        // Обновляем перезарядку
        if self.is_reloading {
            self.reload(delta_time);
        }
    }
    
    pub fn attach(&mut self, attachment: Attachment) {
        let slot = attachment.slot;
        self.attachments.insert(slot, attachment.clone());
        self.recalculate_stats();
        log::info!("📎 Attached {} to {}", attachment.name, self.name);
    }
    
    pub fn detach(&mut self, slot: AttachmentSlot) {
        if self.attachments.remove(&slot).is_some() {
            self.recalculate_stats();
            log::info!("📎 Detached {:?} from {}", slot, self.name);
        }
    }
    
    fn recalculate_stats(&mut self) {
        // Начинаем с базовых статов
        self.current_stats = self.base_stats.clone();
        
        // Применяем модификаторы от всех attachments
        for attachment in self.attachments.values() {
            self.current_stats.damage *= attachment.damage_modifier;
            self.current_stats.fire_rate *= attachment.fire_rate_modifier;
            self.current_stats.accuracy *= attachment.accuracy_modifier;
            self.current_stats.recoil.recovery_speed *= attachment.recoil_modifier;
            self.current_stats.weapon_weight *= attachment.weight_modifier;
        }
    }
    
    pub fn get_damage_at_distance(&self, distance: f32) -> f32 {
        let falloff = 1.0 - (distance / self.current_stats.range_effective).min(1.0);
        self.current_stats.damage * (0.5 + falloff * 0.5) // Минимум 50% урона
    }
    
    pub fn render_json(&self) -> serde_json::Value {
        let attachments_json: Vec<serde_json::Value> = self.attachments
            .values()
            .map(|att| {
                serde_json::json!({
                    "name": att.name,
                    "slot": format!("{:?}", att.slot),
                    "modifiers": {
                        "damage": att.damage_modifier,
                        "fire_rate": att.fire_rate_modifier,
                        "accuracy": att.accuracy_modifier,
                        "recoil": att.recoil_modifier
                    }
                })
            })
            .collect();
        
        serde_json::json!({
            "name": self.name,
            "type": self.weapon_type.to_string(),
            "fire_mode": format!("{:?}", self.fire_mode),
            "ammo": {
                "magazine": self.magazine_ammo,
                "magazine_capacity": self.current_stats.magazine_capacity,
                "reserve": self.reserve_ammo,
                "display": format!("{}/{}", self.magazine_ammo, self.reserve_ammo)
            },
            "stats": {
                "damage": format!("{:.1}", self.current_stats.damage),
                "fire_rate": format!("{:.1} rps", self.current_stats.fire_rate),
                "accuracy": format!("{:.0}%", self.current_stats.accuracy * 100.0),
                "reload_time": format!("{:.2}s", self.current_stats.reload_time),
                "effective_range": format!("{}m", self.current_stats.range_effective as i32),
                "armor_penetration": format!("{:.0}%", self.current_stats.armor_penetration * 100.0)
            },
            "state": {
                "is_reloading": self.is_reloading,
                "reload_progress": format!("{:.0}%", self.reload_progress * 100.0),
                "current_spread": format!("{:.3}", self.current_spread),
                "shots_fired": self.shots_fired
            },
            "attachments": attachments_json,
            "attachments_count": self.attachments.len()
        })
    }
}
