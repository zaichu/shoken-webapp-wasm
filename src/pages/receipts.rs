use crate::components::Layout;
use yew::prelude::*;

#[function_component(Receipts)]
pub fn receipts() -> Html {
    html! {
        <Layout>
            <h2 class="mb-4">{ "受取金" }</h2>
            <p class="lead">{ "ここで受取金を管理できます。" }</p>
            // 受取金の管理機能をここに追加
        </Layout>
    }
}
