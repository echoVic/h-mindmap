use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{console, HtmlCanvasElement, CanvasRenderingContext2d, MouseEvent};
use crate::{MINDMAP, drawing::render_mindmap};

// 添加事件监听器
pub fn add_event_listeners(canvas: &HtmlCanvasElement) {
    // 防御性代码，确保类型正确性
    if !is_html_canvas_element(canvas) {
        console::log_1(&"Canvas element is not an HtmlCanvasElement".into());
        return;
    } else {
        console::log_1(&"Canvas element is an HtmlCanvasElement".into());
    }

    let canvas_clone = canvas.clone();
    let closure = Closure::wrap(Box::new(move |event: MouseEvent| {
        if let Err(e) = on_canvas_click(event.clone(), canvas_clone.clone()) {
            console::error_1(&e);
        }
    }) as Box<dyn FnMut(_)>);
    let _ = canvas.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref());
    closure.forget();

    let canvas_clone = canvas.clone();
    let closure = Closure::wrap(Box::new(move |event: MouseEvent| {
        if let Err(e) = on_canvas_mousemove(event.clone(), canvas_clone.clone()) {
            console::error_1(&e);
        }
    }) as Box<dyn FnMut(_)>);
    let _ = canvas.add_event_listener_with_callback("mousemove", closure.as_ref().unchecked_ref());
    closure.forget();

    let canvas_clone = canvas.clone();
    let closure = Closure::wrap(Box::new(move |event: MouseEvent| {
        if let Err(e) = on_canvas_mouseup(event.clone(), canvas_clone.clone()) {
            console::error_1(&e);
        }
    }) as Box<dyn FnMut(_)>);
    let _ = canvas.add_event_listener_with_callback("mouseup", closure.as_ref().unchecked_ref());
    closure.forget();

    let canvas_clone = canvas.clone();
    let closure = Closure::wrap(Box::new(move |event: MouseEvent| {
        if let Err(e) = on_canvas_mousedown(event.clone(), canvas_clone.clone()) {
            console::error_1(&e);
        }
    }) as Box<dyn FnMut(_)>);
    let _ = canvas.add_event_listener_with_callback("mousedown", closure.as_ref().unchecked_ref());
    closure.forget();
}

// 判断是否为 HtmlCanvasElement
fn is_html_canvas_element(obj: &JsValue) -> bool {
    obj.is_instance_of::<HtmlCanvasElement>()
}

// 点击事件处理程序
fn on_canvas_click(event: MouseEvent, canvas: HtmlCanvasElement) -> Result<(), JsValue> {
    // 输出调试信息
    console::log_1(&"on_canvas_click called".into());

    // 防御性代码，确保类型正确性
    if !is_html_canvas_element(&canvas) {
        return Err(JsValue::from_str("Object is not an HtmlCanvasElement"));
    }
    
    // 获取边界矩形
    let rect = canvas.get_bounding_client_rect();
    console::log_1(&"Bounding rect fetched".into());

    // 获取点击坐标
    let x = event.client_x() as f64 - rect.left();
    let y = event.client_y() as f64 - rect.top();

    // 为调试打印位置
    console::log_2(&"Mouse clicked at:".into(), &format!("({}, {})", x, y).into());

    // 尝试借用 mindmap 并选择节点
    let selected_node_id: Option<usize> = unsafe {
        MINDMAP.as_ref().and_then(|mindmap| {
            mindmap.nodes.iter().enumerate().find_map(|(id, node)| {
                let (width, height) = if node.is_root { (100.0, 50.0) } else { (80.0, 30.0) };

                // 为调试打印节点
                console::log_1(&format!("Checking node id at ({}, {})", node.x, node.y).into());

                if x >= node.x - width / 2.0 && x <= node.x + width / 2.0 &&
                   y >= node.y - height / 2.0 && y <= node.y + height / 2.0 {
                    Some(id)
                } else {
                    None
                }
            })
        })
    };

    // 若找到节点，进行修改
    if let Some(node_id) = selected_node_id {
        unsafe {
            if let Some(ref mut mindmap) = MINDMAP {
                mindmap.select_node(node_id);
                console::log_1(&"Node selected".into());
            }
        }
    } else {
        console::log_1(&"No node selected".into());
    }

    Ok(())
}

fn on_canvas_mousedown(event: MouseEvent, canvas: HtmlCanvasElement) -> Result<(), JsValue> {
    // 防御性代码，确保类型正确性
    if !is_html_canvas_element(&canvas) {
        return Err(JsValue::from_str("Object is not an HtmlCanvasElement"));
    }

    let rect = canvas.get_bounding_client_rect();

    let x = event.client_x() as f64 - rect.left();
    let y = event.client_y() as f64 - rect.top();

    static mut LAST_X: f64 = 0.0;
    static mut LAST_Y: f64 = 0.0;

    unsafe {
        LAST_X = x;
        LAST_Y = y;
    }

    Ok(())
}

fn on_canvas_mousemove(event: MouseEvent, canvas: HtmlCanvasElement) -> Result<(), JsValue> {
    // 防御性代码，确保类型正确性
    if !is_html_canvas_element(&canvas) {
        return Err(JsValue::from_str("Object is not an HtmlCanvasElement"));
    }

    let rect = canvas.get_bounding_client_rect();

    let x = event.client_x() as f64 - rect.left();
    let y = event.client_y() as f64 - rect.top();

    static mut LAST_X: f64 = 0.0;
    static mut LAST_Y: f64 = 0.0;

    unsafe {
        if event.buttons() & 1 == 1 {
            let dx = x - LAST_X;
            let dy = y - LAST_Y;

            console::log_3(&"Mouse moved:".into(), &format!("dx: {}", dx).into(), &format!("dy: {}", dy).into());

            if let Some(ref mut mindmap) = MINDMAP {
                mindmap.move_selected_node(dx, dy);
                let context = canvas
                    .get_context("2d")
                    .map_err(|_| JsValue::from_str("Failed to get 2d context"))?
                    .ok_or_else(|| JsValue::from_str("Context should be available"))?
                    .dyn_into::<CanvasRenderingContext2d>()
                    .map_err(|_| JsValue::from_str("Context should be a CanvasRenderingContext2d"))?;

                render_mindmap(&context, mindmap);
            }

            LAST_X = x;
            LAST_Y = y;
        }
    }

    Ok(())
}

fn on_canvas_mouseup(_event: MouseEvent, canvas: HtmlCanvasElement) -> Result<(), JsValue> {
    // 防御性代码，确保类型正确性
    if !is_html_canvas_element(&canvas) {
        return Err(JsValue::from_str("Object is not an HtmlCanvasElement"));
    }

    unsafe {
        if let Some(ref mindmap) = MINDMAP {
            let context = canvas
                .get_context("2d")
                .map_err(|_| JsValue::from_str("Failed to get 2d context"))?
                .ok_or_else(|| JsValue::from_str("Context should be available"))?
                .dyn_into::<CanvasRenderingContext2d>()
                .map_err(|_| JsValue::from_str("Context should be a CanvasRenderingContext2d"))?;
            render_mindmap(&context, mindmap);
        }
    }

    Ok(())
}