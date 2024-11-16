use strum::EnumMessage;
use yew::prelude::*;

use crate::components::{
    receipts::{dividend_list::DividendList, lib::ReceiptTemplate, profit_and_loss::ProfitAndLoss},
    Layout,
};

#[derive(Clone, PartialEq, Eq, Debug, EnumMessage)]
pub enum ReceiptsType {
    #[strum(message = "配当金")]
    Dividend,

    #[strum(message = "実現損益")]
    ProfitAndLoss,
}

#[function_component]
pub fn Receipts() -> Html {
    let selected_type = use_state(|| ReceiptsType::Dividend);

    let on_click = {
        let selected_type = selected_type.clone();
        Callback::from(move |new_type: ReceiptsType| {
            selected_type.set(new_type);
        })
    };

    let name = selected_type.get_message().unwrap();
    html! {
        <Layout>
            <nav class="navbar navbar-expand-lg navbar-light bg-light">
                <div class="container" style="max-width: 1600px;">
                    <div class="collapse navbar-collapse" id="navbarNav">
                        <ul class="navbar-nav">
                            <li class="nav-item">
                                <button class="nav-link" onclick={on_click.reform(|_| ReceiptsType::Dividend)}>{ ReceiptsType::Dividend.get_message() }</button>
                            </li>
                            <li class="nav-item">
                                <button class="nav-link" onclick={on_click.reform(|_| ReceiptsType::ProfitAndLoss)}>{ ReceiptsType::ProfitAndLoss.get_message() }</button>
                            </li>
                        </ul>
                    </div>
                </div>
            </nav>
            <div class="mt-4"> {
                match *selected_type {
                    ReceiptsType::Dividend =>      html! { <ReceiptTemplate::<DividendList>  name={ name } /> },
                    ReceiptsType::ProfitAndLoss => html! { <ReceiptTemplate::<ProfitAndLoss> name={ name } /> },
                }}
            </div>
        </Layout>
    }
}
