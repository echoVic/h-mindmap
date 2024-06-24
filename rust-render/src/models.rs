pub struct Node {
    pub id: usize,
    pub name: String,
    pub x: f64,
    pub y: f64,
    pub is_root: bool,
}

pub struct MindMap {
    pub nodes: Vec<Node>,
    next_id: usize,
}

impl MindMap {
    pub fn new() -> Self {
        MindMap {
            nodes: vec![Node { id: 0, name: "Root".to_string(), x: 300.0, y: 100.0, is_root: true }],
            next_id: 1,
        }
    }

    pub fn add_node(&mut self, name: String, x: f64, y: f64, is_root: bool) {
        self.nodes.push(Node {
            id: self.next_id,
            name,
            x,
            y,
            is_root,
        });
        self.next_id += 1;
    }
}