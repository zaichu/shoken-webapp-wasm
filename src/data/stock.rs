use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Clone, PartialEq, Deserialize, Serialize, Default, Debug)]
pub struct StockData {
    pub date: String,
    pub code: String,
    pub name: String,
    pub market_category: String,
    pub industry_code_33: Option<String>,
    pub industry_category_33: Option<String>,
    pub industry_code_17: Option<String>,
    pub industry_category_17: Option<String>,
    pub size_code: Option<String>,
    pub size_category: Option<String>,
}

impl StockData {
    pub fn new(
        date: &str,
        code: &str,
        name: &str,
        market_category: &str,
    ) -> Self {
        Self {
            date: date.to_string(),
            code: code.to_string(),
            name: name.to_string(),
            market_category: market_category.to_string(),
            ..Default::default()
        }
    }
    
    pub fn with_industry(
        mut self,
        industry_code_33: Option<String>,
        industry_category_33: Option<String>,
        industry_code_17: Option<String>,
        industry_category_17: Option<String>,
    ) -> Self {
        self.industry_code_33 = industry_code_33;
        self.industry_category_33 = industry_category_33;
        self.industry_code_17 = industry_code_17;
        self.industry_category_17 = industry_category_17;
        self
    }
    
    pub fn with_size(
        mut self,
        size_code: Option<String>,
        size_category: Option<String>,
    ) -> Self {
        self.size_code = size_code;
        self.size_category = size_category;
        self
    }
}

impl fmt::Display for StockData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}] {} ({})", self.code, self.name, self.market_category)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_stock_data_new() {
        let stock = StockData::new(
            "2023-04-01",
            "1234",
            "テスト株式会社",
            "東証プライム",
        );
        
        assert_eq!(stock.date, "2023-04-01");
        assert_eq!(stock.code, "1234");
        assert_eq!(stock.name, "テスト株式会社");
        assert_eq!(stock.market_category, "東証プライム");
        assert_eq!(stock.industry_code_33, None);
    }
    
    #[test]
    fn test_stock_data_with_industry() {
        let stock = StockData::new(
            "2023-04-01",
            "1234",
            "テスト株式会社",
            "東証プライム",
        )
        .with_industry(
            Some("101".to_string()),
            Some("食料品".to_string()),
            Some("10".to_string()),
            Some("消費財".to_string()),
        );
        
        assert_eq!(stock.industry_code_33, Some("101".to_string()));
        assert_eq!(stock.industry_category_33, Some("食料品".to_string()));
        assert_eq!(stock.industry_code_17, Some("10".to_string()));
        assert_eq!(stock.industry_category_17, Some("消費財".to_string()));
    }
    
    #[test]
    fn test_stock_data_with_size() {
        let stock = StockData::new(
            "2023-04-01",
            "1234",
            "テスト株式会社",
            "東証プライム",
        )
        .with_size(
            Some("LARGE".to_string()),
            Some("大型株".to_string()),
        );
        
        assert_eq!(stock.size_code, Some("LARGE".to_string()));
        assert_eq!(stock.size_category, Some("大型株".to_string()));
    }
    
    #[test]
    fn test_stock_data_display() {
        let stock = StockData::new(
            "2023-04-01",
            "1234",
            "テスト株式会社",
            "東証プライム",
        );
        
        assert_eq!(format!("{}", stock), "[1234] テスト株式会社 (東証プライム)");
    }
}
