use lazy_static::lazy_static;
use std::collections::{HashMap, HashSet};

type HeaderMap = HashMap<String, String>;

lazy_static! {
    pub static ref HEADERS: HeaderMap = {
        let mut map = HashMap::new();
        map.insert("trade_date".to_string(), "約定日".to_string());
        map.insert("settlement_date".to_string(), "受渡日".to_string());
        map.insert("security_code".to_string(), "銘柄コード".to_string());
        map.insert("security_name".to_string(), "銘柄名".to_string());
        map.insert("account".to_string(), "口座".to_string());
        map.insert("shares".to_string(), "数量[株]".to_string());
        map.insert("asked_price".to_string(), "売却/決済単価".to_string());
        map.insert("proceeds".to_string(), "売却/決済額".to_string());
        map.insert("purchase_price".to_string(), "平均取得価額".to_string());
        map.insert(
            "realized_profit_and_loss".to_string(),
            "実現損益".to_string(),
        );
        map.insert(
            "total_realized_profit_and_loss".to_string(),
            "合計実現損益".to_string(),
        );
        map.insert("withholding_tax".to_string(), "源泉徴収税額".to_string());
        map.insert("profit_and_loss".to_string(), "損益".to_string());
        map.insert("product".to_string(), "商品".to_string());
        map.insert("currency".to_string(), "受取通貨".to_string());
        map.insert("unit_price".to_string(), "単価".to_string());
        map.insert(
            "dividends_before_tax".to_string(),
            "配当・分配金(税引前)".to_string(),
        );
        map.insert("taxes".to_string(), "税額".to_string());
        map.insert("net_amount_received".to_string(), "受取金額".to_string());
        map.insert(
            "total_dividends_before_tax".to_string(),
            "配当・分配金合計(税引前)".to_string(),
        );
        map.insert("total_taxes".to_string(), "税額合計".to_string());
        map.insert(
            "total_net_amount_received".to_string(),
            "受取金額".to_string(),
        );
        map
    };

    pub static ref NUMBER_FORMAT_KEYS: HashSet<&'static str> = [
        "shares",                         // 数量
    ].iter().cloned().collect();


    pub static ref YEN_FORMAT_KEYS: HashSet<&'static str> = [
        "asked_price",                    // 売却/決済単価
        "dividends_before_tax",           // 配当・分配金(税引前)
        "net_amount_received",            // 受取金額
        "proceeds",                       // 売却/決済額
        "profit_and_loss",                // 損益
        "purchase_price",                 // 平均取得価額
        "realized_profit_and_loss",       // 実現損益
        "taxes",                          // 税額
        "total_dividends_before_tax",     // 配当・分配金合計(税引前)
        "total_net_amount_received",      // 受取金額合計
        "total_realized_profit_and_loss", // 合計実現損益
        "total_taxes",                    // 税額合計
        "withholding_tax",                // 源泉徴収税額
    ].iter().cloned().collect();

    pub static ref DATE_FORMAT_KEYS: HashSet<&'static str> = [
        "settlement_date",                // 受取金
        "trade_date",                     // 約定日
    ].iter().cloned().collect();
}

// tax_rateの定義
pub const TAX_RATE: f64 = 0.20315;
