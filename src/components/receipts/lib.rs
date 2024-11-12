use anyhow::{anyhow, Result};
use chrono::NaiveDate;
use csv::StringRecord;
use encoding_rs::SHIFT_JIS;
use std::str;
use std::{cell::RefCell, collections::BTreeMap};
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::JsFuture;
use web_sys::{console, js_sys, File, HtmlInputElement};
use yew::platform::spawn_local;
use yew::prelude::*;

use crate::setting::{HEADERS, TAX_RATE};

pub type FieldsMap = Vec<(&'static str, Option<String>)>;

use crate::setting::{DATE_FORMAT_KEYS, NUMBER_FORMAT_KEYS, YEN_FORMAT_KEYS};

pub enum Msg {
    CSVFileSelect(Option<File>),
    SetItemMap(Vec<u8>),
}

pub struct BaseReceipt<T: ReceiptProps + 'static> {
    csv_file: Option<File>,
    item_map: RefCell<BTreeMap<NaiveDate, Vec<BaseReceiptProps<T>>>>,
}

impl<T: ReceiptProps + Component<Message = Msg>> BaseReceipt<T> {
    pub fn create(_ctx: &Context<T>) -> Self {
        Self {
            csv_file: None,
            item_map: RefCell::new(BTreeMap::new()),
        }
    }

    pub fn update(&mut self, ctx: &Context<T>, msg: T::Message) -> bool
    where
        <T as Component>::Message: From<Msg>,
    {
        match msg {
            Msg::CSVFileSelect(file) => {
                self.csv_file = file.clone();
                let link = ctx.link().clone();
                wasm_bindgen_futures::spawn_local(async move {
                    if let Some(file) = file {
                        match Self::read_file(&file).await {
                            Ok(content) => link.send_message(Msg::SetItemMap(content)),
                            Err(err) => console::log_1(&JsValue::from_str(&format!(
                                "File read error: {:?}",
                                err
                            ))),
                        }
                    }
                });
                true
            }
            Msg::SetItemMap(content) => {
                if let Err(err) = self.process_csv_content(content) {
                    console::log_1(&JsValue::from_str(&format!(
                        "CSV processing error: {:?}",
                        err
                    )));
                }
                true
            }
        }
    }

    fn view(&self, ctx: &Context<T>) -> Html
    where
        <T as Component>::Message: From<Msg>,
    {
        html! {
            <>
                { self.render_file_input(ctx) }
                <div class="mt-4">
                    { self.render_profit_and_loss_list() }
                </div>
            </>
        }
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

    fn render_file_input(&self, ctx: &Context<T>) -> Html
    where
        <T as Component>::Message: From<Msg>,
    {
        let on_change = ctx.link().callback(|e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            Msg::CSVFileSelect(input.files().and_then(|files| files.get(0)))
        });

        let file_name = self
            .csv_file
            .as_ref()
            .map(|file| file.name())
            .unwrap_or_else(|| "CSVファイルを選択してください。".to_string());

        html! {
            <div class="input-group">
                <label class="input-group-btn" for="csv-file-input">
                    <span class="btn btn-primary">{ "CSVファイル選択" }</span>
                </label>
                <input id="csv-file-input" type="file" accept=".csv" style="display:none" onchange={on_change} />
                <input type="text" class="form-control" readonly=true value={file_name} />
            </div>
        }
    }

    fn render_profit_and_loss_list(&self) -> Html {
        html! {
            <div class="card shadow-sm mb-4">
                <div class="card-header bg-primary text-white">
                    <h5 class="mb-0">{ "実益損益" }</h5>
                </div>
                if self.csv_file.is_some(){
                    { self.render_table() }
                }
            </div>
        }
    }

    fn render_table(&self) -> Html {
        html! {
            <div class="table-responsive" style="max-height: 500px;">
                <table class="table table-bordered">
                    { self.render_table_header() }
                    <tbody>
                        { self.render_table_body() }
                    </tbody>
                </table>
            </div>
        }
    }

    fn render_table_header(&self) -> Html {
        let headers = T::new().get_all_fields();
        html! {
            <thead class="thead-light">
                <tr>
                    { for headers.iter().map(|(header, _)| {
                        let header_text = HEADERS.get(header).unwrap_or(header);
                        html! {
                            <th scope="col" style="position: sticky; top: 0; background-color: white; white-space: nowrap; text-align: center;">
                                { header_text }
                            </th>
                        }
                    }) }
                </tr>
            </thead>
        }
    }

    fn render_table_body(&self) -> Html {
        html! {
            { for self.item_map.borrow().iter().map(|(_, items)| {
                let total = T::calc_total(items);
                html! {
                    <>
                        { for items.iter().map(|item| {
                            html! { item.view() }
                        }) }
                        { total.view() }
                    </>
                }
            }) }
        }
    }

    fn process_csv_content(&self, content: Vec<u8>) -> Result<()> {
        let records = BaseReceipt::<T>::read_csv(content)?;
        self.item_map.borrow_mut().clear();
        for record in records {
            let item = T::from_record(record);
            if let Some(trade_date) = item.props.get_date() {
                self.item_map
                    .borrow_mut()
                    .entry(trade_date)
                    .or_insert_with(Vec::new)
                    .push(item);
            }
        }
        Ok(())
    }
}

pub struct BaseReceiptProps<T: ReceiptProps> {
    pub tr_class: String,
    pub props: T,
}

impl<T: ReceiptProps + Component> BaseReceiptProps<T> {
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
        let (sign, s) = if s.starts_with('-') {
            ("-", &s[1..])
        } else {
            ("", s)
        };

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

        let mut result = format!("{}{}", sign, integer_part);
        if let Some(decimal_part) = parts.get(1) {
            result.push('.');
            result.push_str(decimal_part);
        }
        result
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
            <tr class={&self.tr_class}>
                { for self.props.get_all_fields().iter().map(|(key, value)| {
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

pub trait ReceiptProps: Clone + Sized {
    fn new() -> Self;
    fn get_date(&self) -> Option<NaiveDate>;
    fn get_all_fields(&self) -> Vec<(&'static str, Option<String>)>;
    fn from_record(record: StringRecord) -> BaseReceiptProps<Self>;
    fn calc_total(items: &[BaseReceiptProps<Self>]) -> BaseReceiptProps<Self>;
}
