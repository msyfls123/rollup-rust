use wasm_bindgen::prelude::*;
use web_sys::{Document, console, HtmlInputElement, InputEvent};
use wasm_bindgen::JsCast;

#[wasm_bindgen]
pub fn init_file(id: &str) -> Result<(), JsValue> {
  let window = web_sys::window().expect("should have a window in this context");
  let document = window.document().expect("window should have a document");
  setup_file(&document, id)?;
  Ok(())
}

fn setup_file(document: &Document, id: &str) -> Result<(), JsValue> {

  let callback = Closure::wrap(Box::new(move |event: InputEvent| {
    let target = event.target().expect("event not have target");
    let element = target.dyn_ref::<HtmlInputElement>().expect("target is not HTMLInputElement");

    let file = element.files().unwrap().get(0).unwrap();
    console::log_1(&JsValue::from(file.name()));
    console::log_1(&JsValue::from(file));
  }) as Box<dyn Fn(_)>);
  document
    .get_element_by_id(id)
    .expect("should have file on the page")
    .dyn_ref::<HtmlInputElement>()
    .expect("file should be an HTMLElement")
    .add_event_listener_with_callback("change", callback.as_ref().unchecked_ref())?;
  callback.forget();
  Ok(())
}
