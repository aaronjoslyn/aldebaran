pub fn create_canvas() {
    let window = web_sys::window().expect("Failed to find window.");
    let document = window.document().expect("Failed to find document.");
    let body = document.body().expect("Failed to find document body");
    let canvas = document
        .create_element("canvas")
        .expect("Failed to create canvas.");
    canvas
        .set_attribute("height", &body.client_height().to_string())
        .expect("Failed to set height.");
    canvas
        .set_attribute("width", &body.client_width().to_string())
        .expect("Failed to set width.");
    body.append_child(&canvas)
        .expect("Failed to append canvas to body.");
}
