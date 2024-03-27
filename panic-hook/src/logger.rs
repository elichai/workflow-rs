use std::result::Result;
use std::sync::Mutex;
use wasm_bindgen::prelude::*;
use web_sys::{Document, Element};

pub fn document() -> Document {
    let window = web_sys::window().expect("no global `window` exists");
    window.document().expect("unable to get `document` node")
}

#[derive(Clone)]
struct Logger {
    element: Element,
}

unsafe impl Send for Logger {}

static LOGGER: Mutex<Option<Logger>> = Mutex::new(None);

impl Logger {
    fn new() -> Result<Self, JsValue> {
        let element = document().create_element("pre")?;
        element.set_attribute("class", "wasm-logs")?;
        element.set_attribute("style", "display:none")?;

        document()
            .body()
            .expect("Unable to find body element")
            .append_child(&element)?;
        Ok(Self { element })
    }

    fn get() -> Option<Logger> {
        LOGGER.lock().unwrap().clone()
    }

    fn log_error(&self, msg: String) {
        let html = self.element.inner_html() + &msg;
        self.element.set_inner_html(&html);
    }
    fn show_element(&self) -> Result<(), JsValue> {
        self.element.remove_attribute("style")?;
        Ok(())
    }
}

pub fn error(msg: String) {
    if let Some(logger) = Logger::get() {
        logger.log_error(msg);
    }
}

pub fn init_logger() {
    match Logger::new() {
        Ok(l) => *LOGGER.lock().unwrap() = Some(l),
        Err(_e) => {
            panic!("unable to create Logger");
        }
    }
}

//unsafe impl Sync for Logger{}
pub fn show_logs() {
    if let Some(logger) = Logger::get() {
        let _r = logger.show_element();
    }
}
