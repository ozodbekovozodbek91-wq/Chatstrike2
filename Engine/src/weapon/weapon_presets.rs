use crate::weapon::weapon_types::*;

/// M4A1 - Универсальная штурмовая винтовка
pub fn create_m4a1() -> Weapon {
    let stats = WeaponStats {
        damage: 38.0,
        fire_rate: 10.0,              // 10 выстрелов в сек
        accuracy: 0.75,
        recoil: RecoilPattern::new(
            vec![0.2, 0.3, 0.35, 0.4, 0.3, 0.2, 0.1, 0.0],  // horizontal
            vec![0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1.0, 0.5],  // vertical
            0.8,
        ),
        magazine_capacity: 30,
        reserve_ammo: 120,
        reload_time: 2.3,
        weapon_weight: 3.2,
        range_effective: 50.0,
        armor_penetration: 0.75,
        spread: 0.02,
        spread_increase: 0.01,
        spread_recovery: 0.2,
    };
    
    Weapon::new("M4A1".to_string(), WeaponType::AssaultRifle, stats)
}

/// AK-47 - Мощная штурмовая винтовка с высокой отдачей
pub fn create_ak47() -> Weapon {
    let stats = WeaponStats {
        damage: 41.0,                 // Выше чем M4A1
        fire_rate: 9.5,               // Немного ниже
        accuracy: 0.65,               // Ниже, сложнее в управлении
        recoil: RecoilPattern::new(
            vec![0.3, 0.5, 0.7, 0.8, 0.6, 0.4, 0.2, 0.1],  // Выше horizontal
            vec![0.5, 0.7, 0.9, 1.1, 1.0, 0.8, 0.6, 0.4],  // Выше vertical
            0.7,                                            // Медленнее восстанавливается
        ),
        magazine_capacity: 30,
        reserve_ammo: 120,
        reload_time: 2.5,
        weapon_weight: 3.5,
        range_effective: 55.0,
        armor_penetration: 0.8,
        spread: 0.025,
        spread_increase: 0.012,
        spread_recovery: 0.15,
    };
    
    Weapon::new("AK-47".to_string(), WeaponType::AssaultRifle, stats)
}

/// USP-S - Компактный пистолет для начала раунда
pub fn create_usp_s() -> Weapon {
    let stats = WeaponStats {
        damage: 35.0,
        fire_rate: 8.0,               // Медленно
        accuracy: 0.85,               // Очень точный
        recoil: RecoilPattern::new(
            vec![0.1, 0.15, 0.1],
            vec![0.2, 0.3, 0.2],
            1.2,                       // Быстро восстанавливается
        ),
        magazine_capacity: 12,
        reserve_ammo: 24,
        reload_time: 1.5,
        weapon_weight: 0.8,
        range_effective: 20.0,
        armor_penetration: 0.5,
        spread: 0.01,
        spread_increase: 0.005,
        spread_recovery: 0.3,
    };
    
    Weapon::new("USP-S".to_string(), WeaponType::Pistol, stats)
}

/// Deagle - Мощный пистолет с высоким уроном и отдачей
pub fn create_deagle() -> Weapon {
    let mut weapon = Weapon::new(
        "Desert Eagle".to_string(),
        WeaponType::Pistol,
        WeaponStats {
            damage: 63.0,              // Огромный урон
            fire_rate: 1.0,            // Очень медленно (1 выстрел в сек)
            accuracy: 0.8,
            recoil: RecoilPattern::new(
                vec![0.4],              // Огромная отдача
                vec![1.5],
                0.6,
            ),
            magazine_capacity: 7,
            reserve_ammo: 35,
            reload_time: 1.8,
            weapon_weight: 2.5,
            range_effective: 40.0,
            armor_penetration: 0.9,   // Пробивает броню
            spread: 0.03,
            spread_increase: 0.02,
            spread_recovery: 0.4,
        },
    );
    weapon.fire_mode = FireMode::SemiAutomatic;
    weapon
}

/// MP9 - Быстрый SMG для ближнего боя
pub fn create_mp9() -> Weapon {
    let stats = WeaponStats {
        damage: 20.0,                 // Низкий урон
        fire_rate: 18.0,              // ОЧЕНЬ быстрый!
        accuracy: 0.55,               // Низкая точность
        recoil: RecoilPattern::new(
            vec![0.1, 0.2, 0.15, 0.1, 0.05, 0.0],
            vec![0.3, 0.4, 0.5, 0.4, 0.3, 0.2],
            0.9,
        ),
        magazine_capacity: 30,
        reserve_ammo: 120,
        reload_time: 1.7,
        weapon_weight: 2.0,
        range_effective: 15.0,        // Очень близко
        armor_penetration: 0.3,
        spread: 0.05,                 // Высокий начальный разброс
        spread_increase: 0.02,
        spread_recovery: 0.1,
    };
    
    Weapon::new("MP9".to_string(), WeaponType::SMG, stats)
}

