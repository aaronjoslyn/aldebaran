use wasm_bindgen::{JsCast, JsValue};
use web_sys::{HtmlCanvasElement, WebGlRenderingContext};

pub fn create_context() -> Result<WebGlRenderingContext, JsValue> {
    let window = web_sys::window().expect("Failed to find window.");
    let document = window.document().expect("Failed to find document.");
    let body = document.body().expect("Failed to find document body");
    let canvas: HtmlCanvasElement = document.create_element("canvas")?.dyn_into()?;
    canvas.set_attribute("height", &body.client_height().to_string())?;
    canvas.set_attribute("width", &body.client_width().to_string())?;
    body.append_child(&canvas)?;
    let gl: WebGlRenderingContext = canvas
        .get_context("webgl")?
        .expect("Failed to get WebGL context.")
        .dyn_into()?;
    gl.clear_color(0.0, 0.0, 0.0, 1.0);
    gl.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);
    Ok(gl)
}
