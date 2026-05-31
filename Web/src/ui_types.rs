use serde::{Deserialize, Serialize};
use chrono::Utc;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Color { r, g, b, a }
    }
    
    // CS2 Color Palette
    pub fn primary() -> Self { Color::new(0.0, 0.6, 1.0, 1.0) }      // #0099FF
    pub fn secondary() -> Self { Color::new(1.0, 0.42, 0.2, 1.0) }   // #FF6B35
    pub fn accent() -> Self { Color::new(1.0, 0.71, 0.15, 1.0) }    // #FFB627
    pub fn dark() -> Self { Color::new(0.04, 0.06, 0.15, 1.0) }     // #0A0E27
    pub fn light() -> Self { Color::new(0.91, 0.91, 0.91, 1.0) }    // #E8E8E8
    pub fn success() -> Self { Color::new(0.0, 0.8, 0.4, 1.0) }     // #00CC66
    pub fn danger() -> Self { Color::new(1.0, 0.2, 0.2, 1.0) }      // #FF3333
    pub fn warning() -> Self { Color::new(1.0, 0.6, 0.0, 1.0) }     // #FF9900
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

impl Vector2 {
    pub fn new(x: f32, y: f32) -> Self {
        Vector2 { x, y }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Rect {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Rect { x, y, width, height }
    }
    
    pub fn contains(&self, point: &Vector2) -> bool {
        point.x >= self.x && point.x <= self.x + self.width &&
        point.y >= self.y && point.y <= self.y + self.height
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum UIState {
    Normal,
    Hover,
    Clicked,
    Disabled,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum MenuState {
    MainMenu,
    Settings,
    Lobby,
    LeaderboardsView,
    AccountProfile,
    Loading,
    InGame,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum SettingsTab {
    Graphics,
    Audio,
    Gameplay,
    Controls,
}

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
        }
    }
    
    pub fn is_hovered(&self) -> bool {
        self.state == UIState::Hover
    }
    
    pub fn is_clicked(&self) -> bool {
        self.state == UIState::Clicked
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UISlider {
    pub id: String,
    pub label: String,
    pub rect: Rect,
    pub value: f32,
    pub min: f32,
    pub max: f32,
    pub step: f32,
    pub color: Color,
    pub is_dragging: bool,
}

impl UISlider {
    pub fn new(id: String, label: String, x: f32, y: f32, width: f32, min: f32, max: f32, initial: f32) -> Self {
        UISlider {
            id,
            label,
            rect: Rect::new(x, y, width, 20.0),
            value: initial.clamp(min, max),
            min,
            max,
            step: 1.0,
            color: Color::primary(),
            is_dragging: false,
        }
    }
    
    pub fn get_percentage(&self) -> f32 {
        (self.value - self.min) / (self.max - self.min)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UIPanel {
    pub id: String,
    pub rect: Rect,
    pub background_color: Color,
    pub border_color: Color,
    pub border_width: f32,
    pub border_radius: f32,
    pub opacity: f32,
    pub blur_amount: f32,
}

impl UIPanel {
    pub fn new(id: String, x: f32, y: f32, width: f32, height: f32) -> Self {
        UIPanel {
            id,
            rect: Rect::new(x, y, width, height),
            background_color: Color::dark(),
            border_color: Color::primary(),
            border_width: 2.0,
            border_radius: 12.0,
            opacity: 0.95,
            blur_amount: 10.0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UIText {
    pub text: String,
    pub x: f32,
    pub y: f32,
    pub size: f32,
    pub color: Color,
    pub font: String,
    pub alignment: String, // "left", "center", "right"
}

impl UIText {
    pub fn new(text: String, x: f32, y: f32, size: f32) -> Self {
        UIText {
            text,
            x,
            y,
            size,
            color: Color::light(),
            font: "Arial".to_string(),
            alignment: "left".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphicsSettings {
    pub resolution: String,           // "1920x1080", "2560x1440", etc
    pub quality: String,              // "Low", "Medium", "High", "Ultra"
    pub fps_cap: i32,                 // 60, 120, 144, 240, 0 = unlimited
    pub brightness: f32,              // 0.0 - 100.0
    pub contrast: f32,                // 0.0 - 100.0
    pub gamma: f32,                   // 0.0 - 100.0
    pub saturation: f32,              // 0.0 - 100.0
    pub ray_tracing: bool,
    pub dlss_enabled: bool,
}

impl Default for GraphicsSettings {
    fn default() -> Self {
        GraphicsSettings {
            resolution: "1920x1080".to_string(),
            quality: "Ultra".to_string(),
            fps_cap: 240,
            brightness: 80.0,
            contrast: 50.0,
            gamma: 60.0,
            saturation: 100.0,
            ray_tracing: true,
            dlss_enabled: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioSettings {
    pub master_volume: f32,           // 0.0 - 100.0
    pub music_volume: f32,            // 0.0 - 100.0
    pub sfx_volume: f32,              // 0.0 - 100.0
    pub voice_volume: f32,            // 0.0 - 100.0
    pub audio_output: String,         // "Headphones", "Speakers", etc
    pub microphone: String,           // "Default", etc
    pub voice_chat_enabled: bool,
}

impl Default for AudioSettings {
    fn default() -> Self {
        AudioSettings {
            master_volume: 75.0,
            music_volume: 60.0,
            sfx_volume: 80.0,
            voice_volume: 70.0,
            audio_output: "Default Headphones".to_string(),
            microphone: "Default Microphone".to_string(),
            voice_chat_enabled: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameplaySettings {
    pub mouse_sensitivity: f32,       // 0.1 - 5.0
    pub zoom_sensitivity: f32,        // 0.1 - 2.0
    pub crosshair_style: String,      // "Modern", "Classic", "Minimal", "Dot"
    pub crosshair_color: String,      // Hex color
    pub crosshair_scale: f32,         // 0.5 - 2.0
    pub mouse_acceleration: bool,
    pub invert_y: bool,
    pub raw_input: bool,
    pub hud_scale: f32,               // 0.8 - 1.5
}

impl Default for GameplaySettings {
    fn default() -> Self {
        GameplaySettings {
            mouse_sensitivity: 1.5,
            zoom_sensitivity: 0.8,
            crosshair_style: "Modern".to_string(),
            crosshair_color: "#00FF00".to_string(),
            crosshair_scale: 1.0,
            mouse_acceleration: false,
            invert_y: false,
            raw_input: true,
            hud_scale: 1.0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ControlsSettings {
    pub forward: String,              // "W"
    pub backward: String,             // "S"
    pub left: String,                 // "A"
    pub right: String,                // "D"
    pub fire_primary: String,         // "MOUSE1"
    pub fire_secondary: String,       // "MOUSE2"
    pub reload: String,               // "R"
    pub switch_weapon: String,        // "E"
    pub knife: String,                // "F"
    pub crouch: String,               // "CTRL"
    pub jump: String,                 // "SPACE"
    pub chat: String,                 // "T"
}

impl Default for ControlsSettings {
    fn default() -> Self {
        ControlsSettings {
            forward: "W".to_string(),
            backward: "S".to_string(),
            left: "A".to_string(),
            right: "D".to_string(),
            fire_primary: "MOUSE1".to_string(),
            fire_secondary: "MOUSE2".to_string(),
            reload: "R".to_string(),
            switch_weapon: "E".to_string(),
            knife: "F".to_string(),
            crouch: "CTRL".to_string(),
            jump: "SPACE".to_string(),
            chat: "T".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AllSettings {
    pub graphics: GraphicsSettings,
    pub audio: AudioSettings,
    pub gameplay: GameplaySettings,
    pub controls: ControlsSettings,
}

impl Default for AllSettings {
    fn default() -> Self {
        AllSettings {
            graphics: GraphicsSettings::default(),
            audio: AudioSettings::default(),
            gameplay: GameplaySettings::default(),
            controls: ControlsSettings::default(),
        }
    }
}
