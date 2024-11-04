use chrono::NaiveDate;
use csv::StringRecord;
use std::{cell::RefCell, collections::BTreeMap};
use wasm_bindgen::JsValue;
use web_sys::{console, File, HtmlInputElement};
use yew::prelude::*;

use crate::components::receipts::common;
use crate::setting::TAX_RATE;

#[derive(PartialEq, Properties, Debug, Clone)]
pub struct ProfitAndLoss {
    csv_file: Option<File>,
    profit_and_loss_map: RefCell<BTreeMap<NaiveDate, Vec<ProfitAndLossProps>>>,
}

pub enum ProfitAndLossMsg {
    CSVFileSelect(Option<File>),
}

impl Component for ProfitAndLoss {
    type Message = ProfitAndLossMsg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        ProfitAndLoss {
            csv_file: None,
            profit_and_loss_map: RefCell::new(BTreeMap::new()),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            ProfitAndLossMsg::CSVFileSelect(file) => {
                self.csv_file = file;
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let on_change = ctx.link().callback(|e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            if let Some(files) = input.files() {
                if let Some(file) = files.get(0) {
                    ProfitAndLossMsg::CSVFileSelect(Some(file))
                } else {
                    ProfitAndLossMsg::CSVFileSelect(None)
                }
            } else {
                ProfitAndLossMsg::CSVFileSelect(None)
            }
        });

        let file_name = self
            .csv_file
            .as_ref()
            .map(|file| file.name())
            .unwrap_or_else(|| "CSVファイルを選択してください。".to_string());

        html! {
        <>
            <div class="input-group">
                <label class="input-group-btn" for="csv-file-input">
                    <span class="btn btn-primary">
                        { "CSVファイル選択" }
                    </span>
                </label>
                <input id="csv-file-input" type="file" accept=".csv" style="display:none" onchange={on_change} />
                <input type="text" class="form-control" readonly=true value={file_name} />
            </div>

            { for self.profit_and_loss_map.borrow().iter().map(|(date, props_list)| {
                html! {
                    <div key={date.to_string()}>
                        <h2>{ date.format("%Y-%m-%d").to_string() }</h2>
                        { for props_list.iter().map(|props| html! {
                            <ProfitAndLossProps ..props.clone() />
                        }) }
                    </div>
                }
            }) }
        </>
        }
    }
}

#[derive(PartialEq, Properties, Debug, Clone)]
pub struct ProfitAndLossProps {
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
    pub fn from_record(&self, record: StringRecord) -> Self {
        ProfitAndLossProps {
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

    pub fn get_all_fields(&self) -> Vec<(String, Option<String>)> {
        vec![
            (
                "trade_date".to_string(),
                self.trade_date.map(|d| d.to_string()),
            ),
            (
                "settlement_date".to_string(),
                self.settlement_date.map(|d| d.to_string()),
            ),
            ("security_code".to_string(), self.security_code.clone()),
            ("security_name".to_string(), self.security_name.clone()),
            ("account".to_string(), self.account.clone()),
            ("shares".to_string(), self.shares.map(|s| s.to_string())),
            (
                "asked_price".to_string(),
                self.asked_price.map(|p| p.to_string()),
            ),
            ("proceeds".to_string(), self.proceeds.map(|p| p.to_string())),
            (
                "purchase_price".to_string(),
                self.purchase_price.map(|p| p.to_string()),
            ),
            (
                "realized_profit_and_loss".to_string(),
                self.realized_profit_and_loss.map(|p| p.to_string()),
            ),
            (
                "total_realized_profit_and_loss".to_string(),
                self.total_realized_profit_and_loss.map(|p| p.to_string()),
            ),
            (
                "withholding_tax".to_string(),
                self.withholding_tax.map(|p| p.to_string()),
            ),
            (
                "profit_and_loss".to_string(),
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
        html! {}
    }
}
