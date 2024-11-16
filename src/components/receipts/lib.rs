use anyhow::{anyhow, Result};
use chrono::NaiveDate;
use csv::StringRecord;
use encoding_rs::SHIFT_JIS;
use std::collections::BTreeMap;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::JsFuture;
use web_sys::{console, js_sys, File, HtmlInputElement};
use yew::prelude::*;

use crate::setting::{DATE_FORMAT_KEYS, HEADERS, NUMBER_FORMAT_KEYS, YEN_FORMAT_KEYS};

#[derive(Properties, PartialEq, Debug, Clone)]
pub struct ReceiptTemplateProps {
    pub name: String,
}

#[function_component]
pub fn ReceiptTemplate<T: ReceiptProps + 'static>(props: &ReceiptTemplateProps) -> Html {
    let item_map = use_state(|| BTreeMap::<NaiveDate, Vec<T>>::new());
    let csv_file = use_state(|| None::<File>);
    let file_name = use_state(|| format!("CSVファイルを選択してください。"));

    let on_input = {
        let item_map = item_map.clone();
        let csv_file = csv_file.clone();
        let file_name = file_name.clone();

        Callback::from(move |e: InputEvent| {
            let item_map = item_map.clone();
            let input: HtmlInputElement = e.target_unchecked_into();
            let value = input.files().and_then(|files| files.get(0));
            csv_file.set(value.clone());
            file_name.set(
                csv_file
                    .as_ref()
                    .map(|file| file.name())
                    .unwrap_or_else(|| "CSVファイルを選択してください。".to_string()),
            );

            wasm_bindgen_futures::spawn_local(async move {
                if let Some(csv_file) = value {
                    let err_message = match self::read_file(&csv_file).await {
                        Ok(file) => match self::process_csv_content(item_map, file) {
                            Ok(_) => format!(""),
                            Err(err) => format!("CSV processing error: {:?}", err),
                        },
                        Err(err) => format!("File read error: {:?}", err),
                    };

                    if !err_message.is_empty() {
                        console::log_1(&JsValue::from_str(&err_message));
                    }
                }
            });
        })
    };

    html! {
    <>
        <div class="input-group">
            <label class="input-group-btn" for="csv-file-input">
                <span class="btn btn-primary">{ "CSVファイル選択" }</span>
            </label>
            <input id="csv-file-input" type="file" accept=".csv" style="display:none" oninput={on_input} />
            <input type="text" class="form-control" readonly=true value={(*file_name).clone()} />
        </div>
        <div class="mt-4">
            <div class="card shadow-sm mb-4">
                <div class="card-header bg-primary text-white">
                    <h5 class="mb-0">{ props.name.clone() }</h5>
                </div>
                if csv_file.is_some(){
                    <div class="table-responsive" style="max-height: 500px;">
                        <table class="table table-bordered">
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
                            <tbody> {
                                for item_map.iter().map(|(_, items)| {
                                    html! {
                                    <>
                                        { for items.iter().map(|item| item.view()) }
                                        { T::get_profit_record(items).view() }
                                    </>
                                    }
                                })
                            }
                            </tbody>
                        </table>
                    </div>
                }
            </div>
        </div>
    </>
    }
}

fn process_csv_content<T: ReceiptProps + 'static>(
    item_map: UseStateHandle<BTreeMap<NaiveDate, Vec<T>>>,
    content: Vec<u8>,
) -> Result<()> {
    let records = self::read_csv(content)?;

    item_map.set(BTreeMap::new());
    let mut item_map_tmp = BTreeMap::new();
    for record in records {
        let item = T::from_string_record(record);
        if let Some(date) = item.get_date() {
            item_map_tmp.entry(date).or_insert_with(Vec::new).push(item);
        }
    }
    item_map.set(item_map_tmp);

    Ok(())
}

fn read_csv(bytes: Vec<u8>) -> Result<Vec<StringRecord>> {
    let (cow, _, had_errors) = SHIFT_JIS.decode(&bytes);
    if had_errors {
        return Err(anyhow!("Error decoding Shift-JIS"));
    }
    let utf8_string = cow.into_owned();
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_reader(utf8_string.as_bytes());

    rdr.records()
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| anyhow!("Failed to read CSV: {}", e))
}

