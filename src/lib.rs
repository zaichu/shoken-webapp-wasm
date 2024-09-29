use wasm_bindgen::prelude::*;

mod app;
mod components;
mod pages;
mod routes;

#[wasm_bindgen(start)]
pub fn main() {
    yew::Renderer::<app::App>::new().render();
}
