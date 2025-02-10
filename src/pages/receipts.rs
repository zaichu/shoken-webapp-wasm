use strum::EnumMessage;
use yew::prelude::*;

mod dividend_list;
mod domestic_stock;
mod mutual_fund;
mod receipt_template;

use dividend_list::DividendList;
use domestic_stock::DomesticStock;
use mutual_fund::MutualFund;
use receipt_template::ReceiptTemplate;

use super::layout::Layout;

#[derive(Clone, PartialEq, Eq, Debug, EnumMessage, Copy)]
enum ReceiptsType {
    #[strum(message = "配当金")]
    Dividend,

    #[strum(message = "国内株式")]
    DomesticStock,

    #[strum(message = "投資信託")]
    MutualFund,
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
            <nav class="nav nav-tabs">
                <ul class="nav nav-tabs">
                    {render_nav_item(&selected_type, ReceiptsType::Dividend, &on_click)}
                    {render_nav_item(&selected_type, ReceiptsType::DomesticStock, &on_click)}
                    {render_nav_item(&selected_type, ReceiptsType::MutualFund, &on_click)}
                </ul>
            </nav>
            <div class="mt-4"> {
                match *selected_type {
                    ReceiptsType::Dividend =>      html! { <ReceiptTemplate::<DividendList>  name={ name } /> },
                    ReceiptsType::DomesticStock => html! { <ReceiptTemplate::<DomesticStock> name={ name } /> },
                    ReceiptsType::MutualFund =>    html! { <ReceiptTemplate::<MutualFund> name={ name } /> },
                }}
            </div>
        </Layout>
    }
}

fn render_nav_item(
    selected_type: &ReceiptsType,
    item_type: ReceiptsType,
    on_click: &Callback<ReceiptsType>,
) -> Html {
    html! {
        <li class="nav-item">
            <button
                class={if *selected_type == item_type {"nav-link active"} else {"nav-link"}}
                onclick={on_click.reform(move |_| item_type)}
            >
                { item_type.get_message() }
            </button>
        </li>
    }
}
