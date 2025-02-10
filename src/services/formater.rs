use crate::setting::*;

pub fn format_value(key: &str, value: &str) -> String {
    if YEN_FORMAT_KEYS.contains(key) {
        value.format_yen()
    } else if NUMBER_FORMAT_KEYS.contains(key) {
        value.format_number()
    } else if DATE_FORMAT_KEYS.contains(key) {
        value.format_date()
    } else {
        value.to_string()
    }
}

pub trait StrFormater {
    fn format_date(&self) -> String;
    fn format_number(&self) -> String;
    fn format_yen(&self) -> String;
}

impl StrFormater for &str {
    fn format_date(&self) -> String {
        self.replace("-", "/")
    }

    fn format_number(&self) -> String {
        let (sign, s) = self.strip_prefix('-').map_or(("", self), |_| ("-", self));
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
        format!(
            "{}{}{}",
            sign,
            integer_part,
            parts.get(1).map_or(String::new(), |&s| format!(".{}", s))
        )
    }

    fn format_yen(&self) -> String {
        if self.is_empty() {
            return "-".to_string();
        }
        format!("¥ {}", self.format_number())
    }
}
