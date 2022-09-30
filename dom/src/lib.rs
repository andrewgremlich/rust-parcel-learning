extern crate web_sys;
use web_sys::{Document, Element, HtmlElement, Window};

pub fn create_element(tag: &str) -> Element {
    // Use `web_sys`'s global `window` function to get a handle on the global
    // window object.
    let window: Window = web_sys::window().expect("no global `window` exists");
    let document: Document = window.document().expect("should have a document on window");

    document
        .create_element(tag)
        .expect("should be able to create element")
}

// this might to better as a struct
pub fn create_text_element(element_name: &str, text: &str) -> Element {
    let window: Window = web_sys::window().expect("no global `window` exists");
    let document: Document = window.document().expect("should have a document on window");

    let val: Element = document.create_element(element_name).unwrap();
    val.set_inner_html(text);
    val
}

pub fn append_to_body(element: &Element) {
    let window: Window = web_sys::window().expect("no global `window` exists");
    let document: Document = window.document().expect("should have a document on window");
    let body: HtmlElement = document.body().expect("document should have a body");

    body.append_child(&element)
        .expect("should be able to append child");
}