/// AWP - Легендарная снайперская винтовка
pub fn create_awp() -> Weapon {
    let mut weapon = Weapon::new(
        "AWP".to_string(),
        WeaponType::Sniper,
        WeaponStats {
            damage: 115.0,             // Один выстрел = смерть
            fire_rate: 0.41,           // ~2.4 выстрела в сек
            accuracy: 0.98,            // Почти идеальная
            recoil: RecoilPattern::new(
                vec![0.5],              // Большая отдача
                vec![2.0],              // Вертикальная отдача
                0.3,
            ),
            magazine_capacity: 10,
            reserve_ammo: 30,
            reload_time: 2.47,
            weapon_weight: 6.5,        // Тяжёлая
            range_effective: 100.0,    // Дальняя дистанция
            armor_penetration: 1.0,    // Пробивает полностью
            spread: 0.0,               // Нет разброса
            spread_increase: 0.0,
            spread_recovery: 0.0,
        },
    );
    weapon.fire_mode = FireMode::SemiAutomatic;
    weapon
}

/// XM1014 - Автоматический дробовик
pub fn create_xm1014() -> Weapon {
    let stats = WeaponStats {
        damage: 25.0,                 // Per pellet (8 pellets per shot = 200 damage close range)
        fire_rate: 3.0,               // Медленный
        accuracy: 0.3,                // Очень низкий
        recoil: RecoilPattern::new(
            vec![0.3, 0.5],
            vec![1.0, 1.5],
            0.7,
        ),
        magazine_capacity: 8,
        reserve_ammo: 32,
        reload_time: 3.3,
        weapon_weight: 4.0,
        range_effective: 8.0,         // Очень близко
        armor_penetration: 0.2,       // Не пробивает броню
        spread: 0.3,                  // ОЧЕНЬ высокий разброс
        spread_increase: 0.1,
        spread_recovery: 0.05,
    };
    
    Weapon::new("XM1014".to_string(), WeaponType::Shotgun, stats)
}

/// M249 - Легкий пулемёт
pub fn create_m249() -> Weapon {
    let stats = WeaponStats {
        damage: 32.0,
        fire_rate: 11.0,              // Быстро
        accuracy: 0.5,                // Низкая
        recoil: RecoilPattern::new(
            vec![0.2, 0.3, 0.4, 0.35, 0.3, 0.25, 0.2],
            vec![0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9],
            0.6,                       // Медленно восстанавливается
        ),
        magazine_capacity: 100,        // ОГРОМНЫЙ магазин!
        reserve_ammo: 200,
        reload_time: 4.7,
        weapon_weight: 6.0,           // Очень тяжёлая
        range_effective: 45.0,
        armor_penetration: 0.7,
        spread: 0.03,
        spread_increase: 0.015,
        spread_recovery: 0.08,
    };
    
    Weapon::new("M249".to_string(), WeaponType::LMG, stats)
}

/// Knife - Ближний боевой нож
pub fn create_knife() -> Weapon {
    let mut weapon = Weapon::new(
        "Knife".to_string(),
        WeaponType::Knife,
        WeaponStats {
            damage: 45.0,              // Урон при ударе
            fire_rate: 1.5,            // Урон в сек (может бить 1.5 раза в сек)
            accuracy: 1.0,             // Всегда попадаешь на близком расстоянии
            recoil: RecoilPattern::new(
                vec![],
                vec![],
                1.0,
            ),
            magazine_capacity: 1,      // Нет магазина
            reserve_ammo: 0,           // Нет патронов
            reload_time: 0.0,
            weapon_weight: 0.5,        // Легче всего
            range_effective: 2.0,      // Нужно быть близко
            armor_penetration: 0.5,
            spread: 0.0,
            spread_increase: 0.0,
            spread_recovery: 0.0,
        },
    );
    weapon.magazine_ammo = 1;
    weapon.fire_mode = FireMode::SemiAutomatic;
    weapon
}

/// Weapon factory function
pub fn create_weapon(weapon_name: &str) -> Option<Weapon> {
    match weapon_name.to_lowercase().as_str() {
        "m4a1" => Some(create_m4a1()),
        "ak-47" | "ak47" => Some(create_ak47()),
        "usp-s" | "usps" => Some(create_usp_s()),
        "deagle" | "desert eagle" => Some(create_deagle()),
        "mp9" => Some(create_mp9()),
        "awp" => Some(create_awp()),
        "xm1014" => Some(create_xm1014()),
        "m249" => Some(create_m249()),
        "knife" => Some(create_knife()),
        _ => None,
    }
}
