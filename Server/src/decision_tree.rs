use serde::{Deserialize, Serialize};
use chrono::Utc;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BotCommand {
    pub priority: i32, // Чем выше, тем важнее
    pub command_type: String,
    pub duration: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DecisionNodeType {
    Root,
    EnemyNearby,
    CanSeeEnemy,
    LowHealth,
    LowAmmo,
    Action,
}

#[derive(Debug, Clone)]
pub struct DecisionNode {
    pub node_type: DecisionNodeType,
    pub threshold: f32,
    pub true_action: Box<Option<DecisionNode>>,
    pub false_action: Box<Option<DecisionNode>>,
}

impl DecisionNode {
    pub fn new(node_type: DecisionNodeType) -> Self {
        DecisionNode {
            node_type,
            threshold: 0.0,
            true_action: Box::new(None),
            false_action: Box::new(None),
        }
    }
    
    pub fn evaluate(&self, context: &BotContext) -> Option<String> {
        match self.node_type {
            DecisionNodeType::Root => {
                self.true_action.as_ref().and_then(|n| n.evaluate(context))
            }
            DecisionNodeType::EnemyNearby => {
                if context.nearest_enemy_distance < self.threshold {
                    self.true_action.as_ref().and_then(|n| n.evaluate(context))
                } else {
                    self.false_action.as_ref().and_then(|n| n.evaluate(context))
                }
            }
            DecisionNodeType::CanSeeEnemy => {
                if context.can_see_enemy {
                    self.true_action.as_ref().and_then(|n| n.evaluate(context))
                } else {
                    self.false_action.as_ref().and_then(|n| n.evaluate(context))
                }
            }
            DecisionNodeType::LowHealth => {
                if context.health_percent < self.threshold {
                    self.true_action.as_ref().and_then(|n| n.evaluate(context))
                } else {
                    self.false_action.as_ref().and_then(|n| n.evaluate(context))
                }
            }
            DecisionNodeType::LowAmmo => {
                if context.ammo_percent < self.threshold {
                    self.true_action.as_ref().and_then(|n| n.evaluate(context))
                } else {
                    self.false_action.as_ref().and_then(|n| n.evaluate(context))
                }
            }
            DecisionNodeType::Action => {
                Some(format!("action_{}", context.current_time as i32))
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct BotContext {
    pub bot_id: String,
    pub health: i32,
    pub health_percent: f32,
    pub ammo_percent: f32,
    pub nearest_enemy_distance: f32,
    pub can_see_enemy: bool,
    pub current_time: f32,
    pub last_action_time: f32,
}

impl BotContext {
    pub fn new(bot_id: String) -> Self {
        BotContext {
            bot_id,
            health: 100,
            health_percent: 1.0,
            ammo_percent: 1.0,
            nearest_enemy_distance: f32::MAX,
            can_see_enemy: false,
            current_time: 0.0,
            last_action_time: 0.0,
        }
    }
}

pub struct DecisionTree {
    pub root: DecisionNode,
    pub action_map: HashMap<String, Box<dyn Fn(&BotContext) -> String>>,
}

impl DecisionTree {
    pub fn new() -> Self {
        let mut root = DecisionNode::new(DecisionNodeType::Root);
        
        // Строим дерево решений
        let mut enemy_nearby = Box::new(Some(DecisionNode::new(DecisionNodeType::EnemyNearby)));
        if let Some(ref mut node) = *enemy_nearby {
            node.threshold = 500.0; // 500 units
            
            // True: враг рядом
            let mut can_see = Box::new(Some(DecisionNode::new(DecisionNodeType::CanSeeEnemy)));
            if let Some(ref mut see_node) = *can_see {
                // True: видим врага - атакуем
                see_node.true_action = Box::new(Some(DecisionNode::new(DecisionNodeType::Action)));
                // False: не видим - ищем
                see_node.false_action = Box::new(Some(DecisionNode::new(DecisionNodeType::Action)));
            }
            node.true_action = can_see;
            
            // False: врага нет рядом - патрулируем
            node.false_action = Box::new(Some(DecisionNode::new(DecisionNodeType::Action)));
        }
        
        root.true_action = enemy_nearby;
        
        DecisionTree {
            root,
            action_map: HashMap::new(),
        }
    }
    
    pub fn evaluate(&self, context: &BotContext) -> Option<String> {
        self.root.evaluate(context)
    }
}

pub struct BehaviorTree {
    pub actions: Vec<BotCommand>,
    pub current_action_idx: usize,
}

impl BehaviorTree {
    pub fn new() -> Self {
        BehaviorTree {
            actions: Vec::new(),
            current_action_idx: 0,
        }
    }
    
    pub fn add_action(&mut self, action: BotCommand) {
        self.actions.push(action);
    }
    
    pub fn sort_by_priority(&mut self) {
        self.actions.sort_by(|a, b| b.priority.cmp(&a.priority));
    }
    
    pub fn get_current_action(&self) -> Option<&BotCommand> {
        if self.current_action_idx < self.actions.len() {
            Some(&self.actions[self.current_action_idx])
        } else {
            None
        }
    }
    
    pub fn execute_next(&mut self) -> Option<&BotCommand> {
        if self.current_action_idx < self.actions.len() {
            self.current_action_idx += 1;
            self.get_current_action()
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TacticPattern {
    pub name: String,
    pub description: String,
    pub effectiveness: f32,
    pub movements: Vec<(f32, f32)>, // (x, y) смещения
}

impl TacticPattern {
    pub fn aggressive() -> Self {
        TacticPattern {
            name: "Aggressive".to_string(),
            description: "Быстро атакует, много движется".to_string(),
            effectiveness: 0.8,
            movements: vec![
                (50.0, 0.0),
                (0.0, 50.0),
                (-50.0, 0.0),
                (0.0, -50.0),
            ],
        }
    }
    
    pub fn defensive() -> Self {
        TacticPattern {
            name: "Defensive".to_string(),
            description: "Осторожный стиль, прячется".to_string(),
            effectiveness: 0.6,
            movements: vec![
                (10.0, 0.0),
                (0.0, 10.0),
                (-10.0, 0.0),
            ],
        }
    }
    
    pub fn tactical() -> Self {
        TacticPattern {
            name: "Tactical".to_string(),
            description: "Стратегический подход".to_string(),
            effectiveness: 0.85,
            movements: vec![
                (30.0, 0.0),
                (15.0, 25.0),
                (-30.0, 0.0),
                (-15.0, -25.0),
            ],
        }
    }
}
