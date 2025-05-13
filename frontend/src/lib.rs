#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use common::app::*;
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(App);
}
