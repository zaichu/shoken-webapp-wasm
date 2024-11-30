use chrono::NaiveDate;
use csv::StringRecord;
use yew::prelude::*;

use crate::setting::TAX_RATE;

use super::lib::ReceiptProps;

#[derive(PartialEq, Properties, Debug, Clone)]
pub struct ProfitAndLoss {
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

impl ReceiptProps for ProfitAndLoss {
    fn new() -> Self {
        Self {
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

    fn from_string_record(record: StringRecord) -> Self {
        Self {
            trade_date: Self::parse_date(record.get(0)),
            settlement_date: Self::parse_date(record.get(1)),
            security_code: Self::parse_string(record.get(2)),
            security_name: Self::parse_string(record.get(3)),
            account: Self::parse_string(record.get(4)),
            shares: Self::parse_int(record.get(7)),
            asked_price: Self::parse_float(record.get(8)),
            proceeds: Self::parse_int(record.get(9)),
            purchase_price: Self::parse_float(record.get(10)),
            realized_profit_and_loss: Self::parse_int(record.get(11)),
            total_realized_profit_and_loss: None,
            withholding_tax: None,
            profit_and_loss: None,
        }
    }

    fn get_date(&self) -> Option<NaiveDate> {
        self.trade_date
    }

    fn get_all_fields(&self) -> Vec<(&'static str, Option<String>)> {
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

    fn get_profit_record(items: &[Self]) -> Self {
        let (specific_account_total, nisa_account_total) = items
            .iter()
            .filter_map(|profit_and_loss| {
                Some((
                    profit_and_loss.account.as_deref()?,
                    profit_and_loss.realized_profit_and_loss?,
                ))
            })
            .fold(
                (0, 0),
                |(specific, nisa), (account, realized_profit_and_loss)| {
                    if account.contains("特定") {
                        (specific + realized_profit_and_loss, nisa)
                    } else {
                        (specific, nisa + realized_profit_and_loss)
                    }
                },
            );

        let withholding_tax = ((specific_account_total.max(0) as f64) * TAX_RATE) as u32;
        let total = specific_account_total + nisa_account_total;

        Self {
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
