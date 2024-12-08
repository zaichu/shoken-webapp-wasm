use crate::{
    components::UserInfo,
    pages::{Home, Receipts, Search},
};
use gloo::console;
use url::Url;
use wasm_bindgen::JsValue;
use web_sys::window;
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
    let user_info = use_state(|| initialize_user_info());

    use_effect(|| {
        update_browser_history();
        || ()
    });

    console::log!(format!("user_info: {:?}", user_info));

    let update_user_info = create_update_user_info_callback(user_info.clone());

    html! {
        <ContextProvider<UserInfo> context={(*user_info).clone()}>
            <ContextProvider<Callback<UserInfo>> context={update_user_info}>
                <BrowserRouter>
                    <Switch<Route> render={switch} />
                </BrowserRouter>
            </ContextProvider<Callback<UserInfo>>>
        </ContextProvider<UserInfo>>
    }
}

fn initialize_user_info() -> UserInfo {
    window()
        .and_then(|window| {
            get_user_info_from_url(&window).or_else(|| get_user_info_from_storage(&window))
        })
        .unwrap_or_default()
}

fn get_user_info_from_url(window: &web_sys::Window) -> Option<UserInfo> {
    window.location().search().ok().and_then(|search| {
        Url::parse(&format!(
            "http://localhost:8080/shoken-webapp-wasm/{}",
            &search
        ))
        .ok()
        .and_then(|url| {
            url.query_pairs()
                .find(|(key, _)| key == "code")
                .map(|(_, auth_code)| UserInfo {
                    auth_code: auth_code.to_string(),
                    ..Default::default()
                })
        })
    })
}

fn get_user_info_from_storage(window: &web_sys::Window) -> Option<UserInfo> {
    window
        .local_storage()
        .ok()
        .flatten()
        .and_then(|storage| storage.get_item("user_info").ok().flatten())
        .and_then(|data| serde_json::from_str(&data).ok())
}

fn update_browser_history() {
    if let Some(window) = window() {
        let location = window.location();
        let pathname = location.pathname().unwrap_or_default();
        if pathname != "/shoken-webapp-wasm/" {
            if let Ok(history) = window.history() {
                let _ = history.replace_state_with_url(&JsValue::NULL, "", Some(&pathname));
            }
        }
    }
}

fn create_update_user_info_callback(user_info: UseStateHandle<UserInfo>) -> Callback<UserInfo> {
    Callback::from(move |new_info: UserInfo| {
        if let Ok(json) = serde_json::to_string(&new_info) {
            if let Some(storage) = window().and_then(|w| w.local_storage().ok()).flatten() {
                let _ = storage.set_item("user_info", &json);
            }
        }
        user_info.set(new_info);
    })
}
