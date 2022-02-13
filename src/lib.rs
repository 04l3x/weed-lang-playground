#![recursion_limit = "256"]
use wasm_bindgen::prelude::*;
use wasm_logger;

mod app;
mod console;
mod editor;
mod utils;

#[wasm_bindgen(start)]
pub fn yewterial() -> Result<(), JsValue> {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<app::App>();
    Ok(())
}

#[cfg(test)]
pub mod tests {
    use wasm_bindgen_test::wasm_bindgen_test;

    #[wasm_bindgen_test]
    pub fn unit_test() {
        assert!(true)
    }
}
