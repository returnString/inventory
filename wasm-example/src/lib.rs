extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

struct Flag {
    short: char,
    name: &'static str,
}

impl Flag {
    const fn new(short: char, name: &'static str) -> Self {
        Flag { short, name }
    }
}

struct OtherThing(i32);

inventory::submit! {
    Flag::new('v', "verbose")
}

inventory::collect!(Flag);

inventory::submit! {
    OtherThing(1)
}

inventory::submit! {
    OtherThing(2)
}

inventory::collect!(OtherThing);

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn log_flags() {
    for flag in inventory::iter::<Flag> {
        log(&format!("-{}, --{}", flag.short, flag.name));
    }
}

#[wasm_bindgen]
pub fn log_other_things() {
    for other in inventory::iter::<OtherThing> {
        log(&format!("value = {}", other.0));
    }
}
