use chrono::NaiveDate;

use anyhow::{anyhow, Result};
use csv::StringRecord;
use encoding_rs::SHIFT_JIS;

pub trait Common {
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

    fn parse_date(&self, date_str: Option<&str>) -> Option<NaiveDate> {
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

    fn parse_int(&self, num_str: Option<&str>) -> Option<i32> {
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

    fn parse_float(&self, num_str: Option<&str>) -> Option<f64> {
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

    fn parse_string(&self, value: Option<&str>) -> Option<String> {
        match value {
            Some(s) => Some(s.to_string()),
            None => None,
        }
    }
}
