use crate::components::Layout;
use web_sys::InputEvent;
use yew::prelude::*;

pub struct Search {
    stock: String,
}

pub enum Msg {
    UpdateStock(String),
}

impl Component for Search {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            stock: "".to_string(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::UpdateStock(code) => {
                self.stock = code;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let oninput = ctx.link().callback(|e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            Msg::UpdateStock(input.value())
        });

        // 各ウェブサイトへのリンクを作成
        let kabutan_link = format!("https://kabutan.jp/stock/?code={}", self.stock);
        let yahoo_finance_link = format!("https://finance.yahoo.co.jp/quote/{}", self.stock);
        let nikkei_link = format!("https://www.nikkei.com/nkd/company/?scode={}", self.stock);
        let buffett_code_link = format!("https://www.buffett-code.com/company/{}", self.stock);
        let minkabu_link = format!("https://minkabu.jp/stock/{}/", self.stock);

        html! {
            <Layout>
                <h2 class="mb-4">{ "銘柄検索" }</h2>
                <div class="mb-3">
                    <input
                        type="text"
                        class="form-control"
                        id="stockCode"
                        placeholder="銘柄名・銘柄コードを入力"
                        value={self.stock.clone()}
                        oninput={oninput}
                    />
                </div>
                <div class="mt-3">
                    <a href={kabutan_link} target="_blank" class="btn btn-primary me-2">{ "かぶたんで確認する" }</a>
                    <a href={yahoo_finance_link} target="_blank" class="btn btn-secondary me-2">{ "Yahoo! Financeで確認する" }</a>
                    <a href={nikkei_link} target="_blank" class="btn btn-success me-2">{ "日経で確認する" }</a>
                    <a href={buffett_code_link} target="_blank" class="btn btn-warning me-2">{ "バフェットコードで確認する" }</a>
                    <a href={minkabu_link} target="_blank" class="btn btn-info">{ "みんかぶで確認する" }</a>
                </div>
            </Layout>
        }
    }
}
