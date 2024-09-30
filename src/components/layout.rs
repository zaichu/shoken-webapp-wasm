use crate::routes::Route;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(Layout)]
pub fn layout(props: &yew::html::ChildrenProps) -> Html {
    html! {
        <>
            <nav class="navbar navbar-expand-lg navbar-light bg-light">
                <div class="container">
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
            <main class="container mt-4">
                { for props.children.iter() }
            </main>
            <footer class="bg-light text-center text-lg-start mt-4">
                <div class="container p-1">
                    <p class="text-center mt-3">{ "© 2024 証券Web" }</p>
                </div>
            </footer>
        </>
    }
}
