use wasm_bindgen::JsCast;
use web_sys::{Document, Element, HtmlElement};

pub struct Dom {
    document: Document,
}

impl Dom {
    pub fn new() -> Self {
        let window = web_sys::window().expect("no global `window` exists");
        let document = window.document().expect("should have a document on window");

        Self { document }
    }

    pub fn set_attribute(&self, element: &Element, attr_name: &str, attr_value: &str) {
        element.set_attribute(attr_name, attr_value).unwrap();
    }

    pub fn set_id(&self, element: &Element, id: &str) {
        self.set_attribute(element, "id", id);
    }

    pub fn set_class(&self, element: &Element, class: &str) {
        self.set_attribute(element, "class", class);
    }

    pub fn set_css_text(&self, element: &Element, css_rule: &str) {
        let element = element.dyn_ref::<HtmlElement>().unwrap();
        element.style().set_css_text(css_rule);
    }

    pub fn create_element(&self, tag: &str) -> Element {
        let created_element = self.document.create_element(tag).unwrap();
        created_element
    }

    pub fn create_text_element(&self, element_name: &str, text: &str) -> Element {
        let val = self.create_element(element_name);
        val.set_inner_html(text);
        val
    }

    pub fn append_to_body(&self, element: &Element) -> &Dom {
        let body = self.document.body().unwrap();
        body.append_child(element).unwrap();
        self
    }

    pub fn append_all_to_body(&self, elements: &[Element]) -> &Dom {
        for element in elements {
            self.append_to_body(element);
        }
        self
    }

    pub fn append_to_element(&self, parent: &Element, child: &Element) -> &Dom {
        parent.append_child(child).unwrap();
        self
    }

    pub fn append_all_to_element(&self, parent: &Element, children: &[Element]) -> &Dom {
        for child in children {
            self.append_to_element(parent, child);
        }
        self
    }
}
