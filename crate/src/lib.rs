#[macro_use]
extern crate cfg_if;

extern crate wasm_bindgen;
extern crate web_sys;
use wasm_bindgen::prelude::*;
use web_sys::{Document, Element, HtmlElement, Window};

mod functions;

cfg_if! {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function to get better error messages if we ever panic.
    if #[cfg(feature = "console_error_panic_hook")] {
        extern crate console_error_panic_hook;
        use console_error_panic_hook::set_once as set_panic_hook;
    } else {
        #[inline]
        fn set_panic_hook() {}
    }
}

cfg_if! {
    // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
    // allocator.
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

// Called by our JS entry point to run the example
#[wasm_bindgen]
pub fn run() -> Result<(), JsValue> {
    // If the `console_error_panic_hook` feature is enabled this will set a panic hook, otherwise
    // it will do nothing.
    set_panic_hook();

    // Manufacture the element we're gonna append
    let val: Element = dom::create_text_element("p", "Hello from Rust, WebAssembly, and Parcel!!!");

    let prime_numbers_element: Element = dom::create_text_element(
        "p",
        &format!("Prime numbers: {:?}", functions::prime_numbers(1000)),
    );

    dom::append_to_body(&val);
    dom::append_to_body(&prime_numbers_element);

    Ok(())
}
