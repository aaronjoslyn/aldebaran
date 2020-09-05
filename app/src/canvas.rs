use wasm_bindgen::{JsCast, JsValue};
use web_sys::{HtmlCanvasElement, WebGl2RenderingContext};

pub fn create_context() -> Result<(HtmlCanvasElement, WebGl2RenderingContext), JsValue> {
    let window = web_sys::window().expect("Failed to find window.");
    let document = window.document().expect("Failed to find document.");
    let body = document.body().expect("Failed to find document body");
    let canvas: HtmlCanvasElement = document.create_element("canvas")?.dyn_into()?;
    canvas.set_attribute("height", &body.client_height().to_string())?;
    canvas.set_attribute("width", &body.client_width().to_string())?;
    body.append_child(&canvas)?;
    let gl: WebGl2RenderingContext = canvas
        .get_context("webgl2")?
        .expect("Failed to get WebGL context.")
        .dyn_into()?;
    gl.clear_color(0.0, 0.0, 0.0, 0.0);
    gl.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);
    Ok((canvas, gl))
}
