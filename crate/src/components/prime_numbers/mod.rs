use dom_interaction::Dom;
use web_sys::{console, Element, Event, HtmlInputElement};

mod functions;

fn create_prime_numbers_input(dom: &Dom) -> Element {
    let input_element = dom.create_element("input");
    dom.set_attribute(&input_element, "type", "number");
    dom.set_attribute(&input_element, "id", "prime_numbers_input");
    input_element
}

fn create_prime_numbers_output(dom: &Dom) -> Element {
    let output_element = dom.create_text_element(
        "p",
        &format!("Prime numbers: {:?}", functions::prime_numbers(100)),
    );
    dom.set_attribute(&output_element, "id", "prime_numbers_output");
    output_element
}

fn create_instruction_text(dom: &Dom) -> Element {
    dom.create_text_element(
        "p",
        "Enter a number to calculate prime numbers up to that number.",
    )
}

fn create_component_parent(dom: &Dom) -> Element {
    let parent = dom.create_element("div");

    dom.set_id(&parent, "prime-numbers");
    dom.set_class(&parent, "component");
    dom.append_to_body(&parent);

    parent
}

fn create_comoponent_action(dom: &Dom) -> Element {
    let enter_button = dom.create_text_element("button", "Enter");

    let primer_number_button_click = dom.make_event_listener_closure(move |_: Event| {
        let new_dom = Dom::new();
        let prime_input = new_dom.query_element("#prime_numbers_input");
        let prime_output = new_dom.query_element("#prime_numbers_output");

        let input_value = new_dom.get_ref::<HtmlInputElement>(&prime_input).value();
        let input_value_num = input_value.parse::<u32>().unwrap();

        console::time_with_label("rust_prime_numbers");
        prime_output.set_inner_html(&format!(
            "Prime numbers: {:?}",
            functions::prime_numbers(input_value_num)
        ));
        console::time_end_with_label("rust_prime_numbers");
        console::log_1(&"holy moley that's fast".into());
    });

    dom.make_event(&enter_button, "click", primer_number_button_click);

    enter_button
}

pub fn main(dom: &Dom) {
    let component_parent = create_component_parent(dom);
    let instruction = create_instruction_text(dom);
    let input_element = create_prime_numbers_input(dom);
    let output = create_prime_numbers_output(dom);
    let enter_button = create_comoponent_action(dom);

    dom.append_all_to_element(
        &component_parent,
        &[instruction, input_element, enter_button, output],
    );
}
