use dom_interaction::Dom;

mod greeting;
mod prime_numbers;

pub fn main() {
    let dom = Dom::new();

    greeting::main(&dom);
    prime_numbers::main(&dom);
}
