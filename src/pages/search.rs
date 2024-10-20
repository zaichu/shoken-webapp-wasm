use gloo_net::http::Request;
use serde::{Deserialize, Serialize};
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::spawn_local;
use web_sys::{console, HtmlInputElement};
use yew::prelude::*;

use crate::components::Layout;

#[derive(Clone, PartialEq, Deserialize, Serialize)]
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
    let code = use_state(String::new);

    let onblur = {
        let code = code.clone();

        Callback::from(move |e: FocusEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let code_or_name = input.value();

            console::log_1(&JsValue::from_str(&format!("Input value: {code_or_name}")));

            let code = code.clone();
            spawn_local(async move {
                match Request::get(&format!(
                    "https://shoken-webapp-api-b4a1.shuttle.app/stock/{code_or_name}"
                ))
                .send()
                .await
                {
                    Ok(response) => {
                        if response.ok() {
                            match response.json::<Stock>().await {
                                Ok(stock) => code.set(stock.code),
                                Err(e) => {
                                    console::error_1(&JsValue::from_str(&format!(
                                        "JSON parsing error: {:?}",
                                        e
                                    )));
                                }
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
                        console::error_1(&JsValue::from_str(&format!("Network error: {:?}", e)));
                    }
                }
            });
        });
    };

    html! {
        <Layout>
            <h2 class="mb-4">{ "銘柄検索" }</h2>
            <div class="mb-3">
                <input type="text" class="form-control" id="stockCode" placeholder="銘柄名・銘柄コードを入力" value={(*code).clone()} onblur={onblur} />
            </div>
            { render_link(&code) }
        </Layout>
    }
}

fn render_link(code: &UseStateHandle<String>) -> Html {
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
            { for links.into_iter().map(|(text, href, class)| {
                html! {
                    <a
                        href={href.replace("{}", code)}
                        class={format!("btn {class} me-2")}
                        target="_blank"
                    >
                        { text }
                    </a>
                }
            })}
        </div>
    }
}
