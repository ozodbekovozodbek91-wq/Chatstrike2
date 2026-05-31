use std::collections::{BinaryHeap, HashSet, HashMap};
use std::cmp::Ordering;
use crate::models::Vector3;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PathNode {
    pub x: i32,
    pub y: i32,
}

impl PathNode {
    pub fn new(x: i32, y: i32) -> Self {
        PathNode { x, y }
    }
    
    pub fn distance(&self, other: &PathNode) -> f32 {
        (((self.x - other.x).pow(2) + (self.y - other.y).pow(2)) as f32).sqrt()
    }
    
    pub fn heuristic(&self, goal: &PathNode) -> f32 {
        ((self.x - goal.x).abs() + (self.y - goal.y).abs()) as f32
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct AStarNode {
    pub node: PathNode,
    pub g_cost: i32, // стоимость от старта
    pub h_cost: i32, // эвристика до цели
}

impl AStarNode {
    pub fn f_cost(&self) -> i32 {
        self.g_cost + self.h_cost
    }
}

impl Ord for AStarNode {
    fn cmp(&self, other: &Self) -> Ordering {
        other.f_cost().cmp(&self.f_cost()) // Для min-heap
    }
}

impl PartialOrd for AStarNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub struct NavMesh {
    pub walkable_nodes: HashSet<PathNode>,
    pub width: i32,
    pub height: i32,
    pub tile_size: f32,
}

impl NavMesh {
    pub fn new(width: i32, height: i32, tile_size: f32) -> Self {
        let mut walkable_nodes = HashSet::new();
        
        // Инициализируем все узлы как проходимые
        for x in 0..width {
            for y in 0..height {
                walkable_nodes.insert(PathNode::new(x, y));
            }
        }
        
        log::info!("🗭 Nav Mesh created: {}x{} tiles", width, height);
        
        NavMesh {
            walkable_nodes,
            width,
            height,
            tile_size,
        }
    }
    
    pub fn is_walkable(&self, node: &PathNode) -> bool {
        node.x >= 0 && node.x < self.width &&
        node.y >= 0 && node.y < self.height &&
        self.walkable_nodes.contains(node)
    }
    
    pub fn add_obstacle(&mut self, x: i32, y: i32) {
        self.walkable_nodes.remove(&PathNode::new(x, y));
    }
    
    pub fn get_neighbors(&self, node: &PathNode) -> Vec<PathNode> {
        let mut neighbors = Vec::new();
        let directions = [
            (-1, -1), (-1, 0), (-1, 1),
            (0, -1),           (0, 1),
            (1, -1),  (1, 0),  (1, 1),
        ];
        
        for (dx, dy) in directions.iter() {
            let next = PathNode::new(node.x + dx, node.y + dy);
            if self.is_walkable(&next) {
                neighbors.push(next);
            }
        }
        
        neighbors
    }
}

pub struct Pathfinder {
    pub nav_mesh: NavMesh,
}

impl Pathfinder {
    pub fn new(nav_mesh: NavMesh) -> Self {
        Pathfinder { nav_mesh }
    }
    
    /// A* алгоритм поиска пути
    pub fn find_path(&self, start: &PathNode, goal: &PathNode) -> Vec<PathNode> {
        if !self.nav_mesh.is_walkable(goal) {
            log::warn!("⚠️ Goal node is not walkable: {:?}", goal);
            return Vec::new();
        }
        
        let mut open_set = BinaryHeap::new();
        let mut came_from: HashMap<PathNode, PathNode> = HashMap::new();
        let mut g_score: HashMap<PathNode, i32> = HashMap::new();
        let mut closed_set: HashSet<PathNode> = HashSet::new();
        
        let start_h = ((start.x - goal.x).abs() + (start.y - goal.y).abs()) as i32;
        g_score.insert(start.clone(), 0);
        open_set.push(AStarNode {
            node: start.clone(),
            g_cost: 0,
            h_cost: start_h,
        });
        
        while let Some(AStarNode { node: current, g_cost, .. }) = open_set.pop() {
            if current == *goal {
                log::info!("🗭 Path found!");
                return self.reconstruct_path(&came_from, &current);
            }
            
            if closed_set.contains(&current) {
                continue;
            }
            closed_set.insert(current.clone());
            
            for neighbor in self.nav_mesh.get_neighbors(&current) {
                if closed_set.contains(&neighbor) {
                    continue;
                }
                
                let tentative_g_score = g_cost + (current.distance(&neighbor) as i32);
                
                if !g_score.contains_key(&neighbor) || tentative_g_score < g_score[&neighbor] {
                    came_from.insert(neighbor.clone(), current.clone());
                    g_score.insert(neighbor.clone(), tentative_g_score);
                    
                    let h_cost = ((neighbor.x - goal.x).abs() + (neighbor.y - goal.y).abs()) as i32;
                    open_set.push(AStarNode {
                        node: neighbor,
                        g_cost: tentative_g_score,
                        h_cost,
                    });
                }
            }
        }
        
        log::warn!("❌ No path found!");
        Vec::new()
    }
    
    fn reconstruct_path(
        &self,
        came_from: &HashMap<PathNode, PathNode>,
        current: &PathNode,
    ) -> Vec<PathNode> {
        let mut path = vec![current.clone()];
        let mut current = current.clone();
        
        while let Some(prev) = came_from.get(&current) {
            path.push(prev.clone());
            current = prev.clone();
        }
        
        path.reverse();
        path
    }
}

pub struct VisionSystem {
    pub view_range: f32,
    pub fov_angle: f32, // field of view в градусах
}

impl VisionSystem {
    pub fn new(view_range: f32, fov_angle: f32) -> Self {
        VisionSystem { view_range, fov_angle }
    }
    
    /// Проверяет, может ли бот видеть врага
    pub fn can_see_target(&self, bot_pos: &Vector3, bot_dir: &Vector3, target_pos: &Vector3) -> bool {
        let distance = bot_pos.distance(target_pos);
        
        if distance > self.view_range {
            return false;
        }
        
        // Вычисляем угол между направлением бота и целью
        let to_target = Vector3 {
            x: target_pos.x - bot_pos.x,
            y: target_pos.y - bot_pos.y,
            z: target_pos.z - bot_pos.z,
        };
        
        let dot_product = bot_dir.x * to_target.x + bot_dir.y * to_target.y + bot_dir.z * to_target.z;
        let bot_mag = (bot_dir.x.powi(2) + bot_dir.y.powi(2) + bot_dir.z.powi(2)).sqrt();
        let target_mag = (to_target.x.powi(2) + to_target.y.powi(2) + to_target.z.powi(2)).sqrt();
        
        if bot_mag == 0.0 || target_mag == 0.0 {
            return false;
        }
        
        let cos_angle = dot_product / (bot_mag * target_mag);
        let fov_rad = (self.fov_angle / 2.0).to_radians();
        
        cos_angle >= fov_rad.cos()
    }
}
