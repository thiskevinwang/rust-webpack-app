use std::cell::Cell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

// use wasm_bindgen::prelude::*;
// use wasm_bindgen::JsCast;
use web_sys::{ErrorEvent, MessageEvent, WebSocket};

use serde::{Deserialize, Serialize};
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document
        .create_element("canvas")?
        .dyn_into::<web_sys::HtmlCanvasElement>()?;
    canvas.set_id("my_canvas");
    canvas.set_width(640);
    canvas.set_height(480);
    canvas.style().set_property("border", "solid")?;
    canvas.style().set_property("margin", "auto")?;

    document.body().unwrap().append_child(&canvas)?;
    Ok(())
}

/// ***************
/// WEBSOCKET
/// ***************
/// - Websocket endpoint: "ws://localhost:3000/chat"
/// - Websocket send body: array
///   - `[number, number]`
/// - Websocket message: json
///   - `{ "data": "<User#92>: 730,53" }`
#[wasm_bindgen]
pub fn start_websocket() -> Result<(), JsValue> {
    let document = web_sys::window().unwrap().document().unwrap();

    let canvas = document
        .get_element_by_id("my_canvas")
        .expect("should have #loading on the page")
        .dyn_into::<web_sys::HtmlCanvasElement>()?;

    let context = canvas
        .get_context("2d")?
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()?;

    let context = Rc::new(context);
    // Connect to an echo server
    let ws = WebSocket::new("ws://localhost:3000/chat")?;
    {
        // Process incoming messages
        let context = context.clone();
        let onmessage_callback = Closure::wrap(Box::new(move |e: MessageEvent| {
            if let Ok(txt) = e.data().dyn_into::<js_sys::JsString>() {
                console_log!("{:?}", txt);
                let vec: js_sys::Array = txt.split(": ");

                // "<User#14>"
                let user_string: Option<String> = vec.get(0).as_string();
                if let Some(mut user_string) = user_string {
                    user_string.retain(|c| c.is_numeric()); // "14"
                    let user_id = user_string.parse::<i32>().unwrap(); // 14

                    // Set special colors for incoming messages
                    console_log!("setting stroke color...");
                    match user_id % 5 {
                        0 => context.set_stroke_style(&JsValue::from_str("Tomato")),
                        1 => context.set_stroke_style(&JsValue::from_str("Orange")),
                        2 => context.set_stroke_style(&JsValue::from_str("MediumSeaGreen")),
                        3 => context.set_stroke_style(&JsValue::from_str("DodgerBlue")),
                        4 => context.set_stroke_style(&JsValue::from_str("RebeccaPurple")),
                        _ => context.set_stroke_style(&JsValue::from_str("Gray")),
                    }
                }

                // "391,-46,false"
                let ws_string: Option<String> = vec.get(1).as_string();

                if let Some(ws_string) = ws_string {
                    // console_log!("{:?}", ws_string);
                    let coor: Vec<&str> = ws_string.split(",").collect();
                    // console_log!("{:?}", coor);
                    let x = (coor[0] as &str).parse::<f64>().unwrap() + 320.0;
                    let y = (coor[1] as &str).parse::<f64>().unwrap() + 240.0;
                    let is_drawing = (coor[2] as &str).parse::<bool>().unwrap();

                    // console_log!("{:?},{:?}, {:?}", x, y, is_drawing);

                    let context = context.clone();
                    if is_drawing {
                        context.line_to(x, y);
                        context.stroke();
                        context.begin_path();
                        context.move_to(x, y);
                    } else {
                        context.begin_path();
                        context.move_to(x, y);
                    }
                }
            } else {
                console_log!("message event, received Unknown: {:?}", e.data());
            }
        }) as Box<dyn FnMut(MessageEvent)>);
        // set message event handler on WebSocket
        ws.set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));
        // forget the callback to keep it alive
        onmessage_callback.forget();
    }

    {
        // Process errors
        let onerror_callback = Closure::wrap(Box::new(move |e: ErrorEvent| {
            console_log!("error event: {:?}", e);
        }) as Box<dyn FnMut(ErrorEvent)>);
        ws.set_onerror(Some(onerror_callback.as_ref().unchecked_ref()));
        onerror_callback.forget();
    }

    {
        // Do stuff when websocket connection is opened
        // Like attach mouse move handlers
        let cloned_ws = ws.clone();
        let onopen_callback = Closure::wrap(Box::new(move |_| {
            console_log!("socket opened");

            match cloned_ws.send_with_str("OPENED") {
                Ok(_) => console_log!("message successfully sent"),
                Err(err) => console_log!("error sending message: {:?}", err),
            }
        }) as Box<dyn FnMut(JsValue)>);

        ws.set_onopen(Some(onopen_callback.as_ref().unchecked_ref()));

        let pressed = Rc::new(Cell::new(false));

        {
            // ON MOUSE DOWN
            let context = context.clone();
            let pressed = pressed.clone();
            let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
                context.set_stroke_style(&JsValue::from_str("black"));
                context.begin_path();
                context.move_to(event.offset_x() as f64, event.offset_y() as f64);
                pressed.set(true);
            }) as Box<dyn FnMut(_)>);
            canvas
                .add_event_listener_with_callback("mousedown", closure.as_ref().unchecked_ref())?;
            closure.forget();
        }

        {
            // ON MOUSE MOVE
            let context = context.clone();
            let pressed = pressed.clone();
            let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
                let cloned_ws = ws.clone();
                if pressed.get() {
                    // DRAW
                    context.set_stroke_style(&JsValue::from_str("black"));
                    context.line_to(event.offset_x() as f64, event.offset_y() as f64);
                    context.stroke();
                    context.begin_path();
                    context.move_to(event.offset_x() as f64, event.offset_y() as f64);
                }
                // SEND WS MESSAGE
                // remember to offset the event coordinates
                let x = event.offset_x() as f64 - 320.0;
                let y = event.offset_y() as f64 - 240.0;
                cloned_ws.send_with_str(&format!("{:?},{:?},{:?}", x, y, pressed.get()));
            }) as Box<dyn FnMut(_)>);
            canvas
                .add_event_listener_with_callback("mousemove", closure.as_ref().unchecked_ref())?;
            closure.forget();
        }

        {
            // ON MOUSE UP
            let context = context.clone();

            let pressed = pressed.clone();
            let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
                pressed.set(false);
                context.set_stroke_style(&JsValue::from_str("black"));
                context.line_to(event.offset_x() as f64, event.offset_y() as f64);
                context.stroke();
            }) as Box<dyn FnMut(_)>);
            canvas.add_event_listener_with_callback("mouseup", closure.as_ref().unchecked_ref())?;
            closure.forget();
        }

        onopen_callback.forget();
    }

    Ok(())
}

