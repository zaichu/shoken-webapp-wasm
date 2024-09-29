use html::ChildrenProps;
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
    #[at("/404")]
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
fn layout(props: &ChildrenProps) -> Html {
    html! {
        <div class="app-container">
            <header>
                <h1><Link<Route> to={Route::Home} classes="logo">{ "証券Web" }</Link<Route>></h1>
                <nav>
                    <Link<Route> to={Route::Search} classes="nav-link">{ "銘柄検索" }</Link<Route>>
                    <Link<Route> to={Route::Receipts} classes="nav-link">{ "受取金" }</Link<Route>>
                </nav>
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
        </div>
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
        body {
            font-family: Arial, sans-serif;
            line-height: 1.6;
            color: #333;
            margin: 0;
            padding: 0;
            background-color: #f4f4f4;
        }
        .app-container {
            margin: 0 auto;
            padding: 1rem;
        }
        header {
            background-color: #2c3e50;
            color: #ecf0f1;
            padding: 1rem;
            display: flex;
            justify-content: space-between;
            align-items: center;
        }
        .logo {
            color: #ecf0f1;
            text-decoration: none;
            font-size: 1.5rem;
            font-weight: bold;
        }
        nav {
            display: flex;
            gap: 1rem;
        }
        .nav-link {
            color: #ecf0f1;
            text-decoration: none;
            padding: 0.5rem 1rem;
            border-radius: 4px;
            transition: background-color 0.3s;
        }
        .nav-link:hover {
            background-color: #34495e;
        }
        main {
            background-color: #fff;
            padding: 2rem;
            margin-top: 2rem;
            border-radius: 8px;
            box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
        }
        h2 {
            color: #2c3e50;
        }
        footer {
            text-align: center;
            margin-top: 2rem;
            padding: 1rem;
            background-color: #2c3e50;
            color: #ecf0f1;
        }
        "#,
    )
}
