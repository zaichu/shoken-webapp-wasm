use gloo_net::http::Request;
use serde::{Deserialize, Serialize};
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::spawn_local;
use web_sys::{console, HtmlInputElement};
use yew::prelude::*;

use crate::components::Layout;

#[derive(Clone, PartialEq, Deserialize, Serialize, Default)]
struct Stock {
    pub date: String,
    pub code: String,
    pub name: String,
    pub market_category: String,
    pub industry_code_33: Option<String>,
    pub industry_category_33: Option<String>,
    pub industry_code_17: Option<String>,
    pub industry_category_17: Option<String>,
    pub size_code: Option<String>,
    pub size_category: Option<String>,
}

#[function_component(Search)]
pub fn search() -> Html {
    let stock = use_state(Stock::default);
    let code_or_name = use_state(String::new);

    let on_input = {
        let stock = stock.clone();
        let code_or_name = code_or_name.clone();

        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let value = input.value();
            code_or_name.set(value.clone());
            stock.set(Stock::default());

            console::log_1(&JsValue::from_str(&format!("Input value: {value}")));

            let stock = stock.clone();
            spawn_local(async move {
                match fetch_stock_data(&value).await {
                    Ok(new_stock) => stock.set(new_stock),
                    Err(err) => console::log_1(&JsValue::from_str(&err)),
                }
            });
        })
    };

    html! {
        <Layout>
            <h2 class="mb-4 text-center">{ "銘柄検索" }</h2>
            <div class="mb-3">
                <input
                    type="text"
                    class="form-control"
                    id="stockCode"
                    placeholder="銘柄名・銘柄コードを入力"
                    value={(*code_or_name).clone()}
                    oninput={on_input}
                />
            </div>
            { render_stock_info(&stock) }
            { render_link(&stock) }
        </Layout>
    }
}

async fn fetch_stock_data(value: &str) -> Result<Stock, String> {
    let url = format!("https://shoken-webapp-api-b4a1.shuttle.app/stock/{}", value);
    let response = Request::get(&url).send().await.map_err(|e| e.to_string())?;

    if response.ok() {
        response.json::<Stock>().await.map_err(|e| e.to_string())
    } else {
        Err(format!(
            "HTTP error: {} - {}",
            response.status(),
            response.status_text()
        ))
    }
}

fn render_stock_info(stock: &UseStateHandle<Stock>) -> Html {
    html! {
        <div class="card mt-4">
            <div class="card-header bg-primary text-white">
                { "検索結果" }
            </div>
            <div class="card-body">
                <table class="table table-sm">
                    <tbody>
                        { render_table_row("銘柄名", &stock.name) }
                        { render_table_row("銘柄コード", &stock.code) }
                        { render_table_row("マーケットカテゴリ", &stock.market_category) }
                        { render_table_row("33業種区分", &stock.industry_category_33.clone().unwrap_or_default()) }
                        { render_table_row("17業種区分", &stock.industry_category_17.clone().unwrap_or_default()) }
                        { render_table_row("規模区分", &stock.size_category.clone().unwrap_or_default()) }
                    </tbody>
                </table>
            </div>
        </div>
    }
}

fn render_table_row(label: &str, value: &str) -> Html {
    html! {
        <tr>
            <th scope="row" width="160px">{ label }</th>
            <td>{ value }</td>
        </tr>
    }
}

fn render_link(stock: &UseStateHandle<Stock>) -> Html {
    let links = vec![
        (
            "かぶたん",
            "https://kabutan.jp/stock/?code={}",
            "btn-primary",
        ),
        (
            "Yahoo! Finance",
            "https://finance.yahoo.co.jp/quote/{}",
            "btn-secondary",
        ),
        (
            "日経",
            "https://www.nikkei.com/nkd/company/?scode={}",
            "btn-success",
        ),
        (
            "バフェットコード",
            "https://www.buffett-code.com/company/{}",
            "btn-warning",
        ),
        ("みんかぶ", "https://minkabu.jp/stock/{}/", "btn-info"),
    ];

    html! {
        <div class="mt-3">
            { for links.iter().map(|(text, href, class)| render_link_button(text, href, class, &stock.code)) }
        </div>
    }
}

fn render_link_button(text: &str, href: &str, class: &str, code: &str) -> Html {
    html! {
        <a
            href={href.replace("{}", code)}
            target="_blank"
            class={format!("btn {} me-2", class)}
        >
            { text }
        </a>
    }
}
