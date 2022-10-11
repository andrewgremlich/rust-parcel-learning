use dom_interaction::Dom;
use wasm_bindgen::{closure::Closure, JsCast};
use web_sys::{console, Event, HtmlInputElement};

mod functions;

pub fn main(dom: &Dom) {
    let component_parent = dom.create_element("div");

    dom.set_id(&component_parent, "prime_numbers");

    let instruction = dom.create_text_element(
        "p",
        "Enter a number to calculate prime numbers up to that number.",
    );
    let input_element = dom.create_element("input");
    let enter_button = dom.create_text_element("button", "Enter");
    let output = dom.create_text_element(
        "p",
        &format!("Prime numbers: {:?}", functions::prime_numbers(100)),
    );

    dom.set_attribute(&input_element, "type", "number");
    dom.set_attribute(&input_element, "id", "prime_numbers_input");
    dom.set_attribute(&output, "id", "prime_numbers_output");

    let primer_number_button_click = Closure::wrap(Box::new(move |_event: Event| {
        let new_dom_query = Dom::new();
        let prime_input = new_dom_query.query_element("#prime_numbers_input");
        let prime_output = new_dom_query.query_element("#prime_numbers_output");

        let input_value = prime_input.dyn_ref::<HtmlInputElement>().unwrap().value();
        let input_value_num = input_value.parse::<u32>().unwrap();

        console::time_with_label("rust_prime_numbers");
        let prime_numbers = functions::prime_numbers(input_value_num);
        prime_output.set_inner_html(&format!("Prime numbers: {:?}", prime_numbers));
        console::time_end_with_label("rust_prime_numbers");

    }) as Box<dyn FnMut(_)>);

    match enter_button.add_event_listener_with_callback(
        "click",
        primer_number_button_click.as_ref().unchecked_ref(),
    ) {
        Ok(_) => primer_number_button_click.forget(),
        Err(e) => console::log_1(&format!("Error: {:?}", e).into()),
    };

    dom.append_all_to_element(
        &component_parent,
        &[instruction, input_element, enter_button, output],
    );

    dom.append_to_body(&component_parent);
}
