use gloo::console;
use url::Url;
use wasm_bindgen::JsValue;
use web_sys::{window, Window};
use yew::prelude::*;
use yew_router::prelude::*;

use crate::{
    data::use_info::UserInfo,
    errors::{AppError, AppResult},
    pages::{home::Home, receipts::Receipts, search::Search},
};

#[derive(Clone, Routable, PartialEq, Debug)]
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
    let user_info = use_state(initialize_user_info);

    #[cfg(debug_assertions)]
    console::log!(format!("user_info: {:?}", user_info));

    use_effect(update_browser_history);

    html! {
        <ContextProvider<UserInfo> context={(*user_info).clone()}>
            <BrowserRouter>
                <Switch<Route> render={switch} />
            </BrowserRouter>
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

fn get_user_info_from_url(window: &Window) -> Option<UserInfo> {
    let search = window.location().search().ok()?;

    if search.is_empty() {
        return None;
    }

    let base_url = format!("http://localhost:8080/shoken-webapp-wasm/{}", &search);

    Url::parse(&base_url).ok().and_then(|url| {
        url.query_pairs()
            .find(|(key, _)| key == "code")
            .map(|(_, auth_code)| {
                let new_info = UserInfo {
                    auth_code: Some(auth_code.to_string()),
                    ..Default::default()
                };
                let _ = save_user_info_to_storage(&new_info);
                new_info
            })
    })
}

fn get_user_info_from_storage(window: &Window) -> Option<UserInfo> {
    window
        .local_storage()
        .ok()
        .flatten()
        .and_then(|storage| storage.get_item("user_info").ok().flatten())
        .and_then(|data| serde_json::from_str(&data).ok())
}

fn save_user_info_to_storage(user_info: &UserInfo) -> AppResult<()> {
    let json = serde_json::to_string(user_info).map_err(|e| {
        AppError::ParseError(format!("ユーザー情報のシリアライズに失敗しました: {}", e))
    })?;

    let window = window().ok_or_else(|| {
        AppError::UnknownError("ウィンドウオブジェクトが見つかりません".to_string())
    })?;

    let storage = window
        .local_storage()
        .map_err(|_| AppError::UnknownError("ローカルストレージにアクセスできません".to_string()))?
        .ok_or_else(|| AppError::UnknownError("ローカルストレージが利用できません".to_string()))?;

    storage
        .set_item("user_info", &json)
        .map_err(|_| AppError::UnknownError("ユーザー情報の保存に失敗しました".to_string()))?;

    Ok(())
}

fn update_browser_history() {
    window().and_then(|w| {
        w.location().pathname().ok().and_then(|pathname| {
            if pathname != "/shoken-webapp-wasm/" {
                w.history().ok().and_then(|history| {
                    history
                        .replace_state_with_url(&JsValue::NULL, "", Some(&pathname))
                        .ok()
                })
            } else {
                None
            }
        })
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_routes() {
        let home_route = Route::Home;
        let receipts_route = Route::Receipts;
        let search_route = Route::Search;

        assert_ne!(home_route, receipts_route);
        assert_ne!(home_route, search_route);
        assert_ne!(receipts_route, search_route);
    }
}
