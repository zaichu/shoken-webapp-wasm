use wasm_bindgen::prelude::*;
use yew::prelude::*;

pub struct App {
    count: i32,
}

pub enum Msg {
    Increment,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self { count: 0 }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Increment => {
                self.count += 1;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <h1>{ "カウント: " }{ self.count }</h1>
                <button onclick={ctx.link().callback(|_| Msg::Increment)}>{ "+1" }</button>
            </div>
        }
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    yew::Renderer::<App>::new().render();
}
