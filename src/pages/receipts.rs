use crate::components::Layout;
use yew::prelude::*;

#[function_component(Receipts)]
pub fn receipts() -> Html {
    html! {
        <Layout>
            <h2 class="mb-4">{ "受取金" }</h2>
            <p class="lead">{ "ここで受取金を管理できます。" }</p>

            <div class="collapse navbar-collapse" id="navbarNav">
                <ul class="navbar-nav">
                    <li class="nav-item">
                        <button class="navbar-toggler" type="button" data-bs-toggle="collapse" data-bs-target="#navbarNav" aria-controls="navbarNav" aria-expanded="false" aria-label="Toggle navigation">
                            <span class="navbar-toggler-icon"></span>
                        </button>
                    </li>
                    <li class="nav-item">
                        <button class="navbar-toggler" type="button" data-bs-toggle="collapse" data-bs-target="#navbarNav" aria-controls="navbarNav" aria-expanded="false" aria-label="Toggle navigation">
                            <span class="navbar-toggler-icon"></span>
                        </button>
                    </li>
                </ul>
            </div>
        </Layout>
    }
}
