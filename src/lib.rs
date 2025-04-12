use wasm_bindgen::prelude::*;

mod app;
mod pages;
mod data;
mod services;
mod errors;
mod env;
mod setting;

#[wasm_bindgen(start)]
pub fn main() {
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();
    
    yew::Renderer::<app::App>::new().render();
}
