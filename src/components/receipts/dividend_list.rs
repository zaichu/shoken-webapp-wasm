use yew::prelude::*;

use crate::components::receipts::templete::Template;

#[function_component(DividendList)]
pub fn dividend_list() -> Html {
    html! {
        <>
            <p>{ "配当金のデータを表示します。" }</p>
            <Template />
        </>
    }
}