async fn read_file(file: &File) -> Result<Vec<u8>> {
    let array_buffer = JsFuture::from(file.array_buffer())
        .await
        .map_err(|e| anyhow::anyhow!("{:?}", e))?;
    Ok(js_sys::Uint8Array::new(&array_buffer).to_vec())
}

#[derive(PartialEq, Properties, Debug, Clone, Eq)]
pub struct BaseReceiptProps {
    pub tr_class: String,
}

impl BaseReceiptProps {
    pub fn new(_tr_class: &str) -> Self {
        Self {
            tr_class: _tr_class.to_string(),
        }
    }
}
pub trait ReceiptProps: Clone + Sized + PartialEq {
    fn new() -> Self;
    fn get_all_fields(&self) -> Vec<(&'static str, Option<String>)>;
    fn get_date(&self) -> Option<NaiveDate>;
    fn get_profit_record(items: &[Self]) -> Self;
    fn get_tr_class(&self) -> String;
    fn from_string_record(record: StringRecord) -> Self;

    fn format_value(key: &str, value: &str) -> String {
        if YEN_FORMAT_KEYS.contains(key) {
            Self::format_yen(value)
        } else if NUMBER_FORMAT_KEYS.contains(key) {
            Self::format_number(value)
        } else if DATE_FORMAT_KEYS.contains(key) {
            Self::format_date(value)
        } else {
            value.to_string()
        }
    }

    fn format_date(s: &str) -> String {
        s.replace("-", "/")
    }

    fn format_number(s: &str) -> String {
        let (sign, s) = s.strip_prefix('-').map_or(("", s), |s| ("-", s));
        let parts: Vec<&str> = s.split('.').collect();
        let integer_part = parts[0]
            .chars()
            .rev()
            .enumerate()
            .fold(String::new(), |mut acc, (i, c)| {
                if i > 0 && i % 3 == 0 {
                    acc.push(',');
                }
                acc.push(c);
                acc
            })
            .chars()
            .rev()
            .collect::<String>();

        format!(
            "{}{}{}",
            sign,
            integer_part,
            parts.get(1).map_or(String::new(), |&s| format!(".{}", s))
        )
    }

    fn format_yen(s: &str) -> String {
        if s.is_empty() {
            return "".to_string();
        }

        format!("¥ {}", Self::format_number(s))
    }

    fn parse_date(date_str: Option<&str>) -> Option<NaiveDate> {
        match date_str {
            Some(date_str) => {
                let date = NaiveDate::parse_from_str(&date_str, "%Y/%m/%d")
                    .map_err(|e| anyhow!("Failed to parse date '{}': {}", date_str, e));

                match date {
                    Ok(date) => Some(date),
                    Err(e) => {
                        println!("{e}");
                        None
                    }
                }
            }
            None => None,
        }
    }

    fn parse_int(num_str: Option<&str>) -> Option<i32> {
        match num_str {
            Some(s) => match s.replace(",", "").parse::<i32>() {
                Ok(n) => Some(n),
                Err(e) => {
                    println!("Failed to parse integer '{}': {}", s, e);
                    None
                }
            },
            None => None,
        }
    }

    fn parse_float(num_str: Option<&str>) -> Option<f64> {
        match num_str {
            Some(s) => match s.replace(",", "").parse::<f64>() {
                Ok(n) => Some(n),
                Err(e) => {
                    println!("Failed to parse float '{}': {}", s, e);
                    None
                }
            },
            None => None,
        }
    }

    fn parse_string(value: Option<&str>) -> Option<String> {
        match value {
            Some(s) => Some(s.to_string()),
            None => None,
        }
    }

    fn view(&self) -> Html {
        html! {
            <tr class={&self.get_tr_class()}>
                { for self.get_all_fields().iter().map(|(key, value)| {
                    let value = value.as_deref().unwrap_or("");
                    let value = Self::format_value(key, value);
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
}
