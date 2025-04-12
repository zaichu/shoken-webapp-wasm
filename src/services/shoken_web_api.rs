use gloo::utils::format::JsValueSerdeExt;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};

use crate::data::stock::StockData;
use crate::errors::{AppError, AppResult};
use crate::setting::*;

async fn fetch_json(url: &str) -> AppResult<JsValue> {
    let window = web_sys::window().ok_or_else(|| AppError::ApiError("ウィンドウオブジェクトが見つかりません".to_string()))?;
    let request = create_request(url)?;

    let response = JsFuture::from(window.fetch_with_request(&request))
        .await
        .map_err(|e| AppError::NetworkError(format!("{:?}", e)))?;
    let response: Response = response.dyn_into()
        .map_err(|_| AppError::ApiError("レスポンスが不正です".to_string()))?;

    JsFuture::from(response.json()
        .map_err(|_| AppError::ParseError("JSONパースに失敗しました".to_string()))?)
        .await
        .map_err(|e| AppError::ParseError(format!("JSONデータの取得に失敗しました: {:?}", e)))
}

fn create_request(url: &str) -> AppResult<Request> {
    let opts = RequestInit::new();
    opts.set_method("GET");
    opts.set_mode(RequestMode::Cors);

    Request::new_with_str_and_init(url, &opts)
        .map_err(|_| AppError::ApiError("リクエストの作成に失敗しました".to_string()))
}

pub async fn fetch_stock_data(code: &str) -> AppResult<StockData> {
    let url = format!("{}/stock/{}", SHOKEN_WEB_API_URL, code);
    let json = fetch_json(&url).await?;

    json.into_serde::<StockData>()
        .map_err(|e| AppError::ParseError(format!("データのデシリアライズに失敗しました: {}", e)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    #[test]
    fn test_format_api_url() {
        let code = "7203";
        let expected_url = format!("{}/stock/{}", SHOKEN_WEB_API_URL, code);
        assert!(expected_url.contains("/stock/7203"));
    }
}
