use std::ops::Deref;

use wasm_bindgen::{self, JsCast, JsValue};
use web_sys::{self, Element, Window, window, console, Event};
use leptos_reactive::{self, create_signal, create_runtime, create_scope, Scope, SignalUpdate, SignalGet, create_effect};

fn main() {
    mount(
        E1::new("button")
    )
}

fn mount(root: E1){
    let window = window().unwrap();
    let document = window.document().unwrap();
    let body = document.body().unwrap();
    body.append_child(&root).unwrap();
}

#[derive(Debug, Clone)]
pub struct E1(Element);

impl Deref for E1 {
    type Target = Element;

    fn deref(&self) -> &self::Target {
        &self.0
    }
}

impl E1 {
    pub fn new(tag_name: &str) -> Self {
        let window = window().unwrap();
        let document = window.document().unwrap();
        let e1 = document.create_element(tag_name).unwrap();
        self(e1)
    }
}
