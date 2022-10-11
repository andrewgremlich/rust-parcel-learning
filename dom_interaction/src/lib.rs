use wasm_bindgen::{closure::Closure, convert::FromWasmAbi, JsCast};
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

    pub fn make_event_listener_closure<F: FnMut(T) -> () + 'static, T: FromWasmAbi + 'static>(
        &self,
        callback: F,
    ) -> Closure<dyn FnMut(T)> {
        Closure::wrap(Box::new(callback) as Box<dyn FnMut(_)>)
    }

    pub fn get_ref<'a, T: JsCast>(&'a self, ele: &'a Element) -> &T {
        ele.dyn_ref::<T>().unwrap()
    }

    pub fn query_element(&self, selector: &str) -> Element {
        self.document
            .query_selector(selector)
            .expect("should have a query selector")
            .expect("should have an element")
    }

    pub fn set_attribute(&self, element: &Element, attr_name: &str, attr_value: &str) {
        if let Ok(ele) = element.set_attribute(attr_name, attr_value) {
            return ele;
        }
    }

    pub fn set_id(&self, element: &Element, id: &str) {
        self.set_attribute(element, "id", id);
    }

    pub fn set_class(&self, element: &Element, class: &str) {
        self.set_attribute(element, "class", class);
    }

    pub fn set_css_text(&self, element: &Element, css_rule: &str) {
        if let Some(ele) = element.dyn_ref::<HtmlElement>() {
            ele.style().set_css_text(css_rule);
        }
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
