pub struct Node {
    pub id: usize,
    pub name: String,
    pub x: f64,
    pub y: f64,
    pub is_root: bool,
    pub selected: bool,
}

pub struct MindMap {
    pub nodes: Vec<Node>,
    next_id: usize,
}

impl MindMap {
    pub fn new() -> Self {
        MindMap {
            nodes: vec![Node { id: 0, name: "Root".to_string(), x: 300.0, y: 100.0, is_root: true, selected: false }],
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
            selected: false,
        });
        self.next_id += 1;
    }

    pub fn select_node(&mut self, node_id: usize) {
        for node in &mut self.nodes {
            node.selected = node.id == node_id;
        }
    }

    pub fn clear_selection(&mut self) {
        for node in &mut self.nodes {
            node.selected = false;
        }
    }

    pub fn move_selected_node(&mut self, dx: f64, dy: f64) {
        for node in &mut self.nodes {
            if node.selected {
                node.x += dx;
                node.y += dy;
            }
        }
    }
}