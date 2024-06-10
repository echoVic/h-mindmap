use wasm_bindgen::JsCast;
use web_sys::{HtmlCanvasElement, Element, Document};

pub fn create_or_get_canvas(document: &Document, container_id: &str) -> Result<HtmlCanvasElement, String> {
    // 获取容器元素
    let container = document.get_element_by_id(container_id)
        .ok_or(format!("Container with id {} not found", container_id))?;

    // 检查是否已有 Canvas 元素存在
    if let Some(existing_canvas) = document.get_element_by_id("mindmap-canvas") {
        return existing_canvas.dyn_into::<HtmlCanvasElement>().map_err(|_| "Failed to cast existing canvas element".to_string());
    }

    // 创建新的 Canvas 元素
    let new_canvas = document.create_element("canvas")
        .map_err(|_| "Failed to create canvas element".to_string())?
        .dyn_into::<HtmlCanvasElement>()
        .map_err(|_| "Failed to cast new canvas element".to_string())?;

    new_canvas.set_width(800);
    new_canvas.set_height(600);
    new_canvas.set_id("mindmap-canvas");
    container.append_child(&new_canvas).map_err(|_| "Failed to append canvas to container".to_string())?;

    Ok(new_canvas)
}