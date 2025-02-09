use crate::models::stock::StockData;
use gloo::utils::format::JsValueSerdeExt;
use thiserror::Error;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("Request failed")]
    RequestError,
    #[error("No window object")]
    NoWindowObject,
    #[error("Fetch error")]
    FetchError,
    #[error("Response error")]
    ResponseError,
    #[error("JSON error")]
    JsonError,
    #[error("Deserialization error")]
    DeserializationError,
}

/// 株式データを API から取得する関数。
///
/// - `code`: 銘柄コード (例: "7203")
/// - 成功時: `StockData` を返す。
/// - 失敗時: `ApiError` を返す。
///
/// ```rust
/// let stock = fetch_stock_data("7203").await?;
/// println!("{:?}", stock);
/// ```
pub async fn fetch_stock_data(code: &str) -> Result<StockData, ApiError> {
    let url = format!("https://shoken-webapp-api-b4a1.shuttle.app/stock/{}", code);
    let opts = RequestInit::new();
    opts.set_method("GET");
    opts.set_mode(RequestMode::Cors);
    let request =
        Request::new_with_str_and_init(&url, &opts).map_err(|_| ApiError::RequestError)?;
    let window = web_sys::window().ok_or(ApiError::NoWindowObject)?;
    let response = JsFuture::from(window.fetch_with_request(&request))
        .await
        .map_err(|_| ApiError::FetchError)?;
    let response: Response = response.dyn_into().map_err(|_| ApiError::ResponseError)?;
    let json = JsFuture::from(response.json().map_err(|_| ApiError::JsonError)?)
        .await
        .map_err(|_| ApiError::JsonError)?;

    json.into_serde::<StockData>()
        .map_err(|_| ApiError::DeserializationError)
}
