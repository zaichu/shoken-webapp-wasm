use crate::routes::Route;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(Layout)]
pub fn layout(props: &yew::html::ChildrenProps) -> Html {
    html! {
        <>
            <header class="header">
                <div class="container">
                    <div class="logo">
                        <Link<Route> to={Route::Home}>{ "証券Web" }</Link<Route>>
                    </div>
                    <nav>
                        <ul>
                            <li><Link<Route> to={Route::Search} classes="nav-link">{ "銘柄検索" }</Link<Route>></li>
                            <li><Link<Route> to={Route::Receipts} classes="nav-link">{ "受取金" }</Link<Route>></li>
                        </ul>
                    </nav>
                </div>
            </header>
            <main class="container">
                <div class="content">
                    { for props.children.iter() }
                </div>
            </main>
            <footer>
                <div class="container">
                    <p>{ "© 2024 証券Web" }</p>
                </div>
            </footer>
            <style>
                { get_styles() }
            </style>
        </>
    }
}

fn get_styles() -> String {
    String::from(
        r#"
        :root {
            --primary-color: #3498db;
            --secondary-color: #2c3e50;
            --background-color: #ecf0f1;
            --text-color: #34495e;
        }
        * {
            box-sizing: border-box;
            margin: 0;
            padding: 0;
        }
        body {
            font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
            line-height: 1.6;
            color: var(--text-color);
            background-color: var(--background-color);
        }
        .container {
            max-width: 1600px;
            margin: 0 auto;
        }
        .header {
            background-color: #fff;
            box-shadow: 0 2px 4px rgba(0,0,0,0.1);
            position: fixed;
            width: 100%;
            z-index: 1000;
        }
        .header .container {
            display: flex;
            justify-content: space-between;
            align-items: center;
            padding: 1rem 2rem;
        }
        .logo a {
            color: var(--primary-color);
            text-decoration: none;
            font-size: 1.5rem;
            font-weight: bold;
        }
        nav ul {
            display: flex;
            list-style: none;
        }
        .nav-link {
            color: var(--secondary-color);
            text-decoration: none;
            padding: 0.5rem 1rem;
            border-radius: 4px;
            transition: background-color 0.3s ease;
        }
        .nav-link:hover {
            background-color: var(--primary-color);
            color: #fff;
        }
        main {
            padding-top: 5rem;
        }
        .content {
            background-color: #fff;
            border-radius: 8px;
            box-shadow: 0 2px 4px rgba(0,0,0,0.1);
            padding: 2rem;
        }
        h2 {
            color: var(--primary-color);
            margin-bottom: 1rem;
        }
        footer {
            background-color: #fff;
            text-align: center;
            padding: 1rem 0;
            margin-top: 0.5rem;
        }
        "#,
    )
}
