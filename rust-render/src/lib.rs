use wasm_bindgen::prelude::*;
use web_sys::{self, console, CanvasRenderingContext2d};
use web_sys::window;

mod canvas;
mod drawing;
mod models;

use canvas::create_or_get_canvas;
use drawing::{draw_root_node, draw_child_node, draw_curve};
use models::Node;

#[wasm_bindgen]
pub fn render_mindmap(container_id: &str) {
    console::log_1(&"Rendering mindmap...".into());

    let window = window().expect("should have a window in this context");
    let document = window.document().expect("should have a document in this context");
    
    let canvas = create_or_get_canvas(&document, container_id).expect("Failed to create or get canvas");
    
    let context = canvas
        .get_context("2d")
        .expect("context should be available")
        .expect("context should be available")
        .dyn_into::<CanvasRenderingContext2d>()
        .expect("context should be a CanvasRenderingContext2d");

    // 清除之前的绘制内容
    context.clear_rect(0.0, 0.0, canvas.width().into(), canvas.height().into());

    // 定义节点数据和位置（简化示例）
    let nodes = vec![
        Node { name: "Root", x: 300.0, y: 100.0, is_root: true },
        Node { name: "Child 1", x: 200.0, y: 250.0, is_root: false },
        Node { name: "Child 2", x: 300.0, y: 250.0, is_root: false },
        Node { name: "Child 3", x: 400.0, y: 250.0, is_root: false },
    ];

    // 绘制连线（不同颜色弧线）
    draw_curve(&context, nodes[0].x, nodes[0].y, nodes[1].x, nodes[1].y, "purple");
    draw_curve(&context, nodes[0].x, nodes[0].y, nodes[2].x, nodes[2].y, "blue");
    draw_curve(&context, nodes[0].x, nodes[0].y, nodes[3].x, nodes[3].y, "green");

    // 绘制根节点和子节点
    for node in &nodes {
        if node.is_root {
            draw_root_node(&context, node.x, node.y, node.name);
        } else {
            draw_child_node(&context, node.x, node.y, node.name);
        }
    }
}