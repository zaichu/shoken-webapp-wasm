use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("API エラー: {0}")]
    ApiError(String),
    
    #[error("認証エラー: {0}")]
    AuthError(String),
    
    #[error("パースエラー: {0}")]
    ParseError(String),
    
    #[error("CSV読み込みエラー: {0}")]
    CsvError(String),
    
    #[error("ネットワークエラー: {0}")]
    NetworkError(String),
    
    #[error("不明なエラー: {0}")]
    UnknownError(String),
}

pub type AppResult<T> = Result<T, AppError>;

impl From<serde_json::Error> for AppError {
    fn from(err: serde_json::Error) -> Self {
        AppError::ParseError(err.to_string())
    }
}

impl From<gloo_net::Error> for AppError {
    fn from(err: gloo_net::Error) -> Self {
        AppError::NetworkError(err.to_string())
    }
}

impl From<csv::Error> for AppError {
    fn from(err: csv::Error) -> Self {
        AppError::CsvError(err.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_error_display() {
        let error = AppError::ApiError("テストエラー".to_string());
        assert_eq!(error.to_string(), "API エラー: テストエラー");

        let error = AppError::AuthError("認証失敗".to_string());
        assert_eq!(error.to_string(), "認証エラー: 認証失敗");
    }

    #[test]
    fn test_from_serde_json_error() {
        let json_err = serde_json::from_str::<serde_json::Value>("invalid json").unwrap_err();
        let app_err = AppError::from(json_err);
        
        match app_err {
            AppError::ParseError(_) => assert!(true),
            _ => panic!("Expected ParseError"),
        }
    }

    #[test]
    fn test_app_result() {
        let result: AppResult<i32> = Ok(42);
        assert_eq!(result.unwrap(), 42);

        let result: AppResult<()> = Err(AppError::UnknownError("テスト".to_string()));
        assert!(result.is_err());
    }
}
