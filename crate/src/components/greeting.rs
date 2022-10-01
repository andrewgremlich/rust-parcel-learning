use dom_interaction::Dom;

pub fn main(dom: &Dom) {
    let greetings = dom.create_text_element("p", "Hello from Rust, WebAssembly, and Parcel!!!");

    dom.set_css_text(&greetings, "color: darkred;");

    dom.append_to_body(&greetings);
}
