extern crate web_sys;
use web_sys::{Document, Element, HtmlElement, Window};

pub struct Dom {
    document: Document,
}

impl Dom {
    pub fn new() -> Dom {
        let window: Window = web_sys::window().expect("no global `window` exists");
        let document: Document = window.document().expect("should have a document on window");
        Dom { document }
    }

    pub fn set_style(&self, element: &Element, style: &str) {
        element.set_attribute("style", style).unwrap();
    }

    pub fn create_element(&self, tag: &str) -> Element {
        let created_element: Element = self.document.create_element(tag).unwrap();
        created_element
    }

    pub fn create_text_element(&self, element_name: &str, text: &str) -> Element {
        let val: Element = self.create_element(element_name);
        val.set_inner_html(text);
        val
    }

    pub fn append_to_body(&self, element: &Element) -> &Dom {
        let body: HtmlElement = self.document.body().unwrap();
        body.append_child(element).unwrap();
        self
    }
}
