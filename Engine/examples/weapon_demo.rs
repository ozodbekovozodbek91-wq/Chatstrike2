use anyhow::Result;
use serde_json::json;

fn main() -> Result<()> {
    env_logger::init();
    
    println!("\n🔫 =============== CHATSTRIKE2 WEAPON SYSTEM DEMO ===============\n");
    
    // ==================== СОЗДАНИЕ ОРУЖИЯ ====================
    println!("\n📦 Creating weapons...");
    let mut m4a1 = chatstrike2_engine::create_m4a1();
    let ak47 = chatstrike2_engine::create_ak47();
    let deagle = chatstrike2_engine::create_deagle();
    let awp = chatstrike2_engine::create_awp();
    let mp9 = chatstrike2_engine::create_mp9();
    let knife = chatstrike2_engine::create_knife();
    
    println!("✅ Created 6 weapons");
    
    // ==================== M4A1 INFO ====================
    println!("\n🔸 M4A1 Stats:");
    println!("{}", serde_json::to_string_pretty(&m4a1.render_json())?);
    
    // ==================== ATTACHMENTS ====================
    println!("\n\n📎 Adding attachments to M4A1...");
    m4a1.attach(chatstrike2_engine::Attachment::new_scope(4.0));
    m4a1.attach(chatstrike2_engine::Attachment::new_grip());
    
    println!("\n🔸 M4A1 with attachments:");
    println!("{}", serde_json::to_string_pretty(&m4a1.render_json())?);
    
    // ==================== FIRING ====================
    println!("\n\n🔥 Firing M4A1 (first 5 shots)...");
    for i in 0..5 {
        let delta_time = i as f32 * 0.1;
        if m4a1.fire(delta_time) {
            println!("  Shot {}: Ammo: {}, Spread: {:.3}", 
                i + 1, m4a1.magazine_ammo, m4a1.current_spread);
        }
    }
    
    // ==================== RELOAD ====================
    println!("\n\n🔄 Reloading M4A1...");
    m4a1.is_reloading = true;
    m4a1.reload(m4a1.current_stats.reload_time);
    println!("✅ Reload complete! Ammo: {}/{}", m4a1.magazine_ammo, m4a1.reserve_ammo);
    
    // ==================== WEAPON COMPARISON ====================
    println!("\n\n📊 Weapon Comparison:");
    let weapons = vec![
        ("M4A1", m4a1.render_json()),
        ("AK-47", ak47.render_json()),
        ("Deagle", deagle.render_json()),
        ("AWP", awp.render_json()),
        ("MP9", mp9.render_json()),
        ("Knife", knife.render_json()),
    ];
    
    for (name, stats) in weapons {
        if let Some(obj) = stats.as_object() {
            let damage = obj["stats"]["damage"].as_str().unwrap_or("0");
            let fire_rate = obj["stats"]["fire_rate"].as_str().unwrap_or("0");
            let ammo = obj["ammo"]["display"].as_str().unwrap_or("0/0");
            println!("\n  {}: Damage: {:>4} | Fire Rate: {:>8} | Ammo: {:>6}",
                name, damage, fire_rate, ammo);
        }
    }
    
    // ==================== BALLISTICS ====================
    println!("\n\n💥 Ballistics Simulation:");
    let mut ballistics = chatstrike2_engine::BallisticsEngine::new();
    let shooter_pos = chatstrike2_engine::Vector3::new(0.0, 0.0, 0.0);
    let target_pos = chatstrike2_engine::Vector3::new(30.0, 5.0, 0.0);
    
    let hit = ballistics.simulate_shot(&m4a1, shooter_pos, target_pos, 50);
    println!("\n{}", serde_json::to_string_pretty(&hit.render_json())?);
    
    // ==================== SPRAY PATTERNS ====================
    println!("\n\n📈 Spray Patterns:");
    let m4a1_spray = chatstrike2_engine::SprayPatternLibrary::get_spray_pattern("m4a1").unwrap();
    let ak47_spray = chatstrike2_engine::SprayPatternLibrary::get_spray_pattern("ak47").unwrap();
    
    println!("\n🔸 M4A1 Spray Pattern (Difficulty: 65%)");
    println!("  First 5 shots:");
    for i in 1..=5 {
        let (x, y) = m4a1_spray.get_spray_offset(i);
        println!("    Shot {}: X: {:.2}, Y: {:.2}", i, x, y);
    }
    
    println!("\n🔸 AK-47 Spray Pattern (Difficulty: 80%)");
    println!("  First 5 shots:");
    for i in 1..=5 {
        let (x, y) = ak47_spray.get_spray_offset(i);
        println!("    Shot {}: X: {:.2}, Y: {:.2}", i, x, y);
    }
    
    // ==================== WEAPON MANAGER ====================
    println!("\n\n🎮 Weapon Manager:");
    let mut weapon_mgr = chatstrike2_engine::WeaponManager::new();
    weapon_mgr.equip_weapon(0, m4a1);
    weapon_mgr.equip_weapon(1, deagle);
    
    println!("\n{}", serde_json::to_string_pretty(&weapon_mgr.get_all_weapons_json())?);
    
    println!("\n\n✅ Weapon system demo completed!");
    println!("\n🔫 =============== END OF DEMO ===============\n");
    
    Ok(())
}
