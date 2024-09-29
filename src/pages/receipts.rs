use crate::components::Layout;
use yew::prelude::*;

#[function_component(Receipts)]
pub fn receipts() -> Html {
    html! {
        <Layout>
            <h2>{ "受取金" }</h2>
            <p>{ "ここで受取金を管理できます。" }</p>
        </Layout>
    }
}
