use wasm_bindgen::prelude::*;

mod app;
mod components;
mod env;
mod models;
mod pages;
mod services;
mod setting;

#[wasm_bindgen(start)]
pub fn main() {
    yew::Renderer::<app::App>::new().render();
}
