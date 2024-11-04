use anyhow::Result;
use chrono::NaiveDate;
use csv::StringRecord;
use std::{cell::RefCell, collections::BTreeMap};
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::JsFuture;
use web_sys::{console, js_sys, File, HtmlInputElement};
use yew::prelude::*;

use crate::components::receipts::common;
use crate::setting::{HEADERS, TAX_RATE};

type ProfitAndLossMap = RefCell<BTreeMap<NaiveDate, Vec<ProfitAndLossProps>>>;
type FieldsMap = Vec<(&'static str, Option<String>)>;

pub struct ProfitAndLoss {
    csv_file: Option<File>,
    profit_and_loss_map: ProfitAndLossMap,
}

pub enum Msg {
    CSVFileSelect(Option<File>),
    SetProfitAndLossMap(Vec<u8>),
}

impl ProfitAndLoss {
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
        let headers = ProfitAndLossProps::new().get_all_fields();
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
            { for self.profit_and_loss_map.borrow().iter().map(|(_, profit_and_loss_list)| {
                let totals = self.calculate_totals(profit_and_loss_list);
                let profit_and_loss_total = ProfitAndLossProps::new_total_realized_profit_and_loss(totals);
                html! {
                    <>
                        { for profit_and_loss_list.iter().map(|profit_and_loss| {
                            html! { <ProfitAndLossProps ..profit_and_loss.clone() /> }
                        }) }
                        <ProfitAndLossProps ..profit_and_loss_total />
                    </>
                }
            }) }
        }
    }

    fn calculate_totals(&self, profit_and_loss_list: &[ProfitAndLossProps]) -> (i32, i32) {
        profit_and_loss_list
            .iter()
            .fold((0, 0), |(specific, nisa), profit_and_loss| {
                if let (Some(account), Some(realized_profit_and_loss)) = (
                    profit_and_loss.account.as_deref(),
                    profit_and_loss.realized_profit_and_loss,
                ) {
                    if account.contains("特定") {
                        (specific + realized_profit_and_loss, nisa)
                    } else {
                        (specific, nisa + realized_profit_and_loss)
                    }
                } else {
                    (specific, nisa)
                }
            })
    }

    async fn read_file(file: &File) -> Result<Vec<u8>> {
        let array_buffer = JsFuture::from(file.array_buffer())
            .await
            .map_err(|e| anyhow::anyhow!("{:?}", e))?;
        Ok(js_sys::Uint8Array::new(&array_buffer).to_vec())
    }

    fn process_csv_content(&self, content: Vec<u8>) -> Result<()> {
        let records = common::read_csv(content)?;
        self.profit_and_loss_map.borrow_mut().clear();
        for record in records {
            let profit_and_loss = ProfitAndLossProps::from_record(record);
            if let Some(trade_date) = profit_and_loss.trade_date {
                self.profit_and_loss_map
                    .borrow_mut()
                    .entry(trade_date)
                    .or_insert_with(Vec::new)
                    .push(profit_and_loss);
            }
        }
        Ok(())
    }
}

impl Component for ProfitAndLoss {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            csv_file: None,
            profit_and_loss_map: RefCell::new(BTreeMap::new()),
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
                            Ok(content) => link.send_message(Msg::SetProfitAndLossMap(content)),
                            Err(err) => console::log_1(&JsValue::from_str(&format!(
                                "File read error: {:?}",
                                err
                            ))),
                        }
                    }
                });
                true
            }
            Msg::SetProfitAndLossMap(content) => {
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
                    { self.render_profit_and_loss_list() }
                </div>
            </>
        }
    }
}

