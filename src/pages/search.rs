use crate::components::Layout;
use web_sys::{HtmlInputElement, InputEvent};
use yew::prelude::*;

#[function_component(Search)]
pub fn search() -> Html {
    let search_input = use_state(|| String::new());
    let kabutan_link = use_state(|| String::new());

    let oninput = {
        let search_input = search_input.clone();
        let kabutan_link = kabutan_link.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let value = input.value();
            search_input.set(value.clone());
            let link = format!("https://kabutan.jp/stock/?code={}", value);
            kabutan_link.set(link);
        })
    };

    html! {
        <Layout>
            <h2 class="mb-4">{ "銘柄検索" }</h2>
            <div class="mb-3">
                <label for="stockCode" class="form-label">{ "銘柄コード" }</label>
                <input
                    type="text"
                    class="form-control"
                    id="stockCode"
                    placeholder="銘柄コードを入力"
                    value={(*search_input).clone()}
                    oninput={oninput}
                />
            </div>
            <div class="mt-3">
                <a href={(*kabutan_link).clone()} target="_blank" class="btn btn-primary">{ "かぶたんで確認する" }</a>
            </div>
        </Layout>
    }
}