/// A struct to hold some data from the github Branch API.
///
/// Note how we don't have to define every member -- serde will ignore extra
/// data when deserializing
#[derive(Debug, Serialize, Deserialize)]
pub struct Branch {
    pub name: String,
    pub commit: Commit,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Commit {
    pub sha: String,
    pub commit: CommitDetails,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommitDetails {
    pub author: Signature,
    pub committer: Signature,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Signature {
    pub name: String,
    pub email: String,
}

/// fetch demo
/// https://rustwasm.github.io/wasm-bindgen/examples/fetch.html
#[wasm_bindgen]
pub async fn fetch(repo: String) -> Result<JsValue, JsValue> {
    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);

    let url = format!("https://api.github.com/repos/{}/branches/master", repo);

    let request = Request::new_with_str_and_init(&url, &opts)?;

    request
        .headers()
        .set("Accept", "application/vnd.github.v3+json")?;

    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
    // console_log!("resp_value {:?}", resp_value);

    // `resp_value` is a `Response` object.
    assert!(resp_value.is_instance_of::<Response>());
    let resp: Response = resp_value.dyn_into().unwrap();
    // console_log!("resp {:?}", resp);

    // Convert this other `Promise` into a rust `Future`.
    let json = JsFuture::from(resp.json()?).await?;
    // console_log!("json {:?}", json);

    // Use serde to parse the JSON into a struct.
    let branch_info: Branch = json.into_serde().unwrap();
    // console_log!("branch_info {:?}", branch_info);

    // Send the `Branch` struct back to JS as an `Object`.
    Ok(JsValue::from_serde(&branch_info).unwrap())
}
