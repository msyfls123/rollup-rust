use wasm_bindgen::prelude::*;
use js_sys::{Date};
use web_sys::{Document, Element, Window, console, MouseEvent, HtmlElement};
use wasm_bindgen::JsCast;

#[wasm_bindgen]
pub fn clock(time: i32) -> Result<(), JsValue> {
    let window = web_sys::window().expect("should have a window in this context");
    let document = window.document().expect("window should have a document");
    setup_clock(&window, &document, time)
}

// Set up a clock on our page and update it each second to ensure it's got
// an accurate date.
//
// Note the usage of `Closure` here because the closure is "long lived",
// basically meaning it has to persist beyond the call to this one function.
// Also of note here is the `.as_ref().unchecked_ref()` chain, which is how
// you can extract `&Function`, what `web-sys` expects, from a `Closure`
// which only hands you `&JsValue` via `AsRef`.
fn setup_clock(window: &Window, document: &Document, time: i32) -> Result<(), JsValue> {
    let current_time = document
        .get_element_by_id("current-time")
        .expect("should have #current-time on the page");
    update_time(&current_time);
    let a = Closure::wrap(Box::new(move || update_time(&current_time)) as Box<dyn Fn()>);
    window
        .set_interval_with_callback_and_timeout_and_arguments_0(a.as_ref().unchecked_ref(), time)?;
    fn update_time(current_time: &Element) {
        current_time.set_inner_html(&String::from(
            Date::new_0().to_locale_string("cn-ZH", &JsValue::undefined()),
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

    let b = Closure::wrap(Box::new(move |event: MouseEvent| {
        let target = event.target().expect("event not have target");
        let element = target.dyn_ref::<HtmlElement>().expect("target is not element");
        console::log_1(&JsValue::from(element.text_content().unwrap()));
    }) as Box<dyn Fn(_)>);

    document
        .get_element_by_id("current-time")
        .expect("should have #current-time on the page")
        .dyn_ref::<HtmlElement>()
        .expect("#current-time be an `HtmlElement`")
        .add_event_listener_with_callback("click", b.as_ref().unchecked_ref())?;

    b.forget();

    Ok(())
}
