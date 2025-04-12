use csv::StringRecord;
use encoding_rs::SHIFT_JIS;
use wasm_bindgen_futures::JsFuture;
use web_sys::{js_sys, File};

use crate::errors::{AppError, AppResult};

pub async fn read_file(file: &File) -> AppResult<Vec<u8>> {
    let array_buffer = JsFuture::from(file.array_buffer())
        .await
        .map_err(|e| AppError::CsvError(format!("ファイル読み込み失敗: {:?}", e)))?;
    Ok(js_sys::Uint8Array::new(&array_buffer).to_vec())
}

pub fn read_csv(bytes: Vec<u8>) -> AppResult<Vec<StringRecord>> {
    let (cow, _, had_errors) = SHIFT_JIS.decode(&bytes);
    if had_errors {
        return Err(AppError::CsvError("デコードに失敗しました".to_string()));
    }
    
    let utf8_string = cow.into_owned();
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_reader(utf8_string.as_bytes());
        
    rdr.records()
        .collect::<Result<Vec<_>, csv::Error>>()
        .map_err(|e| AppError::CsvError(format!("CSV読み込み失敗: {}", e)))
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_csv_decode_error() {
        let invalid_bytes = vec![0xFF, 0xFF, 0xFF, 0xFF];
        
        let result = read_csv(invalid_bytes);
        
        assert!(result.is_err());
        
        if let Err(AppError::CsvError(msg)) = result {
            assert!(msg.contains("デコード"));
        } else {
            panic!("Expected CsvError");
        }
    }
}
