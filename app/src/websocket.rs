use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{console, ErrorEvent, MessageEvent, WebSocket};

pub fn connect_websocket() -> Result<(), JsValue> {
    let ws = WebSocket::new("ws://localhost:4000")?;
    ws.set_binary_type(web_sys::BinaryType::Arraybuffer);
    let onmessage_callback = Closure::wrap(Box::new(on_message) as Box<dyn FnMut(MessageEvent)>);
    ws.set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));
    onmessage_callback.forget();
    let onerror_callback = Closure::wrap(Box::new(on_error) as Box<dyn FnMut(ErrorEvent)>);
    ws.set_onerror(Some(onerror_callback.as_ref().unchecked_ref()));
    onerror_callback.forget();
    Ok(())
}

fn on_message(e: MessageEvent) {
    if let Ok(txt) = e.data().dyn_into::<js_sys::JsString>() {
        if txt == "Reload" {
            let window = web_sys::window().expect("Failed to find window.");
            window.location().reload().expect("Failed to reload page.");
        }
    } else {
        console::error_2(&"Received unexpected message:".into(), &e.data());
    }
}

fn on_error(e: ErrorEvent) {
    console::error_2(&"Socket error:".into(), &e);
}
