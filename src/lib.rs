use js_sys::{Array, Date, Number};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{console, Document, Element, HtmlElement, Window};

// #[wasm_bindgen(start)]
#[wasm_bindgen]
pub fn run(num: Number, arr: Array) -> Result<(), JsValue> {
    console::log_1(&format!("run::{:?}", &num).into());
    console::log_1(&format!("run::{:?}", &arr).into());
    console::group_1(&JsValue::from_str("Hello!"));
    arr.for_each(&mut |obj: JsValue, idx: u32, arr: js_sys::Array| {
        // Two std::fmt compiler errors:
        // `wasm_bindgen::JsValue` doesn't implement `std::fmt::Display`
        // `js_sys::Array` doesn't implement `std::fmt::Display`
        console::log_1(
            &format!("{arr:?}[{idx}] = {obj:?}", obj = obj, idx = idx, arr = arr).into(),
        );
    });
    console::group_end();

    let window: Window = web_sys::window().expect("should have a window in this context");
    let document: Document = window.document().expect("window should have a document");

    // One of the first interesting things we can do with closures is simply
    // access stack data in Rust!
    let array = Array::new();
    array.push(&"Hello".into());
    array.push(&1.into());
    let mut first_item = None;
    array.for_each(&mut |obj, idx, _arr| match idx {
        0 => {
            assert_eq!(obj, "Hello");
            first_item = obj.as_string();
        }
        1 => assert_eq!(obj, 1),
        _ => panic!("unknown index: {}", idx),
    });
    assert_eq!(first_item, Some("Hello".to_string()));

    // Below are some more advanced usages of the `Closure` type for closures
    // that need to live beyond our function call.

    setup_clock(&window, &document)?;
    setup_clicker(&document);

    // And now that our demo is ready to go let's switch things up so
    // everything is displayed and our loading prompt is hidden.
    document
        .get_element_by_id("loading")
        .expect("should have #loading on the page")
        .dyn_ref::<HtmlElement>()
        .expect("#loading should be an `HtmlElement`")
        .style()
        .set_property("display", "none")?;

    let body: HtmlElement = document.body().expect("document should have a body");

    // Manufacture the element we're gonna append
    let p_tag: Element = document.create_element("p")?;
    p_tag.set_inner_html("Hello from Rust!");

    body.append_child(&p_tag)?;
    Ok(())
}

// Set up a clock on our page and update it each second to ensure it's got
// an accurate date.
//
// Note the usage of `Closure` here because the closure is "long lived",
// basically meaning it has to persist beyond the call to this one function.
// Also of note here is the `.as_ref().unchecked_ref()` chain, which is how
// you can extract `&Function`, what `web-sys` expects, from a `Closure`
// which only hands you `&JsValue` via `AsRef`.
fn setup_clock(window: &Window, document: &Document) -> Result<(), JsValue> {
    let current_time = document
        .get_element_by_id("current-time")
        .expect("should have #current-time on the page");
    update_time(&current_time);
    let a = Closure::wrap(Box::new(move || update_time(&current_time)) as Box<dyn Fn()>);
    window
        .set_interval_with_callback_and_timeout_and_arguments_0(a.as_ref().unchecked_ref(), 1000)?;
    fn update_time(current_time: &Element) {
        current_time.set_inner_html(&String::from(
            Date::new_0().to_locale_string("en-GB", &JsValue::undefined()),
        ));
    }

    // The instance of `Closure` that we created will invalidate its
    // corresponding JS callback whenever it is dropped, so if we were to
    // normally return from `setup_clock` then our registered closure will
    // raise an exception when invoked.
    //
    // Normally we'd store the handle to later get dropped at an appropriate
    // time but for now we want it to be a global handler so we use the
    // `forget` method to drop it without invalidating the closure. Note that
    // this is leaking memory in Rust, so this should be done judiciously!
    a.forget();

    Ok(())
}

// We also want to count the number of times that our green square has been
// clicked. Our callback will update the `#num-clicks` div.
//
// This is pretty similar above, but showing how closures can also implement
// `FnMut()`.
fn setup_clicker(document: &Document) {
    let num_clicks = document
        .get_element_by_id("num-clicks")
        .expect("should have #num-clicks on the page");

    let mut clicks = 0;

    let a = Closure::wrap(Box::new(move || {
        clicks += 1;
        console::log_1(&format!("setup_clicker::clicks::{}", clicks).into());
        num_clicks.set_inner_html(&clicks.to_string());
    }) as Box<dyn FnMut()>);

    // DERP
    // Attach onclick handler to #green-square
    document
        .get_element_by_id("green-square")
        .expect("should have #green-square on the page")
        .dyn_ref::<HtmlElement>()
        .expect("#green-square be an `HtmlElement`")
        .set_onclick(Some(a.as_ref().unchecked_ref()));

    // See comments in `setup_clock` above for why we use `a.forget()`.
    a.forget();
}
