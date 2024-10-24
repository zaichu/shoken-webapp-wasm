use yew::prelude::*;

use crate::components::receipts::templete::Template;

#[function_component(ProfitAndLoss)]
pub fn profit_and_loss() -> Html {
    html! {
        <>
            <p>{ "実益損益のデータを表示します。" }</p>
            <Template />
        </>
    }
}
