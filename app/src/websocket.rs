use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{ErrorEvent, MessageEvent, WebSocket};

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
        console_log!("Received message: {:?}", txt);
    } else {
        console_error!("Received unexpected message: {:?}", e.data());
    }
}

fn on_error(e: ErrorEvent) {
    console_error!("Socket error: {:?}", e);
}
