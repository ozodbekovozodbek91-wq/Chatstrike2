use serde::{Deserialize, Serialize};
use crate::ui_types::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettingsPanel {
    pub current_tab: SettingsTab,
    pub graphics: GraphicsSettings,
    pub audio: AudioSettings,
    pub gameplay: GameplaySettings,
    pub controls: ControlsSettings,
}

impl SettingsPanel {
    pub fn new() -> Self {
        SettingsPanel {
            current_tab: SettingsTab::Graphics,
            graphics: GraphicsSettings::default(),
            audio: AudioSettings::default(),
            gameplay: GameplaySettings::default(),
            controls: ControlsSettings::default(),
        }
    }
    
    pub fn switch_tab(&mut self, tab: SettingsTab) {
        self.current_tab = tab;
        log::info!("📁 Settings tab switched to: {:?}", tab);
    }
    
    pub fn render_graphics_tab_json(&self) -> serde_json::Value {
        serde_json::json!({
            "tab": "Graphics",
            "settings": [
                {
                    "name": "Resolution",
                    "type": "dropdown",
                    "value": self.graphics.resolution,
                    "options": ["1280x720", "1920x1080", "2560x1440", "3840x2160"],
                    "icon": "🖥️"
                },
                {
                    "name": "Quality",
                    "type": "dropdown",
                    "value": self.graphics.quality,
                    "options": ["Low", "Medium", "High", "Ultra"],
                    "icon": "⚡"
                },
                {
                    "name": "FPS Cap",
                    "type": "dropdown",
                    "value": self.graphics.fps_cap,
                    "options": [60, 120, 144, 240, 0],
                    "icon": "📊"
                },
                {
                    "name": "Advanced Options",
                    "type": "collapsible",
                    "items": [
                        {
                            "name": "Ray Tracing",
                            "type": "toggle",
                            "value": self.graphics.ray_tracing
                        },
                        {
                            "name": "DLSS",
                            "type": "toggle",
                            "value": self.graphics.dlss_enabled
                        }
                    ]
                }
            ],
            "sliders": [
                {
                    "name": "Brightness",
                    "value": self.graphics.brightness,
                    "min": 0.0,
                    "max": 100.0,
                    "icon": "☀️"
                },
                {
                    "name": "Contrast",
                    "value": self.graphics.contrast,
                    "min": 0.0,
                    "max": 100.0,
                    "icon": "◐"
                },
                {
                    "name": "Gamma",
                    "value": self.graphics.gamma,
                    "min": 0.0,
                    "max": 100.0,
                    "icon": "🌓"
                },
                {
                    "name": "Saturation",
                    "value": self.graphics.saturation,
                    "min": 0.0,
                    "max": 150.0,
                    "icon": "🎨"
                }
            ]
        })
    }
    
    pub fn render_audio_tab_json(&self) -> serde_json::Value {
        serde_json::json!({
            "tab": "Audio",
            "sliders": [
                {
                    "name": "Master Volume",
                    "value": self.audio.master_volume,
                    "min": 0.0,
                    "max": 100.0,
                    "icon": "🔊"
                },
                {
                    "name": "Music",
                    "value": self.audio.music_volume,
                    "min": 0.0,
                    "max": 100.0,
                    "icon": "🎵"
                },
                {
                    "name": "SFX",
                    "value": self.audio.sfx_volume,
                    "min": 0.0,
                    "max": 100.0,
                    "icon": "🔔"
                },
                {
                    "name": "Voice Chat",
                    "value": self.audio.voice_volume,
                    "min": 0.0,
                    "max": 100.0,
                    "icon": "🎤"
                }
            ],
            "dropdowns": [
                {
                    "name": "Audio Output",
                    "value": self.audio.audio_output,
                    "options": ["Default Headphones", "Headphones", "Speakers", "USB Device"]
                },
                {
                    "name": "Microphone",
                    "value": self.audio.microphone,
                    "options": ["Default Microphone", "USB Microphone", "Headset"]
                }
            ],
            "toggles": [
                {
                    "name": "Voice Chat Enabled",
                    "value": self.audio.voice_chat_enabled
                }
            ]
        })
    }
    
