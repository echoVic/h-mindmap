use web_sys::CanvasRenderingContext2d;

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

pub fn draw_curve(context: &CanvasRenderingContext2d, x1: f64, y1: f64, x2: f64, y2: f64, color: &str) {
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