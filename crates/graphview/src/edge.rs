#[derive(Clone)]
pub struct GraphEdge {
    pub source: usize,
    pub target: usize,
    pub path: Vec<(f32, f32)>, // Edge waypoints in pixels
}

impl GraphEdge {
    pub fn new(source: usize, target: usize) -> Self {
        Self {
            source,
            target,
            path: vec![],
        }
    }
}
