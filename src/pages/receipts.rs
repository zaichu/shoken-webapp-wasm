use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::components::Layout;

#[derive(Clone, PartialEq)]
pub enum ReceiptsType {
    Dividend,
    ProfitAndLoss,
}

#[function_component(Receipts)]
pub fn receipts() -> Html {
    let selected_type = use_state(|| ReceiptsType::Dividend);
    let csv_file = use_state(|| String::new());

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
            <ReceiptsContent csv_file={csv_file} receipts_type={(*selected_type).clone()} />
        </Layout>
    }
}

#[derive(Properties, PartialEq)]
struct ReceiptsContentProps {
    csv_file: UseStateHandle<String>,
    receipts_type: ReceiptsType,
}

#[function_component(ReceiptsContent)]
fn receipts_content(props: &ReceiptsContentProps) -> Html {
    let on_change = {
        let csv_file = props.csv_file.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            if let Some(files) = input.files() {
                if let Some(file) = files.get(0) {
                    csv_file.set(file.name());
                }
            }
        })
    };

    let file_name = if props.csv_file.is_empty() {
        "ファイルが選択されていません".to_string()
    } else {
        (*props.csv_file).clone().to_string()
    };

    html! {
        <>
            <div class="input-group">
                <label class="input-group-btn">
                    <span class="btn btn-primary">
                        {"CSVファイル選択"}
                        <input type="file" accept=".csv" style="display:none" onchange={on_change} />
                    </span>
                </label>
                <input type="text" class="form-control" readonly=true value={file_name} />
            </div>
            <div class="mt-4">
                { match props.receipts_type {
                    ReceiptsType::Dividend => html! { <p>{ "配当金のデータを表示します。" }</p> },
                    ReceiptsType::ProfitAndLoss => html! { <p>{ "実益損益のデータを表示します。" }</p> },
                }}
            </div>
        </>
    }
}
