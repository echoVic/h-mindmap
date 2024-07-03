use web_sys::CanvasRenderingContext2d;
use crate::models::MindMap;

pub fn draw_root_node(context: &CanvasRenderingContext2d, x: f64, y: f64, text: &str) {
    context.begin_path();
    context.set_fill_style(&"#D3D3D3".into()); // 浅灰色背景
    context.set_stroke_style(&"black".into());
    context.set_line_width(2.0);

    let width = 100.0;
    let height = 50.0;
    let radius = 10.0;

    context.move_to(x - width / 2.0 + radius, y - height / 2.0);
    context.line_to(x + width / 2.0 - radius, y - height / 2.0);
    context.quadratic_curve_to(
        x + width / 2.0,
        y - height / 2.0,
        x + width / 2.0,
        y - height / 2.0 + radius,
    );
    context.line_to(x + width / 2.0, y + height / 2.0 - radius);
    context.quadratic_curve_to(x + width / 2.0, y + height / 2.0, x + width / 2.0 - radius, y + height / 2.0);
    context.line_to(x - width / 2.0 + radius, y + height / 2.0);
    context.quadratic_curve_to(x - width / 2.0, y + height / 2.0, x - width / 2.0, y + height / 2.0 - radius);
    context.line_to(x - width / 2.0, y - height / 2.0 + radius);
    context.quadratic_curve_to(
        x - width / 2.0,
        y - height / 2.0,
        x - width / 2.0 + radius,
        y - height / 2.0,
    );

    context.close_path();
    context.fill();
    context.stroke();

    context.set_fill_style(&"black".into());
    context.set_font("20px Arial");
    context.set_text_align("center");
    context.set_text_baseline("middle");
    context.fill_text(text, x, y).unwrap();
}

pub fn draw_child_node(context: &CanvasRenderingContext2d, x: f64, y: f64, text: &str) {
    context.begin_path(); // 确保开始新的路径
    let width = 80.0;
    let height = 30.0;

    context.set_font("15px Arial");
    context.set_fill_style(&"black".into());
    context.set_text_align("center");
    context.set_text_baseline("middle");
    context.fill_text(text, x, y).unwrap();
}

pub fn draw_curve(
    context: &CanvasRenderingContext2d,
    from_x: f64,
    from_y: f64,
    to_x: f64,
    to_y: f64,
    color: &str,
) {
    context.begin_path();
    context.set_stroke_style(&color.into());
    context.set_line_width(2.0);

    // 假设子节点是 80x30 大小
    let node_width = 80.0;
    let node_height = 30.0;

    // 计算根节点的边缘坐标
    let start_x = from_x;
    let start_y = from_y + 50.0 / 2.0; // 根节点高度的一半

    // 计算子节点的边缘坐标
    let end_x = to_x;
    let end_y = to_y - node_height / 2.0;

    // 绘制贝塞尔曲线
    let cp1x = start_x;
    let cp1y = (start_y + end_y) / 2.0;
    let cp2x = end_x;
    let cp2y = (start_y + end_y) / 2.0;

    context.move_to(start_x, start_y);
    context.bezier_curve_to(cp1x, cp1y, cp2x, cp2y, end_x, end_y);
    context.stroke();
}

pub fn render_mindmap(context: &CanvasRenderingContext2d, mindmap: &MindMap) {
    let canvas = context.canvas().expect("Should have a canvas");

    // 清除之前的绘制内容
    context.clear_rect(0.0, 0.0, canvas.width() as f64, canvas.height() as f64);

    // 绘制连线
    for node in &mindmap.nodes {
        if !node.is_root {
            draw_curve(context, mindmap.nodes[0].x, mindmap.nodes[0].y, node.x, node.y, "black");
        }
    }

    // 绘制根节点和子节点
    for node in &mindmap.nodes {
        if node.is_root {
            draw_root_node(context, node.x, node.y, &node.name);
        } else {
            draw_child_node(context, node.x, node.y, &node.name);
        }
    }
}