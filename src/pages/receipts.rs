use yew::prelude::*;

use crate::components::{
    receipts::{dividend_list::DividendList, profit_and_loss::ProfitAndLoss},
    Layout,
};

#[derive(Clone, PartialEq)]
pub enum ReceiptsType {
    Dividend,
    ProfitAndLoss,
}

#[function_component(Receipts)]
pub fn receipts() -> Html {
    let selected_type = use_state(|| ReceiptsType::Dividend);

    let on_click = {
        let selected_type = selected_type.clone();
        Callback::from(move |new_type: ReceiptsType| {
            selected_type.set(new_type);
        })
    };

    html! {
        <Layout>
            <h2 class="mb-4 text-center">{ "受取金" }</h2>
            <nav class="navbar navbar-expand-lg navbar-light bg-light">
                <div class="container">
                    <div class="collapse navbar-collapse" id="navbarNav">
                        <ul class="navbar-nav">
                            <li class="nav-item">
                                <button class="nav-link" onclick={on_click.reform(|_| ReceiptsType::Dividend)}>{ "配当金" }</button>
                            </li>
                            <li class="nav-item">
                                <button class="nav-link" onclick={on_click.reform(|_| ReceiptsType::ProfitAndLoss)}>{ "実益損益" }</button>
                            </li>
                        </ul>
                    </div>
                </div>
            </nav>
            <div class="mt-4">
                { match (*selected_type).clone() {
                    ReceiptsType::Dividend => html! { <DividendList /> } ,
                    ReceiptsType::ProfitAndLoss => html! { <ProfitAndLoss /> },
                }}
            </div>
        </Layout>
    }
}
