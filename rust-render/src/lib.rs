use wasm_bindgen::prelude::*;
use web_sys::{HtmlCanvasElement, CanvasRenderingContext2d};
use wasm_bindgen::JsCast;

#[wasm_bindgen]
pub fn render_mindmap() {
    web_sys::console::log_1(&"Rendering mindmap...".into());

    // 获取 Canvas 元素和 2D 渲染上下文
    let window = web_sys::window().expect("should have a window in this context");
    let document = window.document().expect("window should have a document");
    let canvas = document.get_element_by_id("mindmap-canvas")
        .expect("document should have a #mindmap-canvas on it")
        .dyn_into::<HtmlCanvasElement>()
        .expect("#mindmap-canvas should be an HtmlCanvasElement");

    let context = canvas
        .get_context("2d")
        .expect("context should be available")
        .expect("context should be available")
        .dyn_into::<CanvasRenderingContext2d>()
        .expect("context should be a CanvasRenderingContext2d");

    // 定义节点数据和位置（简化示例）
    let nodes = vec![
        ("Root", 300.0, 100.0, true),
        ("Child 1", 200.0, 250.0, false),
        ("Child 2", 300.0, 250.0, false),
        ("Child 3", 400.0, 250.0, false),
    ];

    // 绘制连线（不同颜色弧线）
    draw_curve(&context, nodes[0].1, nodes[0].2, nodes[1].1, nodes[1].2, "purple");
    draw_curve(&context, nodes[0].1, nodes[0].2, nodes[2].1, nodes[2].2, "blue");
    draw_curve(&context, nodes[0].1, nodes[0].2, nodes[3].1, nodes[3].2, "green");

    // 绘制根节点和子节点
    for (name, x, y, is_root) in &nodes {
        if *is_root {
            draw_root_node(&context, *x, *y, name);
        } else {
            draw_child_node(&context, *x, *y, name);
        }
    }
}

// 绘制根节点函数（浅灰色填充带四角弧形长方形，黑色文字）
fn draw_root_node(context: &CanvasRenderingContext2d, x: f64, y: f64, text: &str) {
    context.begin_path();
    context.set_fill_style(&"#D3D3D3".into()); // 浅灰色背景
    context.set_stroke_style(&"black".into());
    context.set_line_width(2.0);

    let width = 100.0;
    let height = 50.0;
    let radius = 10.0;

    context.move_to(x - width / 2.0 + radius, y - height / 2.0);
    context.line_to(x + width / 2.0 - radius, y - height / 2.0);
    context.quadratic_curve_to(x + width / 2.0, y - height / 2.0, x + width / 2.0, y - height / 2.0 + radius);
    context.line_to(x + width / 2.0, y + height / 2.0 - radius);
    context.quadratic_curve_to(x + width / 2.0, y + height / 2.0, x + width / 2.0 - radius, y + height / 2.0);
    context.line_to(x - width / 2.0 + radius, y + height / 2.0);
    context.quadratic_curve_to(x - width / 2.0, y + height / 2.0, x - width / 2.0, y + height / 2.0 - radius);
    context.line_to(x - width / 2.0, y - height / 2.0 + radius);
    context.quadratic_curve_to(x - width / 2.0, y - height / 2.0, x - width / 2.0 + radius, y - height / 2.0);

    context.close_path();
    context.fill();
    context.stroke();

    context.set_fill_style(&"black".into());
    context.set_font("20px Arial");
    context.set_text_align("center");
    context.set_text_baseline("middle");
    context.fill_text(text, x, y).unwrap();
}

// 绘制子节点函数（带四角弧形长方形，无框线，黑色文字）
fn draw_child_node(context: &CanvasRenderingContext2d, x: f64, y: f64, text: &str) {
    context.begin_path(); // 确保开始新的路径
    let width = 80.0;
    let height = 30.0;
    let radius = 10.0;

    context.set_font("15px Arial");
    context.set_fill_style(&"black".into());
    context.set_text_align("center");
    context.set_text_baseline("middle");
    context.fill_text(text, x, y).unwrap();
}

// 绘制弧线函数（可指定颜色）
fn draw_curve(context: &CanvasRenderingContext2d, x1: f64, y1: f64, x2: f64, y2: f64, color: &str) {
    context.begin_path();
    context.set_stroke_style(&color.into());
    context.set_line_width(2.0);

    // 绘制贝塞尔曲线
    let cp1x = x1;
    let cp1y = (y1 + y2) / 2.0;
    let cp2x = x2;
    let cp2y = (y1 + y2) / 2.0;
    context.move_to(x1, y1);
    context.bezier_curve_to(cp1x, cp1y, cp2x, cp2y, x2, y2);
    context.stroke();
}