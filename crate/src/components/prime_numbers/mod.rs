use dom_interaction::Dom;

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
        &format!("Prime numbers: {:?}", functions::prime_numbers(1000)),
    );

    dom.append_all_to_element(
        &component_parent,
        &[instruction, input_element, enter_button, output],
    );

    dom.append_to_body(&component_parent);
}