#[derive(PartialEq, Properties, Debug, Clone)]
pub struct ProfitAndLossProps {
    pub tr_class: String,                            // 行のclass
    pub trade_date: Option<NaiveDate>,               // 約定日
    pub settlement_date: Option<NaiveDate>,          // 受渡日
    pub security_code: Option<String>,               // 銘柄コード
    pub security_name: Option<String>,               // 銘柄名
    pub account: Option<String>,                     // 口座
    pub shares: Option<i32>,                         // 数量[株]
    pub asked_price: Option<f64>,                    // 売却/決済単価[円]
    pub proceeds: Option<i32>,                       // 売却/決済額[円]
    pub purchase_price: Option<f64>,                 // 平均取得価額[円]
    pub realized_profit_and_loss: Option<i32>,       // 実現損益[円]
    pub total_realized_profit_and_loss: Option<i32>, // 合計実現損益[円]
    pub withholding_tax: Option<u32>,                // 源泉徴収税額
    pub profit_and_loss: Option<i32>,                // 損益
}

impl ProfitAndLossProps {
    pub fn new() -> Self {
        ProfitAndLossProps {
            tr_class: "".to_string(),
            trade_date: None,
            settlement_date: None,
            security_code: None,
            security_name: None,
            account: None,
            shares: None,
            asked_price: None,
            proceeds: None,
            purchase_price: None,
            realized_profit_and_loss: None,
            total_realized_profit_and_loss: None,
            withholding_tax: None,
            profit_and_loss: None,
        }
    }
    pub fn from_record(record: StringRecord) -> Self {
        ProfitAndLossProps {
            tr_class: "".to_string(),
            trade_date: common::parse_date(record.get(0)),
            settlement_date: common::parse_date(record.get(1)),
            security_code: common::parse_string(record.get(2)),
            security_name: common::parse_string(record.get(3)),
            account: common::parse_string(record.get(4)),
            shares: common::parse_int(record.get(7)),
            asked_price: common::parse_float(record.get(8)),
            proceeds: common::parse_int(record.get(9)),
            purchase_price: common::parse_float(record.get(10)),
            realized_profit_and_loss: common::parse_int(record.get(11)),
            total_realized_profit_and_loss: None,
            withholding_tax: None,
            profit_and_loss: None,
        }
    }

    pub fn get_all_fields(&self) -> FieldsMap {
        vec![
            ("trade_date", self.trade_date.map(|d| d.to_string())),
            (
                "settlement_date",
                self.settlement_date.map(|d| d.to_string()),
            ),
            ("security_code", self.security_code.clone()),
            ("security_name", self.security_name.clone()),
            ("account", self.account.clone()),
            ("shares", self.shares.map(|s| s.to_string())),
            ("asked_price", self.asked_price.map(|p| p.to_string())),
            ("proceeds", self.proceeds.map(|p| p.to_string())),
            ("purchase_price", self.purchase_price.map(|p| p.to_string())),
            (
                "realized_profit_and_loss",
                self.realized_profit_and_loss.map(|p| p.to_string()),
            ),
            (
                "total_realized_profit_and_loss",
                self.total_realized_profit_and_loss.map(|p| p.to_string()),
            ),
            (
                "withholding_tax",
                self.withholding_tax.map(|p| p.to_string()),
            ),
            (
                "profit_and_loss",
                self.profit_and_loss.map(|p| p.to_string()),
            ),
        ]
    }

    pub fn new_total_realized_profit_and_loss(
        (specific_account_total, nisa_account_total): (i32, i32),
    ) -> Self {
        let withholding_tax = if specific_account_total < 0 {
            0
        } else {
            (specific_account_total as f64 * TAX_RATE) as u32
        };
        let total = specific_account_total + nisa_account_total;

        ProfitAndLossProps {
            tr_class: "table-success".to_string(),
            trade_date: None,
            settlement_date: None,
            security_code: None,
            security_name: None,
            account: None,
            shares: None,
            asked_price: None,
            proceeds: None,
            purchase_price: None,
            realized_profit_and_loss: None,
            total_realized_profit_and_loss: Some(total),
            withholding_tax: Some(withholding_tax),
            profit_and_loss: Some(total - withholding_tax as i32),
        }
    }
}

impl Component for ProfitAndLossProps {
    type Message = ();
    type Properties = ProfitAndLossProps;

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
                let mut class = "text-nowrap".to_string();
                if value.starts_with("¥ -") {
                     class = format!("{} text-danger", class);
                }

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
