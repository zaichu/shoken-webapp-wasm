use web_sys::HtmlInputElement;
use yew::prelude::*;

use anyhow::{anyhow, Result};
use csv::StringRecord;
use encoding_rs::SHIFT_JIS;

pub trait TemplateModel {
    fn read_csv(bytes: Vec<u8>) -> Result<Vec<StringRecord>> {
        let (cow, _, had_errors) = SHIFT_JIS.decode(&bytes);
        if had_errors {
            return Err(anyhow!("Error decoding Shift-JIS"));
        }
        let utf8_string = cow.into_owned();
        let mut rdr = csv::ReaderBuilder::new()
            .has_headers(true)
            .from_reader(utf8_string.as_bytes());

        let mut result = Vec::new();
        for record in rdr.records() {
            result.push(record?);
        }
        Ok(result)
    }

    fn format_value(&self, key: &str, value: &str) -> String {
        match key {
            "settlement_date" | "trade_date" => self.format_date(value),
            "asked_price"
            | "dividends_before_tax"
            | "net_amount_received"
            | "proceeds"
            | "profit_and_loss"
            | "purchase_price"
            | "realized_profit_and_loss"
            | "shares"
            | "taxes"
            | "total_dividends_before_tax"
            | "total_net_amount_received"
            | "total_realized_profit_and_loss"
            | "total_taxes"
            | "withholding_tax" => self.format_number(value),
            _ => value.to_string(),
        }
    }

    fn format_date(&self, s: &str) -> String {
        s.replace("-", "/")
    }

    fn format_number(&self, s: &str) -> String {
        let is_negative = s.starts_with('-');
        let s = if is_negative { &s[1..] } else { s };
        let s = s.replace(",", ""); // カンマを除去
        let parts: Vec<&str> = s.split('.').collect();
        let mut result = String::new();
        for (i, c) in parts[0].chars().rev().enumerate() {
            if i > 0 && i % 3 == 0 {
                result.push(',');
            }
            result.push(c);
        }
        result = result.chars().rev().collect();
        if parts.len() > 1 {
            result.push('.');
            result.push_str(parts[1]);
        }
        if is_negative {
            format!("-{result}")
        } else {
            result
        }
    }
}

#[function_component(Template)]
pub fn template(props: &yew::html::ChildrenProps) -> Html {
    let csv_file = use_state(|| String::new());

    let on_change = {
        let csv_file = csv_file.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            if let Some(files) = input.files() {
                if let Some(file) = files.get(0) {
                    csv_file.set(file.name());
                }
            }
        })
    };

    let file_name = if csv_file.is_empty() {
        "CSVファイルを選択してください。".to_string()
    } else {
        (*csv_file).clone().to_string()
    };

    html! {
        <>
            <div class="input-group">
                <label class="input-group-btn">
                    <span class="btn btn-primary">
                        {"CSVファイル選択"}
                        <input type="file" accept=".csv" style="display:none" onchange={on_change} />
                    </span>
                </label>
                <input type="text" class="form-control" readonly=true value={file_name} />
            </div>
            <div class="mt-4">
                { for props.children.iter() }
            </div>
        </>
    }
}
