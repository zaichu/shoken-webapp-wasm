use gloo::console;
use gloo_net::http::Request;
use web_sys::window;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::{app::Route, env};

#[function_component]
pub fn Layout(props: &yew::html::ChildrenProps) -> Html {
    let on_click = on_login_callback();
    html! {
        <>
            <div class="mb-3">
                <button onclick={on_click}>
                    { "Googleでログイン" }
                </button>
            </div>
            <nav class="navbar navbar-expand-lg navbar-light bg-light">
                <div class="container" style="max-width: 1600px;">
                    <Link<Route> classes="navbar-brand" to={Route::Home}>{ "証券Web" }</Link<Route>>
                    <button class="navbar-toggler" type="button" data-bs-toggle="collapse" data-bs-target="#navbarNav" aria-controls="navbarNav" aria-expanded="false" aria-label="Toggle navigation">
                        <span class="navbar-toggler-icon"></span>
                    </button>
                    <div class="collapse navbar-collapse" id="navbarNav">
                        <ul class="navbar-nav">
                            <li class="nav-item">
                                <Link<Route> classes="nav-link" to={Route::Search}>{ "銘柄検索" }</Link<Route>>
                            </li>
                            <li class="nav-item">
                                <Link<Route> classes="nav-link" to={Route::Receipts}>{ "受取金" }</Link<Route>>
                            </li>
                        </ul>
                    </div>
                </div>
            </nav>
            <main class="container mt-4" style="max-width: 1600px;">
                { props.children.clone() }
            </main>
            <footer class="bg-light text-center text-lg-start mt-4">
                <div class="container p-1">
                    <p class="text-center mt-3">{ "© 2024 証券Web" }</p>
                </div>
            </footer>
        </>
    }
}

fn on_login_callback() -> Callback<MouseEvent> {
    Callback::from(move |_: MouseEvent| {
        let future = async {
            match Request::get(&env::SHOKEN_WEBAPI_OAUTH_GOOGLE).send().await {
                Ok(response) => match response.json::<String>().await {
                    Ok(auth_url) => {
                        let window = window().unwrap();
                        window.location().set_href(&auth_url).unwrap();
                    }
                    Err(err) => console::log!(&err.to_string()),
                },
                Err(err) => console::log!(&err.to_string()),
            }
        };
        yew::platform::spawn_local(future);
    })
}
