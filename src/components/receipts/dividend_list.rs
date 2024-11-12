use anyhow::Result;
use chrono::{Datelike, NaiveDate};
use csv::StringRecord;
use std::{cell::RefCell, collections::BTreeMap};
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::JsFuture;
use web_sys::{console, js_sys, File, HtmlInputElement};
use yew::prelude::*;

use crate::components::receipts::common;
use crate::setting::HEADERS;

type DividendListMap = RefCell<BTreeMap<NaiveDate, Vec<DividendListProps>>>;
type FieldsMap = Vec<(&'static str, Option<String>)>;

pub struct DividendList {
    csv_file: Option<File>,
    dividend_list_map: DividendListMap,
}

pub enum Msg {
    CSVFileSelect(Option<File>),
    SetDividendListMap(Vec<u8>),
}

impl DividendList {
    fn render_file_input(&self, ctx: &Context<Self>) -> Html {
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

    fn render_dividend_list(&self) -> Html {
        html! {
            <div class="card shadow-sm mb-4">
                <div class="card-header bg-primary text-white">
                    <h5 class="mb-0">{ "配当金" }</h5>
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
        let headers = DividendListProps::new().get_all_fields();
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
            { for self.dividend_list_map.borrow().iter().map(|(_, dividend_list)| {
                let totals = self.calculate_totals(dividend_list);
                let dividend_list_total = DividendListProps::new_total_dividend_list(totals);
                html! {
                    <>
                        { for dividend_list.iter().map(|dividend_list| {
                            html! { <DividendListProps ..dividend_list.clone() /> }
                        }) }
                        <DividendListProps ..dividend_list_total />
                    </>
                }
            }) }
        }
    }

    fn calculate_totals(&self, dividend_list: &[DividendListProps]) -> (i32, i32, i32) {
        dividend_list.iter().fold(
            (0, 0, 0),
            |(total_dividends_before_tax, total_taxes, total_net_amount_received), dividend| {
                if let (Some(dividends_before_tax), Some(taxes), Some(net_amount_received)) = (
                    dividend.dividends_before_tax,
                    dividend.taxes,
                    dividend.net_amount_received,
                ) {
                    (
                        total_dividends_before_tax + dividends_before_tax,
                        total_taxes + taxes,
                        total_net_amount_received + net_amount_received,
                    )
                } else {
                    (
                        total_dividends_before_tax,
                        total_taxes,
                        total_net_amount_received,
                    )
                }
            },
        )
    }

    async fn read_file(file: &File) -> Result<Vec<u8>> {
        let array_buffer = JsFuture::from(file.array_buffer())
            .await
            .map_err(|e| anyhow::anyhow!("{:?}", e))?;
        Ok(js_sys::Uint8Array::new(&array_buffer).to_vec())
    }

    fn process_csv_content(&self, content: Vec<u8>) -> Result<()> {
        let records = common::read_csv(content)?;
        self.dividend_list_map.borrow_mut().clear();
        for record in records {
            let dividend = DividendListProps::from_record(record);
            if let Some(settlement_date) = dividend.settlement_date {
                let date =
                    NaiveDate::from_ymd_opt(settlement_date.year(), settlement_date.month(), 1)
                        .unwrap();
                self.dividend_list_map
                    .borrow_mut()
                    .entry(date)
                    .or_insert_with(Vec::new)
                    .push(dividend);
            }
        }
        Ok(())
    }
}

impl Component for DividendList {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            csv_file: None,
            dividend_list_map: RefCell::new(BTreeMap::new()),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::CSVFileSelect(file) => {
                self.csv_file = file.clone();
                let link = ctx.link().clone();
                wasm_bindgen_futures::spawn_local(async move {
                    if let Some(file) = file {
                        match Self::read_file(&file).await {
                            Ok(content) => link.send_message(Msg::SetDividendListMap(content)),
                            Err(err) => console::log_1(&JsValue::from_str(&format!(
                                "File read error: {:?}",
                                err
                            ))),
                        }
                    }
                });
                true
            }
            Msg::SetDividendListMap(content) => {
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

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
                { self.render_file_input(ctx) }
                <div class="mt-4">
                    { self.render_dividend_list() }
                </div>
            </>
        }
    }
}

#[derive(PartialEq, Properties, Debug, Clone)]
pub struct DividendListProps {
    pub tr_class: String,                        // 行のclass
    pub settlement_date: Option<NaiveDate>,      // 入金日(受渡日)
    pub product: Option<String>,                 // 商品
    pub account: Option<String>,                 // 口座
    pub security_code: Option<String>,           // 銘柄コード
    pub security_name: Option<String>,           // 銘柄
    pub currency: Option<String>,                // 受取通貨
    pub unit_price: Option<String>,              // 単価[円/現地通貨]
    pub shares: Option<i32>,                     // 数量[株/口]
    pub dividends_before_tax: Option<i32>,       // 配当・分配金（税引前）[円/現地通貨]
    pub taxes: Option<i32>,                      // 税額[円/現地通貨]
    pub net_amount_received: Option<i32>,        // 受取金額[円/現地通貨]
    pub total_dividends_before_tax: Option<i32>, // 配当・分配金合計（税引前）[円/現地通貨]
    pub total_taxes: Option<i32>,                // 税額合計[円/現地通貨]
    pub total_net_amount_received: Option<i32>,  // 受取金額合計[円/現地通貨]
}

impl DividendListProps {
    pub fn new() -> Self {
        DividendListProps {
            tr_class: "".to_string(),
            settlement_date: None,
            product: None,
            account: None,
            security_code: None,
            security_name: None,
            currency: None,
            unit_price: None,
            shares: None,
            dividends_before_tax: None,
            taxes: None,
            net_amount_received: None,
            total_dividends_before_tax: None,
            total_taxes: None,
            total_net_amount_received: None,
        }
    }
    pub fn from_record(record: StringRecord) -> Self {
        DividendListProps {
            tr_class: "".to_string(),
            settlement_date: common::parse_date(record.get(0)),
            product: common::parse_string(record.get(1)),
            account: common::parse_string(record.get(2)),
            security_code: common::parse_string(record.get(3)),
            security_name: common::parse_string(record.get(4)),
            currency: common::parse_string(record.get(5)),
            unit_price: common::parse_string(record.get(6)),
            shares: common::parse_int(record.get(7)),
            dividends_before_tax: common::parse_int(record.get(8)),
            taxes: common::parse_int(record.get(9)),
            net_amount_received: common::parse_int(record.get(10)),
            total_dividends_before_tax: None,
            total_taxes: None,
            total_net_amount_received: None,
        }
    }

    pub fn get_all_fields(&self) -> FieldsMap {
        vec![
            (
                "settlement_date",
                self.settlement_date.map(|d| d.to_string()),
            ),
            ("product", self.product.clone()),
            ("account", self.account.clone()),
            ("security_code", self.security_code.clone()),
            ("security_name", self.security_name.clone()),
            ("currency", self.currency.clone()),
            ("unit_price", self.unit_price.clone()),
            ("shares", self.shares.map(|s| s.to_string())),
            (
                "dividends_before_tax",
                self.dividends_before_tax.map(|t| t.to_string()),
            ),
            ("taxes", self.taxes.map(|t| t.to_string())),
            (
                "net_amount_received",
                self.net_amount_received.map(|t| t.to_string()),
            ),
            (
                "total_dividends_before_tax",
                self.total_dividends_before_tax.map(|t| t.to_string()),
            ),
            ("total_taxes", self.total_taxes.map(|t| t.to_string())),
            (
                "total_net_amount_received",
                self.total_net_amount_received.map(|t| t.to_string()),
            ),
        ]
    }

    pub fn new_total_dividend_list(
        (total_dividends_before_tax, total_taxes, total_net_amount_received): (i32, i32, i32),
    ) -> Self {
        DividendListProps {
            tr_class: "table-success".to_string(),
            settlement_date: None,
            product: None,
            account: None,
            security_code: None,
            security_name: None,
            currency: None,
            unit_price: None,
            shares: None,
            dividends_before_tax: None,
            taxes: None,
            net_amount_received: None,
            total_dividends_before_tax: Some(total_dividends_before_tax),
            total_taxes: Some(total_taxes),
            total_net_amount_received: Some(total_net_amount_received),
        }
    }
}

impl Component for DividendListProps {
    type Message = ();
    type Properties = DividendListProps;

    fn create(ctx: &Context<Self>) -> Self {
        ctx.props().clone()
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
        <>
        <tr class={&self.tr_class}>
            { for self.get_all_fields().iter().map(|(key, value)| {
                let value = value.as_deref().unwrap_or("");
                let value = common::format_value(key, value);
                let style = "overflow-wrap: break-word; white-space: normal;";
                let class = "text-nowrap".to_string();

                html! {
                    <td class={class} style={style}>
                        {value}
                    </td>
                }})
            }
        </tr>
        </>
        }
    }
}
