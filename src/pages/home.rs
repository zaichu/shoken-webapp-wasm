use crate::components::Layout;
use yew::prelude::*;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <Layout>
            <h2>{ "ホーム" }</h2>
            <p>{ "証券Webへようこそ。このプラットフォームでは、銘柄検索や受取金の管理ができます。" }</p>
        </Layout>
    }
}
