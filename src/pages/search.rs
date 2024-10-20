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
    pub code: Option<String>,
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

    let oninput = {
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
                match Request::get(&format!(
                    "https://shoken-webapp-api-b4a1.shuttle.app/stock/{value}"
                ))
                .send()
                .await
                {
                    Ok(response) => {
                        if response.ok() {
                            if let Ok(new_stock) = response.json::<Stock>().await {
                                stock.set(new_stock);
                            } else {
                                console::error_1(&JsValue::from_str("JSON parsing error."));
                            }
                        } else {
                            console::error_1(&JsValue::from_str(&format!(
                                "HTTP error: {} - {}",
                                response.status(),
                                response.status_text()
                            )));
                        }
                    }
                    Err(e) => {
                        console::error_1(&JsValue::from_str(&format!("Network error: {:?}", e)))
                    }
                }
            });
        })
    };

    html! {
        <Layout>
            <h2 class="mb-4 text-center">{ "銘柄検索" }</h2>
            <div class="mb-3">
                <input type="text" class="form-control" id="stockCode" placeholder="銘柄名・銘柄コードを入力" value={(*code_or_name).clone()} oninput={oninput} />
            </div>
            { render_stock_info(&stock) }
            { render_link(&stock) }
        </Layout>
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
                        <tr>
                            <th scope="row" width="160px">{ "銘柄名" }</th>
                            <td>{ &stock.name }</td>
                        </tr>
                        <tr>
                            <th scope="row">{ "銘柄コード" }</th>
                            <td>{ stock.code.clone().unwrap_or_default() }</td>
                        </tr>
                        <tr>
                            <th scope="row">{ "マーケットカテゴリ" }</th>
                            <td>{ &stock.market_category }</td>
                        </tr>
                        <tr>
                            <th scope="row">{ "33業種区分" }</th>
                            <td>{ stock.industry_category_33.clone().unwrap_or_default() }</td>
                        </tr>
                        <tr>
                            <th scope="row">{ "17業種区分" }</th>
                            <td>{ stock.industry_category_17.clone().unwrap_or_default() }</td>
                        </tr>
                        <tr>
                            <th scope="row">{ "規模区分" }</th>
                            <td>{ stock.size_category.clone().unwrap_or_default() }</td>
                        </tr>
                    </tbody>
                </table>
            </div>
        </div>
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
            { for links.iter().map(|(text, href, class)| {
                if let Some(code) = &stock.code {
                    html! {
                        <a
                            href={href.replace("{}", code)}
                            target="_blank"
                            class={format!("btn {class} me-2")}
                        >
                            { text }
                        </a>
                    }
                } else {
                    html! {
                        <span class={format!("btn {class} me-2 disabled")} aria-disabled="true">
                            { text }
                        </span>
                    }
                }
            }) }
        </div>
    }
}
