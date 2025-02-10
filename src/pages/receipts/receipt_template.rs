use chrono::NaiveDate;
use csv::StringRecord;
use gloo::console;
use std::collections::BTreeMap;
use wasm_bindgen_futures::spawn_local;
use web_sys::{File, HtmlInputElement};
use yew::prelude::*;

use crate::{services::*, setting::*};

#[derive(Properties, PartialEq, Debug, Clone)]
pub struct ReceiptTemplateProps {
    pub name: String,
}

#[function_component]
pub fn ReceiptTemplate<T: ReceiptProps>(props: &ReceiptTemplateProps) -> Html {
    let receipt_map = use_state(BTreeMap::<NaiveDate, Vec<T>>::new);
    let receipt_summary = use_state(BTreeMap::<NaiveDate, T>::new);
    let csv_file = use_state(|| None::<File>);
    let file_name = use_state(|| "CSVファイルを選択してください。".to_string());

    let on_input = on_input_callback(csv_file.clone());

    {
        let file_name = file_name.clone();
        let receipt_map = receipt_map.clone();
        let receipt_summary = receipt_summary.clone();

        use_effect_with((*csv_file).clone(), move |csv_file| {
            handle_file_change((*csv_file).clone(), file_name, receipt_map, receipt_summary);
        });
    }

    html! {
        <>
            <div class="input-group">
                <label class="input-group-btn" for="csv-file-input">
                    <span class="btn bg-info text-white">{ "CSVファイル選択" }</span>
                </label>
                <input id="csv-file-input" type="file" accept=".csv" style="display:none" oninput={on_input} />
                <input type="text" class="form-control" readonly=true value={(*file_name).clone()} />
            </div>
            <div class="mt-2">
                <table class="table table-bordered">{ T::view_summary(&(*receipt_summary)) }</table>
            </div>
            <div class="mt-1">
                <div class="card shadow-sm">
                    <div class="card-header bg-info text-white">
                        <h5 class="mb-0">{ props.name.clone() }</h5>
                    </div>
                    if csv_file.is_some() {
                        <div class="table-responsive" style="max-height: 500px;">
                            <table class="table table-bordered">
                                { render_thead::<T>() }
                                { render_tbody::<T>(&receipt_map, &receipt_summary) }
                            </table>
                        </div>
                    }
                </div>
            </div>
        </>
    }
}

fn render_thead<T: ReceiptProps>() -> Html {
    html! {
    <thead class="thead-light">
        <tr> {
            for T::new().get_all_fields().iter().map(|(header, _)| {
                let header_text = HEADERS.get(header).unwrap_or(header);
                html! {
                    <th scope="col" style="position: sticky; top: 0; background-color: white; white-space: nowrap; text-align: center;">
                        { header_text }
                    </th>
                }
            })
        }
        </tr>
    </thead>
    }
}

fn render_tbody<T: ReceiptProps>(
    receipt_map: &UseStateHandle<BTreeMap<NaiveDate, Vec<T>>>,
    receipt_summary: &UseStateHandle<BTreeMap<NaiveDate, T>>,
) -> Html {
    html! {
    <tbody> {
        for receipt_map.iter().map(|(date, receipts)| {
            let summary_view = if T::is_view_summary_table() {
                receipt_summary.get(date).map( |summary| summary.view(Some(format!("table-success"))))
            } else {
                None
            };
            html! { for receipts.iter().rev().map(|receipt| receipt.view(None)).chain(summary_view) }
        })
    }
    </tbody>
    }
}

fn handle_file_change<T: ReceiptProps>(
    csv_file: Option<File>,
    file_name: UseStateHandle<String>,
    receipt_map: UseStateHandle<BTreeMap<NaiveDate, Vec<T>>>,
    receipt_summary: UseStateHandle<BTreeMap<NaiveDate, T>>,
) {
    file_name.set("".to_string());
    receipt_summary.set(BTreeMap::new());

    if let Some(csv_file) = csv_file.clone() {
        spawn_local(async move {
            file_name.set(csv_file.name());
            if let Err(err) = csv_reader::read_file(&csv_file)
                .await
                .and_then(|content| process_csv_content(receipt_map, receipt_summary, content))
            {
                console::log!(err.to_string());
            };
        });
    }
}

fn on_input_callback(csv_file: UseStateHandle<Option<File>>) -> Callback<InputEvent> {
    Callback::from(move |e: InputEvent| {
        let input: HtmlInputElement = e.target_unchecked_into();
        let value = input.files().and_then(|files| files.get(0));
        csv_file.set(value.clone());
    })
}

fn process_csv_content<T: ReceiptProps>(
    receipt_map: UseStateHandle<BTreeMap<NaiveDate, Vec<T>>>,
    receipt_summary: UseStateHandle<BTreeMap<NaiveDate, T>>,
    content: Vec<u8>,
) -> Result<(), csv_reader::CSVError> {
    let records = csv_reader::read_csv(content)?;
    let new_receipt_map = records
        .into_iter()
        .filter_map(|record| {
            let receipt = T::new_from_string_record(record);
            receipt.get_date().map(|date| (date, receipt))
        })
        .fold(
            BTreeMap::new(),
            |mut acc: BTreeMap<NaiveDate, Vec<T>>, (date, receipt)| {
                acc.entry(date).or_default().push(receipt);
                acc
            },
        );
    receipt_map.set(new_receipt_map.clone());

    let new_receipt_summary = new_receipt_map
        .iter()
        .map(|(date, receipts)| (*date, T::new_summary(receipts)))
        .collect::<BTreeMap<NaiveDate, T>>();
    receipt_summary.set(new_receipt_summary);

    Ok(())
}

pub trait ReceiptProps: Clone + Sized + PartialEq + 'static {
    fn new() -> Self;
    fn new_summary(receipts: &[Self]) -> Self;
    fn new_from_string_record(record: StringRecord) -> Self;

    fn get_all_fields(&self) -> Vec<(&'static str, Option<String>)>;
    fn get_date(&self) -> Option<NaiveDate>;
    fn is_view_summary_table() -> bool;
    fn view_summary(receipt_summary: &BTreeMap<NaiveDate, Self>) -> Html;
    fn view(&self, tr_class: Option<String>) -> Html {
        html! {
            <tr class={tr_class}>
                { for self.get_all_fields().iter().map(|(key, value)| {
                    let value = value.as_deref().unwrap_or("");
                    let value = formater::format_value(key, value);
                    let style = "overflow-wrap: break-word; white-space: normal;";
                    let mut class = "text-nowrap".to_string();
                    if value.starts_with("¥ -") {
                        class = format!("{} text-danger", class);
                    }
                    html! {
                        <td class={class} style={style}>
                            {value}
                        </td>
                    }
                })}
            </tr>
        }
    }

    fn render_summary_th_td(key: &str, value: i32) -> Html {
        let style = "max-width: 30px;";
        let mut class = "text-nowrap".to_string();
        let value = &format!("{value}");
        let value = formater::format_value(key, value);
        if value.starts_with("¥ -") {
            class = format!("{} text-danger", class);
        }
        html! {
        <>
            <th class="bg-info text-white text-nowrap" style="max-width: 20px;">{HEADERS.get(key).unwrap_or(&key)}</th>
            <td class={class} style={style}>{value}</td>
        </>
        }
    }
}
