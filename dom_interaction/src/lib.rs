extern crate web_sys;
use web_sys::{Document, Element, HtmlElement};

pub struct Dom {
    document: Document,
}

impl Dom {
    pub fn new() -> Dom {
        let window = web_sys::window().expect("no global `window` exists");
        let document = window.document().expect("should have a document on window");
        Dom { document }
    }

    pub fn create_element(&self, tag: &str) -> Element {
        self.document.create_element(tag).unwrap()
    }

    pub fn create_text_element(&self, element_name: &str, text: &str) -> Element {
        let val: Element = self.document.create_element(element_name).unwrap();
        val.set_inner_html(text);
        val
    }

    pub fn append_to_body(&self, element: &Element) {
        let body: HtmlElement = self.document.body().unwrap();
        body.append_child(element).unwrap();
    }
}
