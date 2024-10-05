use reqwest;
use select::document::Document;
use select::predicate::Name;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::spawn_local;
use web_sys::{console, window, HtmlInputElement};
use yew::prelude::*;

use crate::components::Layout;

#[function_component(Search)]
pub fn search() -> Html {
    let stock = use_state(String::new);

    let oninput = {
        let stock = stock.clone();

        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let value = input.value();
            stock.set(value.clone());

            console::log_1(&JsValue::from_str(&format!("Input value: {}", value)));
        })
    };

    html! {
        <Layout>
            <h2 class="mb-4">{ "銘柄検索" }</h2>
            <div class="mb-3">
                <input type="text" class="form-control" id="stockCode" placeholder="銘柄名・銘柄コードを入力" value={(*stock).clone()} oninput={oninput} />
            </div>
            { render_link(&stock) }
        </Layout>
    }
}

fn render_link(stock_code: &UseStateHandle<String>) -> Html {
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
                let stock = stock_code.clone();
                html! {
                    <button onclick={Callback::from(move |_| {
                        let stock = stock.clone();
                        spawn_local(async move {
                            if let Some(window) = window() {
                                let url = format!("https://www.buffett-code.com/company/search?keyword={}", urlencoding::encode(&stock));
                                let client = reqwest::Client::new();
                                let stock_code = match client.get(&url)
                                    .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36")
                                    .send()
                                    .await {
                                    Ok(res) => {
                                        if let Ok(body) = res.text().await {
                                            let document = Document::from(body.as_str());
                                            document.find(Name("body"))
                                                .next()
                                                .and_then(|body| body.find(Name("div")).nth(0))
                                                .and_then(|div| div.find(Name("div")).nth(0))
                                                .and_then(|div| div.find(Name("main")).next())
                                                .and_then(|main| main.find(Name("div")).nth(2))
                                                .and_then(|div| div.find(Name("div")).next())
                                                .and_then(|div| div.find(Name("div")).nth(1))
                                                .and_then(|div| div.find(Name("div")).nth(1))
                                                .and_then(|div| div.find(Name("div")).next())
                                                .and_then(|div| div.find(Name("div")).next())
                                                .and_then(|div| div.find(Name("p")).next())
                                                .and_then(|p| p.find(Name("span")).next())
                                                .map(|span| span.text())
                                                .unwrap_or_else(|| stock.to_string())
                                        } else {
                                            stock.to_string()
                                        }
                                    },
                                    Err(_) => stock.to_string(),
                                };
                                let url = href.replace("{}", &stock_code);
                                let _ = window.open_with_url_and_target(&url, "_blank");
                            }
                        });
                    })} class={format!("btn {class} me-2")}>
                        { text }
                    </button>
                }
            })}
        </div>
    }
}
