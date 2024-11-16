use wasm_bindgen::prelude::*;

mod app;
mod components;
mod pages;
mod setting;

#[wasm_bindgen(start)]
pub fn main() {
    yew::Renderer::<app::App>::new().render();
}
