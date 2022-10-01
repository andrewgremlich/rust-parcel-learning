use dom_interaction::Dom;

mod functions;

pub fn main(dom: &Dom) {
    let prime_numbers_element = dom.create_text_element(
        "p",
        &format!("Prime numbers: {:?}", functions::prime_numbers(1000)),
    );

    dom.append_to_body(&prime_numbers_element);
}
