use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{self, console, HtmlCanvasElement, CanvasRenderingContext2d, window};

mod canvas;
mod events;
mod drawing;
mod models;

use canvas::create_or_get_canvas;
use drawing::render_mindmap;
use events::add_event_listeners;
use models::MindMap;

static mut MINDMAP: Option<MindMap> = None;

#[wasm_bindgen]
pub fn init_mindmap(container_id: &str) {
    console::log_1(&"Initializing mindmap...".into());

    let window = window().expect("should have a window in this context");
    let document = window.document().expect("should have a document in this context");

    let canvas = create_or_get_canvas(&document, container_id).expect("Failed to create or get canvas");

    let context = canvas.get_context("2d")
        .expect("context should be available")
        .expect("context should be available")
        .dyn_into::<CanvasRenderingContext2d>()
        .expect("context should be a CanvasRenderingContext2d");

    // 初始化 MindMap 实例
    unsafe {
        MINDMAP = Some(MindMap::new());
    }

    // 添加事件监听器
    add_event_listeners(&canvas);

    // 渲染初始 MindMap
    unsafe { render_mindmap(&context, MINDMAP.as_ref().unwrap()) };
}

#[wasm_bindgen]
pub fn add_node(name: String, x: f64, y: f64, is_root: bool) {
    unsafe {
        if let Some(ref mut mindmap) = MINDMAP {
            mindmap.add_node(name, x, y, is_root);

            // 重新渲染画布
            let window = window().expect("should have a window in this context");
            let document = window.document().expect("should have a document in this context");
            let canvas_element = document.get_element_by_id("mindmap-canvas").expect("Canvas element not found");

            // 获取Canvas类型的引用
            let canvas = canvas_element
                .dyn_into::<HtmlCanvasElement>()
                .expect("Canvas element should be an HtmlCanvasElement");

            let context = canvas.get_context("2d")
                .expect("Context should be available")
                .expect("Context should be available")
                .dyn_into::<CanvasRenderingContext2d>()
                .expect("Context should be a CanvasRenderingContext2d");

            render_mindmap(&context, mindmap);
        }
    }
}