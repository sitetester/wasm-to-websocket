use ::log::{debug, error};
use js_sys::Promise;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{ErrorEvent, MessageEvent, WebSocket};

/// It will be called automatically when the WASM module is instantiated
/// `init_logging` function name can be anything
#[wasm_bindgen(start)]
pub fn init_logging() {
    // Initialize logging for JavaScript environments (browser or Node.js)
    wasm_logger::init(wasm_logger::Config::default());
}

/// Sends a text message to the specified WebSocket endpoint
/// #[wasm_bindgen] - Makes a function callable from JavaScript
///
/// # Arguments
/// * `endpoint` - WebSocket server URL to connect to
/// * `message` - Message to send over WebSocket
///
/// # Returns
/// * `Ok(String)` -  Message received from the WebSocket server
/// * `Err(JsValue)` - WebSocket connection or message sending error
#[wasm_bindgen]
pub async fn ws_ping(endpoint: String, message: String) -> Result<String, JsValue> {
    let ws = WebSocket::new(&endpoint)?;
    // Convert into reference-counted smart pointer for shared ownership
    let ws = Rc::new(ws);
    // Enables shared ownership of message across multiple closures without cloning data
    let message = Rc::new(message);

    // Create a new JavaScript Promise that will resolve when we get a response or reject if there's an error
    // These callbacks control the promise's state and determine whether .then() or .catch() handlers will be called
    // when the JavaScript code uses this promise
    let promise = Promise::new(&mut |resolve, reject| {
        debug!("Setting up WebSocket handlers...");
        // Wrap reject callback in Rc to share between multiple handlers
        let reject = Rc::new(reject);

        // Each handler needs its own reference to shared resources
        // `.clone()` simply creates a new reference (doesn't clone WebSocket or message or reject)
        let onopen = create_onopen_handler(ws.clone(), message.clone(), reject.clone());
        let onmessage = create_onmessage_handler(resolve, reject.clone());
        // Last usage of `reject` (clone not needed)
        let onerror = create_onerror_handler(reject);

        // `unchecked_ref()` will convert to the required JavaScript type (&Function)
        ws.set_onmessage(Some(onmessage.as_ref().unchecked_ref()));
        ws.set_onopen(Some(onopen.as_ref().unchecked_ref()));
        ws.set_onerror(Some(onerror.as_ref().unchecked_ref()));

        // Prevent Rust from dropping the closures when they go out of scope (intentional memory leak)
        // JavaScript still needs these callbacks to remain valid, otherwise, it could lead to undefined behavior or crashes)
        // Without forget(), the callbacks would be deallocated while JavaScript is still trying to use them
        onmessage.forget();
        onopen.forget();
        onerror.forget();
    });

    // Convert JavaScript Promise to Rust Future (a bridge) and await its result
    let result = JsFuture::from(promise).await?;
    // Convert JsValue to String
    Ok(result.as_string().unwrap_or_default())
}

/// Creates a WebSocket connection open event handler that sends the initial message
///
/// # Arguments
/// * `ws` - WebSocket instance wrapped in Rc for sharing across callbacks
/// * `message` - Text message to send when connection opens
/// * `reject_cb` - JavaScript function to call if sending fails
///
/// # Returns
/// `Closure<dyn FnMut()>` - Closure that handles WebSocket open event and
///  - Sends the provided message when connection is established
///  - Calls `reject_cb` if sending the message fails
fn create_onopen_handler(
    ws: Rc<WebSocket>,
    message: Rc<String>,
    reject_cb: Rc<js_sys::Function>,
) -> Closure<dyn FnMut()> {
    // Create a closure that will be called when WebSocket connection opens
    Closure::wrap(Box::new(move || {
        debug!("Sending message: {}", &message);
        // Try to send the message through WebSocket
        if let Err(err) = ws.send_with_str(&message) {
            error!("Failed to send message: {:?}", err);
            // Call reject callback with the error
            let _ = reject_cb.call1(&JsValue::NULL, &err);
        }
    }) as Box<dyn FnMut()>)
}

/// Creates a WebSocket message event handler that controls Promise resolution
///
/// # Arguments
/// * `resolve_fn` - JavaScript function to call when message is received
/// * `reject_fn` - JavaScript function to call when an error occurs
///
/// # Returns
/// `Closure<dyn FnMut(MessageEvent)>` - A Closure that handles incoming WebSocket messages and
///  - Calls `resolve_fn` with the message data
///  - Calls `reject_fn` if message processing fails
fn create_onmessage_handler(
    resolve_fn: js_sys::Function,
    reject_fn: Rc<js_sys::Function>,
) -> Closure<dyn FnMut(MessageEvent)> {
    // Create a closure that will be called when a message is received
    Closure::wrap(Box::new(move |e: MessageEvent| {
        // Try to extract message as string
        if let Some(text) = e.data().as_string() {
            // Call resolve callback with the received message (.then() in js)
            if resolve_fn
                .call1(&JsValue::NULL, &JsValue::from_str(&text))
                .is_err()
            {
                error!("Failed to process message");
            }
        } else {
            error!("Invalid message format received");
            // Call reject callback with error message (.catch() in js)
            let _ = reject_fn.call1(&JsValue::NULL, &JsValue::from_str("Invalid message type"));
        }
    }) as Box<dyn FnMut(MessageEvent)>)
}

/// Creates a WebSocket error event handler that controls Promise rejection
///
/// # Arguments
/// * `reject_fn` - JavaScript function to call when an error occurs
///
/// # Returns
/// `Closure<dyn FnMut(ErrorEvent)>` - A Closure that handles WebSocket error events and
///  Calls `reject_fn` with the error event to reject the Promise
fn create_onerror_handler(reject_fn: Rc<js_sys::Function>) -> Closure<dyn FnMut(ErrorEvent)> {
    // Create a closure that will be called if WebSocket encounters an error
    Closure::wrap(Box::new(move |e: ErrorEvent| {
        error!("{}", format!("WebSocket error occurred: {:?}", e));
        // Call reject_fn to reject the JavaScript Promise
        let _ = reject_fn.call1(&JsValue::NULL, &e);
    }) as Box<dyn FnMut(ErrorEvent)>)
}
