use crate::pages::{Home, Receipts, Search};
use wasm_bindgen::JsValue;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/shoken-webapp-wasm/")]
    Home,
    #[at("/shoken-webapp-wasm/receipts")]
    Receipts,
    #[at("/shoken-webapp-wasm/search")]
    Search,
    #[not_found]
    #[at("/shoken-webapp-wasm/404")]
    NotFound,
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
        Route::Receipts => html! { <Receipts /> },
        Route::Search => html! { <Search /> },
        Route::NotFound => html! { <h1>{ "404 - Page not found" }</h1> },
    }
}

#[function_component]
pub fn App() -> Html {
    use_effect(|| {
        if let Some(window) = web_sys::window() {
            let location = window.location();
            let pathname = location.pathname().unwrap_or_default();
            if pathname != "/shoken-webapp-wasm/" {
                let history = window.history().unwrap();
                let _ = history.replace_state_with_url(&JsValue::NULL, "", Some(&pathname));
            }
        }
        || ()
    });

    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}
