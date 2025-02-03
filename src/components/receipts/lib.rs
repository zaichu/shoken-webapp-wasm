use crate::setting::{DATE_FORMAT_KEYS, HEADERS, NUMBER_FORMAT_KEYS, YEN_FORMAT_KEYS};
use anyhow::{anyhow, Result};
use chrono::NaiveDate;
use csv::StringRecord;
use encoding_rs::SHIFT_JIS;
use gloo::console;
use std::collections::BTreeMap;
use wasm_bindgen_futures::{spawn_local, JsFuture};
use web_sys::{js_sys, File, HtmlInputElement};
use yew::prelude::*;

#[derive(Properties, PartialEq, Debug, Clone)]
pub struct ReceiptTemplateProps {
    pub name: String,
}

#[function_component]
pub fn ReceiptTemplate<T: ReceiptProps + 'static>(props: &ReceiptTemplateProps) -> Html {
    let item_map = use_state(BTreeMap::<NaiveDate, Vec<T>>::new);
    let item_summary = use_state(Vec::<T>::new);
    let csv_file = use_state(|| None::<File>);
    let file_name = use_state(|| "CSVファイルを選択してください。".to_string());

    let on_input = on_input_callback(csv_file.clone());

    {
        let file_name = file_name.clone();
        let item_map = item_map.clone();
        let item_summary = item_summary.clone();

        use_effect_with((*csv_file).clone(), move |csv_file| {
            file_name.set("".to_string());
            item_summary.set(Vec::<T>::new());

            if let Some(csv_file) = csv_file.clone() {
                spawn_local(async move {
                    file_name.set(csv_file.name());
                    if let Err(err) = read_file(&csv_file)
                        .await
                        .and_then(|content| process_csv_content(item_map, item_summary, content))
                    {
                        console::log!(err.to_string());
                    };
                });
            }
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
                <table class="table table-bordered">{ T::view_summary(&(*item_summary)) }</table>
            </div>
            <div class="mt-1">
                <div class="card shadow-sm">
                    <div class="card-header bg-info text-white">
                        <h5 class="mb-0">{ props.name.clone() }</h5>
                    </div>
                    if csv_file.is_some() {
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
                                            { for items.iter().rev().map(|item| item.view(None)) }
                                            { T::get_profit_record(items).view(Some(format!("table-success"))) }
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

fn on_input_callback(csv_file: UseStateHandle<Option<File>>) -> Callback<InputEvent> {
    Callback::from(move |e: InputEvent| {
        let input: HtmlInputElement = e.target_unchecked_into();
        let value = input.files().and_then(|files| files.get(0));
        csv_file.set(value.clone());
    })
}

fn process_csv_content<T: ReceiptProps + 'static>(
    item_map: UseStateHandle<BTreeMap<NaiveDate, Vec<T>>>,
    item_summary: UseStateHandle<Vec<T>>,
    content: Vec<u8>,
) -> Result<()> {
    let records = self::read_csv(content)?;
    let new_item_map = records
        .into_iter()
        .filter_map(|record| {
            let item = T::from_string_record(record);
            item.get_date().map(|date| (date, item))
        })
        .fold(
            BTreeMap::new(),
            |mut acc: BTreeMap<NaiveDate, Vec<T>>, (date, item)| {
                acc.entry(date).or_default().push(item);
                acc
            },
        );
    item_map.set(new_item_map.clone());

    let new_item_summary: Vec<_> = new_item_map
        .iter()
        .map(|(_, items)| T::get_profit_record(items))
        .collect();
    item_summary.set(new_item_summary);

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

pub trait ReceiptProps: Clone + Sized + PartialEq {
    fn new() -> Self;
    fn is_view(&self) -> bool;
    fn get_all_fields(&self) -> Vec<(&'static str, Option<String>)>;
    fn get_date(&self) -> Option<NaiveDate>;
    fn get_profit_record(items: &[Self]) -> Self;
    fn from_string_record(record: StringRecord) -> Self;
    fn view_summary(items: &[Self]) -> Html;

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

    fn parse_i32(num_str: Option<&str>) -> Option<i32> {
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

    fn parse_u32(num_str: Option<&str>) -> Option<u32> {
        match num_str {
            Some(s) => match s.replace(",", "").parse::<u32>() {
                Ok(n) => Some(n),
                Err(e) => {
                    println!("Failed to parse integer '{}': {}", s, e);
                    None
                }
            },
            None => None,
        }
    }

    fn parse_f64(num_str: Option<&str>) -> Option<f64> {
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

    fn view(&self, tr_class: Option<String>) -> Html {
        if !self.is_view() {
            return html! {};
        };

        html! {
            <tr class={tr_class}>
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

    fn render_td_tr_summary(key: &str, value: i32) -> Html {
        let style = "max-width: 30px;";
        let mut class = "text-nowrap".to_string();
        let value = &format!("{value}");
        let value = Self::format_value(key, value);
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
