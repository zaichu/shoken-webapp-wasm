use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
enum Route {
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

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
        Route::Receipts => html! { <Receipts /> },
        Route::Search => html! { <Search /> },
        Route::NotFound => html! { <h1>{ "404 - Page not found" }</h1> },
    }
}

#[function_component(Home)]
fn home() -> Html {
    html! {
        <Layout>
            <h2>{ "ホーム" }</h2>
            <p>{ "証券Webへようこそ。このプラットフォームでは、銘柄検索や受取金の管理ができます。" }</p>
        </Layout>
    }
}

#[function_component(Search)]
fn search() -> Html {
    html! {
        <Layout>
            <h2>{ "銘柄検索" }</h2>
            <p>{ "ここで銘柄を検索できます。" }</p>
        </Layout>
    }
}

#[function_component(Receipts)]
fn receipts() -> Html {
    html! {
        <Layout>
            <h2>{ "受取金" }</h2>
            <p>{ "ここで受取金を管理できます。" }</p>
        </Layout>
    }
}

#[function_component(Layout)]
fn layout(props: &yew::html::ChildrenProps) -> Html {
    html! {
        <>
            <header class="header">
                <div class="header-content">
                    <div class="logo">
                        <Link<Route> to={Route::Home}>{ "証券Web" }</Link<Route>>
                    </div>
                    <nav>
                        <ul>
                            <li><Link<Route> to={Route::Search}>{ "銘柄検索" }</Link<Route>></li>
                            <li><Link<Route> to={Route::Receipts}>{ "受取金" }</Link<Route>></li>
                        </ul>
                    </nav>
                </div>
            </header>
            <main>
                { for props.children.iter() }
            </main>
            <footer>
                <p>{ "© 2024 証券Web" }</p>
            </footer>
            <style>
                { get_styles() }
            </style>
        </>
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    yew::Renderer::<App>::new().render();
}

fn get_styles() -> String {
    String::from(
        r#"
        * {
            box-sizing: border-box;
            margin: 0;
            padding: 0;
        }
        body {
            font-family: Arial, sans-serif;
            line-height: 1.6;
            color: #333;
        }
        .header {
            background-color: #fff;
            box-shadow: 1px 1px 4px 0 rgba(0,0,0,.1);
            position: fixed;
            width: 100%;
            z-index: 3;
        }
        .header-content {
            max-width: 1200px;
            margin: 0 auto;
            display: flex;
            justify-content: space-between;
            align-items: center;
            padding: 1rem;
        }
        .logo a {
            color: #000;
            text-decoration: none;
            font-size: 1.5rem;
            font-weight: bold;
        }
        .header ul {
            margin: 0;
            padding: 0;
            list-style: none;
            overflow: hidden;
            background-color: #fff;
        }
        .header li {
            display: inline-block;
            margin-left: 1rem;
        }
        .header li a {
            display: block;
            padding: 10px;
            text-decoration: none;
            color: #000;
        }
        .header li a:hover,
        .header .menu-btn:hover {
            background-color: #f4f4f4;
        }
        main {
            padding: 5rem 2rem 2rem;
            max-width: 1200px;
            margin: 0 auto;
        }
        "#,
    )
}
