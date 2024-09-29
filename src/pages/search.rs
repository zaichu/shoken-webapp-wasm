use crate::components::Layout;
use yew::prelude::*;

#[function_component(Search)]
pub fn search() -> Html {
    html! {
        <Layout>
            <h2>{ "銘柄検索" }</h2>
            <p>{ "ここで銘柄を検索できます。" }</p>
        </Layout>
    }
}
