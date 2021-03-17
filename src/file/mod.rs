use wasm_bindgen::prelude::*;
use web_sys::{
  Document, console, 
  HtmlInputElement, InputEvent, HtmlImageElement, HtmlElement,
  File, FileReader
};
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
    read_file(&file).unwrap_throw();
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

fn read_file(file: &File) -> Result<(), JsValue> {
  let window = web_sys::window().expect("should have a window in this context");
  let document = window.document().expect("window should have a document");
  let reader = FileReader::new()?;
  let reader2 = reader.clone();
  let callback = Closure::wrap(Box::new(move || {
    let ele = document
      .get_element_by_id("preview")
      .expect("should have file on the page");
    let img = ele
      .dyn_ref::<HtmlImageElement>()
      .expect("file should be an HTMLElement");
    let result = reader2.result().unwrap();
    img.set_src(&result.as_string().unwrap());
    let style = ele.dyn_ref::<HtmlElement>().expect("not HtmlElement").style();
    style.set_property("display", "block").unwrap();
  }) as Box<dyn Fn()>);
  reader.set_onload(Some(callback.as_ref().unchecked_ref()));
  reader.read_as_data_url(&file.as_ref())?;
  callback.forget();
  Ok(())
}
