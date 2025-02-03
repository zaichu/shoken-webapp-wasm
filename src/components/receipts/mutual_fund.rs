use chrono::NaiveDate;
use csv::StringRecord;
use yew::prelude::*;

use crate::setting::TAX_RATE;

use super::lib::ReceiptProps;

#[derive(PartialEq, Properties, Debug, Clone, Default)]
pub struct MutualFund {
    pub is_view: bool,
    pub trade_date: Option<NaiveDate>,                   // 約定日
    pub settlement_date: Option<NaiveDate>,              // 受渡日
    pub fund_name: Option<String>,                       // ファンド名
    pub dividends: Option<String>,                       // 分配金
    pub account: Option<String>,                         // 口座
    pub shares: Option<u32>,                             // 数量[株]
    pub exchange_rate: Option<u32>,                      // 為替レート［円］
    pub cancellation_unit_price_yen: Option<u32>,        // 解約単価［円］
    pub cancellation_amount_yen: Option<u32>,            // 解約額［円］
    pub average_acquisition_price_yen: Option<f64>,      // 平均取得価額［円］
    pub realized_profit_and_loss: Option<i32>,           // 実現損益［円］
    pub taxes: Option<i32>,                              // 税額
    pub realized_profit_and_loss_after_tax: Option<i32>, // 実現損益(税引)
}

impl ReceiptProps for MutualFund {
    fn new() -> Self {
        MutualFund::default()
    }

    fn from_string_record(record: StringRecord) -> Self {
        let tmp_account = Self::parse_string(record.get(4)).unwrap();
        let tmp_realized_profit_and_loss = Self::parse_i32(record.get(11));
        let (taxes, tmp_realized_profit_and_loss_after_tax) =
            tmp_realized_profit_and_loss.map_or((None, None), |profit| {
                if profit > 0 {
                    if tmp_account.contains("特定") {
                        let tmp_taxes = (profit as f64 * TAX_RATE) as i32;
                        (Some(tmp_taxes), Some(profit - tmp_taxes))
                    } else {
                        (Some(0), Some(profit))
                    }
                } else {
                    (Some(0), Some(profit))
                }
            });

        Self {
            is_view: true,
            trade_date: Self::parse_date(record.get(0)),
            settlement_date: Self::parse_date(record.get(1)),
            fund_name: Self::parse_string(record.get(2)),
            dividends: Self::parse_string(record.get(3)),
            account: Some(tmp_account),
            shares: Self::parse_u32(record.get(6)),
            exchange_rate: Self::parse_u32(record.get(7)),
            cancellation_unit_price_yen: Self::parse_u32(record.get(8)),
            cancellation_amount_yen: Self::parse_u32(record.get(9)),
            average_acquisition_price_yen: Self::parse_f64(record.get(10)),
            realized_profit_and_loss: tmp_realized_profit_and_loss,
            taxes: taxes,
            realized_profit_and_loss_after_tax: tmp_realized_profit_and_loss_after_tax,
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
            ("fund_name", self.fund_name.clone()),
            ("dividends", self.dividends.clone()),
            ("account", self.account.clone()),
            ("shares", self.shares.map(|s| s.to_string())),
            ("exchange_rate", self.exchange_rate.map(|s| s.to_string())),
            (
                "cancellation_unit_price_yen",
                self.cancellation_unit_price_yen.map(|s| s.to_string()),
            ),
            (
                "cancellation_amount_yen",
                self.cancellation_amount_yen.map(|s| s.to_string()),
            ),
            (
                "average_acquisition_price_yen",
                self.average_acquisition_price_yen.map(|s| s.to_string()),
            ),
            (
                "realized_profit_and_loss",
                self.realized_profit_and_loss.map(|s| s.to_string()),
            ),
            ("taxes", self.taxes.map(|s| s.to_string())),
            (
                "realized_profit_and_loss_after_tax",
                self.realized_profit_and_loss_after_tax
                    .map(|s| s.to_string()),
            ),
        ]
    }

    fn get_profit_record(items: &[Self]) -> Self {
        let item = items.first().unwrap().clone();
        Self {
            is_view: false,
            ..item
        }
    }

    fn view_summary(items: &[Self]) -> Html {
        let (total_realized_profit_and_loss, total_taxes, total_realized_profit_and_loss_after_tax) =
            items.iter().fold(
                (0, 0, 0),
                |(total_realized_profit_and_loss, withholding_tax, profit_and_loss), p| {
                    (
                        total_realized_profit_and_loss + p.realized_profit_and_loss.unwrap(),
                        withholding_tax + p.taxes.unwrap(),
                        profit_and_loss + p.realized_profit_and_loss_after_tax.unwrap(),
                    )
                },
            );

        html! {
            <tbody>
                <tr>
                    { Self::render_td_tr_summary("total_realized_profit_and_loss", total_realized_profit_and_loss) }
                    { Self::render_td_tr_summary("total_taxes", total_taxes as i32) }
                    { Self::render_td_tr_summary("total_realized_profit_and_loss_after_tax", total_realized_profit_and_loss_after_tax) }
                </tr>
            </tbody>
        }
    }

    fn is_view(&self) -> bool {
        self.is_view
    }
}
