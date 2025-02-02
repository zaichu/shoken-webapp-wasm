use gloo::console;
use gloo_net::http::Request;
use serde::{Deserialize, Serialize};
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::{components::Layout, env, setting::STOCK_INFO_LINKS};

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

#[function_component]
pub fn Search() -> Html {
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

            console::log!(format!("Input value: {value}"));

            let stock = stock.clone();
            spawn_local(async move {
                match fetch_stock_data(&value).await {
                    Ok(new_stock) => stock.set(new_stock),
                    Err(err) => console::log!(&err.to_string()),
                }
            });
        })
    };

    html! {
        <Layout>
            <div class="mb-3">
                <input
                    type="text"
                    class="form-control form-control-lg shadow-sm"
                    id="stockCode"
                    placeholder="銘柄名・銘柄コードを入力"
                    value={(*code_or_name).clone()}
                    oninput={on_input}
                />
            </div>
            <div class="card shadow-sm">
                <div class="card-header bg-primary text-white">
                    <h5 class="mb-0">{ "検索結果" }</h5>
                </div>
                <div class="card-body">
                    <table class="table">
                        <tbody>
                            { render_table_row("銘柄名", &stock.name) }
                            { render_table_row("銘柄コード", &stock.code) }
                            { render_table_row("マーケットカテゴリ", &stock.market_category) }
                            { render_table_row("33業種区分", &stock.industry_category_33.clone().unwrap_or_default()) }
                            { render_table_row("17業種区分", &stock.industry_category_17.clone().unwrap_or_default()) }
                            { render_table_row("規模区分", &stock.size_category.clone().unwrap_or_default()) }
                        </tbody>
                    </table>
                    <div class="d-flex flex-wrap">
                        { for STOCK_INFO_LINKS.iter().enumerate().map(|(i, (text, href))| {
                            html! {
                                <>
                                    <a href={href.replace("{}", &stock.code)} target="_blank" class="fw-bold">
                                        { text }
                                    </a>
                                    { if i < STOCK_INFO_LINKS.len() - 1 { html! { <span class="mx-2">{"|"} </span> } } else { html! {} } }
                                </>
                            }
                        })}
                    </div>
                </div>
            </div>
        </Layout>
    }
}

async fn fetch_stock_data(value: &str) -> Result<Stock, String> {
    // let url = format!("{}{}", env::SHOKEN_WEBAPI_STOCK, value);
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

fn render_table_row(label: &str, value: &str) -> Html {
    html! {
        <tr>
            <th scope="row" width="125px">{ label }</th>
            <td>{ value }</td>
        </tr>
    }
}
