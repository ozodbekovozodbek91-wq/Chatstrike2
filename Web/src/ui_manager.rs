use serde::{Deserialize, Serialize};
use crate::ui_types::*;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UIButton {
    pub id: String,
    pub label: String,
    pub rect: Rect,
    pub state: UIState,
    pub color: Color,
    pub hover_color: Color,
    pub text_color: Color,
    pub border_radius: f32,
    pub glow_intensity: f32,
    pub shadow_blur: f32,
    pub shadow_color: Color,
}

impl UIButton {
    pub fn new(id: String, label: String, x: f32, y: f32, width: f32, height: f32) -> Self {
        UIButton {
            id,
            label,
            rect: Rect::new(x, y, width, height),
            state: UIState::Normal,
            color: Color::primary(),
            hover_color: Color::secondary(),
            text_color: Color::light(),
            border_radius: 8.0,
            glow_intensity: 0.0,
            shadow_blur: 0.0,
            shadow_color: Color::dark(),
        }
    }
    
    pub fn on_hover(&mut self) {
        self.state = UIState::Hover;
        self.glow_intensity = 1.0;
        self.shadow_blur = 15.0;
    }
    
    pub fn on_leave(&mut self) {
        self.state = UIState::Normal;
        self.glow_intensity = 0.0;
        self.shadow_blur = 0.0;
    }
    
    pub fn on_click(&mut self) {
        self.state = UIState::Clicked;
    }
    
    pub fn render_json(&self) -> serde_json::Value {
        serde_json::json!({
            "id": self.id,
            "label": self.label,
            "position": { "x": self.rect.x, "y": self.rect.y },
            "size": { "width": self.rect.width, "height": self.rect.height },
            "state": format!("{:?}", self.state),
            "color": format!("rgba({}, {}, {}, {})", 
                (self.color.r * 255.0) as i32,
                (self.color.g * 255.0) as i32,
                (self.color.b * 255.0) as i32,
                self.color.a),
            "glow": self.glow_intensity,
            "shadow": self.shadow_blur,
            "border_radius": self.border_radius,
        })
    }
}

pub struct UIManager {
    pub buttons: HashMap<String, UIButton>,
    pub sliders: HashMap<String, crate::ui_types::UISlider>,
    pub panels: HashMap<String, UIPanel>,
    pub current_menu: MenuState,
    pub current_settings_tab: SettingsTab,
    pub settings: AllSettings,
}

impl UIManager {
    pub fn new() -> Self {
        log::info!("🎮 UI Manager initialized");
        
        let mut ui = UIManager {
            buttons: HashMap::new(),
            sliders: HashMap::new(),
            panels: HashMap::new(),
            current_menu: MenuState::MainMenu,
            current_settings_tab: SettingsTab::Graphics,
            settings: AllSettings::default(),
        };
        
        ui.create_main_menu_buttons();
        ui
    }
    
    fn create_main_menu_buttons(&mut self) {
        // Center screen buttons
        let button_width = 250.0;
        let button_height = 50.0;
        let start_x = (1920.0 - button_width) / 2.0;
        let start_y = 300.0;
        let spacing = 80.0;
        
        // Play Button
        let mut play_btn = UIButton::new(
            "btn_play".to_string(),
            "▶ PLAY".to_string(),
            start_x,
            start_y,
            button_width,
            button_height,
        );
        play_btn.color = Color::primary();
        self.buttons.insert("btn_play".to_string(), play_btn);
        
        // Tutorial Button
        let mut tutorial_btn = UIButton::new(
            "btn_tutorial".to_string(),
            "🎓 TUTORIAL".to_string(),
            start_x,
            start_y + spacing,
            button_width,
            button_height,
        );
        tutorial_btn.color = Color::accent();
        self.buttons.insert("btn_tutorial".to_string(), tutorial_btn);
        
        // Battle Pass Button
        let mut battlepass_btn = UIButton::new(
            "btn_battlepass".to_string(),
            "🎁 BATTLE PASS".to_string(),
            start_x,
            start_y + spacing * 2.0,
            button_width,
            button_height,
        );
        battlepass_btn.color = Color::secondary();
        self.buttons.insert("btn_battlepass".to_string(), battlepass_btn);
        
        // Leaderboards Button
        let mut leaderboard_btn = UIButton::new(
            "btn_leaderboard".to_string(),
            "🏆 LEADERBOARDS".to_string(),
            start_x,
            start_y + spacing * 3.0,
            button_width,
            button_height,
        );
        leaderboard_btn.color = Color::warning();
        self.buttons.insert("btn_leaderboard".to_string(), leaderboard_btn);
        
        // Account Button
        let mut account_btn = UIButton::new(
            "btn_account".to_string(),
            "👤 ACCOUNT".to_string(),
            start_x,
            start_y + spacing * 4.0,
            button_width,
            button_height,
        );
        account_btn.color = Color::success();
        self.buttons.insert("btn_account".to_string(), account_btn);
        
        // Settings Button (top right)
        let mut settings_btn = UIButton::new(
            "btn_settings".to_string(),
            "⚙️ SETTINGS".to_string(),
            1920.0 - 200.0,
            20.0,
            180.0,
            40.0,
        );
        settings_btn.color = Color::light();
        settings_btn.text_color = Color::dark();
        self.buttons.insert("btn_settings".to_string(), settings_btn);
        
        // Quit Button
        let mut quit_btn = UIButton::new(
            "btn_quit".to_string(),
            "❌ QUIT".to_string(),
            start_x,
            start_y + spacing * 5.0 + 20.0,
            button_width,
            button_height,
        );
        quit_btn.color = Color::danger();
        self.buttons.insert("btn_quit".to_string(), quit_btn);
        
        log::info!("✅ Main menu buttons created");
    }
    
    pub fn handle_mouse_hover(&mut self, x: f32, y: f32) {
        let pos = Vector2::new(x, y);
        
        for button in self.buttons.values_mut() {
            if button.rect.contains(&pos) && button.state != UIState::Disabled {
                if button.state != UIState::Hover {
                    button.on_hover();
                    log::debug!("🗁️ Hovering: {}", button.label);
                }
            } else if button.state == UIState::Hover {
                button.on_leave();
            }
        }
    }
    
    pub fn handle_mouse_click(&mut self, x: f32, y: f32) -> Option<String> {
        let pos = Vector2::new(x, y);
        
        for button in self.buttons.values_mut() {
            if button.rect.contains(&pos) && button.state != UIState::Disabled {
                button.on_click();
                log::info!("🔘 Button clicked: {}", button.id);
                return Some(button.id.clone());
            }
        }
        
        None
    }
    
    pub fn get_all_buttons_json(&self) -> serde_json::Value {
        let buttons: Vec<serde_json::Value> = self.buttons
            .values()
            .map(|btn| btn.render_json())
            .collect();
        
        serde_json::json!({
            "buttons": buttons,
            "total": buttons.len()
        })
    }
    
    pub fn render_main_menu_json(&self) -> serde_json::Value {
        serde_json::json!({
            "menu_state": "MainMenu",
            "background": {
                "type": "gradient",
                "colors": ["#0A0E27", "#1a1f3a"],
                "animation": "particles"
            },
            "logo": {
                "text": "CHATSTRIKE2",
                "size": 72.0,
                "x": 960.0,
                "y": 100.0,
                "glow": true,
                "animation": "float"
            },
            "buttons": self.get_all_buttons_json(),
            "version": "0.1.0"
        })
    }
}