    pub fn render_gameplay_tab_json(&self) -> serde_json::Value {
        serde_json::json!({
            "tab": "Gameplay",
            "sliders": [
                {
                    "name": "Mouse Sensitivity",
                    "value": self.gameplay.mouse_sensitivity,
                    "min": 0.1,
                    "max": 5.0,
                    "step": 0.1,
                    "icon": "🖱️"
                },
                {
                    "name": "Zoom Sensitivity",
                    "value": self.gameplay.zoom_sensitivity,
                    "min": 0.1,
                    "max": 2.0,
                    "step": 0.1,
                    "icon": "🔍"
                },
                {
                    "name": "Crosshair Scale",
                    "value": self.gameplay.crosshair_scale,
                    "min": 0.5,
                    "max": 2.0,
                    "step": 0.1,
                    "icon": "✛"
                },
                {
                    "name": "HUD Scale",
                    "value": self.gameplay.hud_scale,
                    "min": 0.8,
                    "max": 1.5,
                    "step": 0.1,
                    "icon": "📊"
                }
            ],
            "dropdowns": [
                {
                    "name": "Crosshair Style",
                    "value": self.gameplay.crosshair_style,
                    "options": ["Modern", "Classic", "Minimal", "Dot"]
                }
            ],
            "toggles": [
                {
                    "name": "Mouse Acceleration",
                    "value": self.gameplay.mouse_acceleration
                },
                {
                    "name": "Invert Y Axis",
                    "value": self.gameplay.invert_y
                },
                {
                    "name": "Raw Input",
                    "value": self.gameplay.raw_input
                }
            ]
        })
    }
    
    pub fn render_controls_tab_json(&self) -> serde_json::Value {
        serde_json::json!({
            "tab": "Controls",
            "categories": [
                {
                    "name": "MOVEMENT",
                    "icon": "🚶",
                    "bindings": [
                        {"action": "Forward", "key": self.controls.forward},
                        {"action": "Backward", "key": self.controls.backward},
                        {"action": "Left", "key": self.controls.left},
                        {"action": "Right", "key": self.controls.right},
                        {"action": "Jump", "key": self.controls.jump},
                        {"action": "Crouch", "key": self.controls.crouch}
                    ]
                },
                {
                    "name": "COMBAT",
                    "icon": "⚔️",
                    "bindings": [
                        {"action": "Fire Primary", "key": self.controls.fire_primary},
                        {"action": "Fire Secondary", "key": self.controls.fire_secondary},
                        {"action": "Reload", "key": self.controls.reload},
                        {"action": "Switch Weapon", "key": self.controls.switch_weapon},
                        {"action": "Knife", "key": self.controls.knife}
                    ]
                },
                {
                    "name": "COMMUNICATION",
                    "icon": "💬",
                    "bindings": [
                        {"action": "Chat", "key": self.controls.chat}
                    ]
                }
            ]
        })
    }
    
    pub fn render_settings_json(&self) -> serde_json::Value {
        let tab_content = match self.current_tab {
            SettingsTab::Graphics => self.render_graphics_tab_json(),
            SettingsTab::Audio => self.render_audio_tab_json(),
            SettingsTab::Gameplay => self.render_gameplay_tab_json(),
            SettingsTab::Controls => self.render_controls_tab_json(),
        };
        
        serde_json::json!({
            "menu_state": "Settings",
            "tabs": [
                {
                    "name": "Graphics",
                    "icon": "🖥️",
                    "active": self.current_tab == SettingsTab::Graphics
                },
                {
                    "name": "Audio",
                    "icon": "🔊",
                    "active": self.current_tab == SettingsTab::Audio
                },
                {
                    "name": "Gameplay",
                    "icon": "🎮",
                    "active": self.current_tab == SettingsTab::Gameplay
                },
                {
                    "name": "Controls",
                    "icon": "⌨️",
                    "active": self.current_tab == SettingsTab::Controls
                }
            ],
            "content": tab_content,
            "buttons": [
                {"label": "APPLY CHANGES", "style": "primary", "color": "#0099FF"},
                {"label": "CANCEL", "style": "secondary", "color": "#666666"},
                {"label": "DEFAULTS", "style": "tertiary", "color": "#999999"}
            ]
        })
    }
}
