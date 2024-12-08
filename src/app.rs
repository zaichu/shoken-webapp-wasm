use crate::{
    components::UserInfo,
    pages::{Home, Receipts, Search},
};
use url::Url;
use wasm_bindgen::JsValue;
use web_sys::console;
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
    let user_info = use_state(|| {
        web_sys::window()
            .and_then(|window| {
                window
                    .location()
                    .search()
                    .ok()
                    .and_then(|search| {
                        Url::parse(&format!(
                            "http://localhost:8080/shoken-webapp-wasm/{}",
                            &search
                        ))
                        .ok()
                        .and_then(|url| {
                            url.query_pairs().find(|(key, _)| key == "code").map(
                                |(_, auth_code)| UserInfo {
                                    auth_code: auth_code.to_string(),
                                    ..Default::default()
                                },
                            )
                        })
                    })
                    .or_else(|| {
                        window
                            .local_storage()
                            .ok()
                            .flatten()
                            .and_then(|storage| storage.get_item("user_info").ok().flatten())
                            .and_then(|data| serde_json::from_str(&data).ok())
                    })
            })
            .unwrap_or_default()
    });

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

    console::log_1(&JsValue::from_str(&format!("{:?}", user_info)));

    let update_user_info = {
        let user_info = user_info.clone();
        Callback::from(move |new_info: UserInfo| {
            let json = serde_json::to_string(&new_info).unwrap();
            let storage = web_sys::window().unwrap().local_storage().unwrap().unwrap();
            storage.set_item("user_info", &json).unwrap();
            user_info.set(new_info);
        })
    };

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
