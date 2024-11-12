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

use super::lib::{BaseReceipt, BaseReceiptProps, Msg, ReceiptProps};

type FieldsMap = Vec<(&'static str, Option<String>)>;

pub struct ProfitAndLoss {
    base: RefCell<BTreeMap<NaiveDate, Vec<BaseReceiptProps<ProfitAndLossProps>>>>,
}

impl Component for ProfitAndLoss {
    type Message = Msg;

    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        BaseReceipt::create(ctx)
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        todo!()
    }
}

#[derive(PartialEq, Properties, Debug, Clone)]
pub struct ProfitAndLossProps {
    pub base: BaseReceiptProps,
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

impl ReceiptProps for ProfitAndLossProps {
    pub fn new() -> Self {
        Self {
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
        Self {
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

    fn get_date(&self) -> Option<NaiveDate> {
        todo!()
    }

    fn get_all_fields(&self) -> Vec<(&'static str, Option<String>)> {
        todo!()
    }

    fn calc_total(items: &[BaseReceiptProps<Self>]) -> BaseReceiptProps<Self> {
        todo!()
    }
}
