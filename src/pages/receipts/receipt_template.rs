use chrono::NaiveDate;
use csv::StringRecord;
use gloo::console;
use itertools::Itertools;
use std::ops::Not;
use wasm_bindgen_futures::spawn_local;
use web_sys::{File, HtmlInputElement};
use yew::{prelude::*, virtual_dom::VNode};

use crate::{services::*, setting::*};

#[derive(Properties, PartialEq, Debug, Clone)]
pub struct ReceiptTemplateProps {
    pub name: String,
}

#[function_component]
pub fn ReceiptTemplate<T: ReceiptProps>(props: &ReceiptTemplateProps) -> Html {
    let receipts = use_state(Vec::<T>::new);
    let csv_file = use_state(|| None::<File>);
    let file_name = use_state(String::new);
    let query = use_state(|| None::<String>);

    {
        let file_name = file_name.clone();
        let receipts = receipts.clone();

        use_effect_with((*csv_file).clone(), move |csv_file| {
            if let Some(file) = csv_file.clone() {
                process_uploaded_file(file, file_name, receipts);
            } else {
                file_name.set(String::new());
            }
        });
    }

    html! {
        <>
            { render_file_input(csv_file.clone(), file_name.clone()) }
            <div class="mt-2">
                <table class="table table-bordered">{ T::view_summary(&(*receipts)) }</table>
            </div>
            { render_receipt_card::<T>(props, &receipts, &query, csv_file.is_some()) }
        </>
    }
}

fn render_file_input(
    csv_file: UseStateHandle<Option<File>>,
    file_name: UseStateHandle<String>,
) -> Html {
    let on_input = Callback::from(move |e: InputEvent| {
        let input: HtmlInputElement = e.target_unchecked_into();
        let value = input.files().and_then(|files| files.get(0));
        csv_file.set(value.clone());
    });

    html! {
        <div class="input-group">
            <label class="input-group-btn" for="csv-file-input">
                <span class="btn bg-info text-white">{ "CSVファイル選択" }</span>
            </label>
            <input 
                id="csv-file-input" 
                type="file" 
                accept=".csv" 
                style="display:none" 
                oninput={on_input} 
            />
            <input 
                type="text" 
                class="form-control form-control-sm" 
                readonly=true 
                placeholder="CSVファイルを選択してください。" 
                value={(*file_name).clone()} 
            />
        </div>
    }
}

fn render_receipt_card<T: ReceiptProps>(
    props: &ReceiptTemplateProps,
    receipts: &UseStateHandle<Vec<T>>,
    query: &UseStateHandle<Option<String>>,
    has_file: bool,
) -> Html {
    html! {
        <div class="card shadow-sm">
            <div class="card-header bg-info text-white">
                <div class="row align-items-center">
                    <div class="col col-lg-1"><h5 class="mb-0">{ props.name.clone() }</h5></div>
                    if T::is_view_search() {
                        <div class="col col-md-auto"><h6 class="mb-0">{ "銘柄コード:" }</h6></div>
                        <div class="col col-lg-2">{ render_search_dropdown::<T>(&*receipts, query) }</div>
                    }
                </div>
            </div>
            <div class="table-responsive" style="max-height: 500px;">
                <table class="table table-bordered">
                    { render_table_header::<T>() }
                    if has_file {
                        { render_table_body::<T>(&*receipts, &*query) }
                    }
                </table>
            </div>
        </div>
    }
}

fn render_search_dropdown<T: ReceiptProps>(
    receipts: &[T],
    query: &UseStateHandle<Option<String>>,
) -> Html {
    let query_clone = query.clone();
    let on_input = Callback::from(move |e: InputEvent| {
        let input: HtmlInputElement = e.target_unchecked_into();
        let value = input.value();
        query_clone.set(value.is_empty().not().then_some(value));
    });

    html! {
        <select class="form-select form-select-sm" oninput={on_input}>
            <option selected=true />
            {
                receipts
                    .iter()
                    .map(|receipt| { (receipt.get_security_code().to_string(), receipt) })
                    .sorted_by(|(a, _), (b, _)| a.cmp(b))
                    .chunk_by(|(key, _)| key.clone())
                    .into_iter()
                    .map(|(security_code, group)| {
                        let security_name = group.max_by_key(|(_, r)| r.get_date().unwrap_or_default())
                                                .map(|(_, r)| r.get_security_name().to_string())
                                                .unwrap_or_default();

                        if security_code.is_empty() {
                            html! { <option value={security_name.clone()}>{security_name}</option> }
                        } else {
                            html! { <option value={security_code.clone()}>{format!("{}: {}", security_code, security_name)}</option> }
                        }
                    })
                    .collect::<Vec<VNode>>()
            }
        </select>
    }
}

fn render_table_header<T: ReceiptProps>() -> Html {
    html! {
        <thead class="thead-light">
            <tr>
            {
                for T::new().get_all_fields().iter().map(|(header, _)| {
                    let header_text = HEADERS.get(header).unwrap_or(header);
                    html! {
                        <th scope="col" style="position: sticky; top: 0; background-color: white; white-space: nowrap; text-align: center;">
                            { header_text }
                        </th>
                    }
                })
            }
            </tr>
        </thead>
    }
}

fn render_table_body<T: ReceiptProps>(receipts: &[T], query: &Option<String>) -> Html {
    html! {
        <tbody>
        {
            receipts
                .iter()
                .filter_map(|receipt| {
                    match query {
                        Some(q) => receipt.search(q).then(|| (q.to_string(), receipt)),
                        None => receipt.get_date().map(|date| (date.to_string(), receipt)),
                    }
                })
                .chunk_by(|(key, _)| key.clone())
                .into_iter()
                .flat_map(|(_, group)| {
                    let receipts: Vec<&T> = group.map(|(_, receipt)| receipt).collect();
                    let mut views: Vec<Html> = receipts.iter().map(|r| r.view(None)).collect();
                    if let Some(summary) = T::new_summary(&receipts) {
                        views.push(summary.view(Some("table-success".to_string())));
                    }
                    views
                })
                .collect::<Vec<VNode>>()
        }
        </tbody>
    }
}

fn process_uploaded_file<T: ReceiptProps>(
    file: File, 
    file_name: UseStateHandle<String>,
    receipts: UseStateHandle<Vec<T>>,
) {
    spawn_local(async move {
        file_name.set(file.name());
        match process_file_async(&file).await {
            Ok(new_receipts) => {
                receipts.set(new_receipts);
            }
            Err(err) => {
                console::error!("ファイル処理エラー:", err.to_string());
            }
        }
    });
}

async fn process_file_async<T: ReceiptProps>(file: &File) -> Result<Vec<T>, String> {
    let content = csv_reader::read_file(file)
        .await
        .map_err(|e| format!("ファイル読み込みエラー: {}", e))?;
    
    let records = csv_reader::read_csv(content)
        .map_err(|e| format!("CSV解析エラー: {}", e))?;

    let new_receipts = records
        .into_iter()
        .map(|record| T::new_from_string_record(record))
        .sorted_by(|a, b| {
            a.get_date()
                .unwrap_or_default()
                .cmp(&b.get_date().unwrap_or_default())
        })
        .collect();

    Ok(new_receipts)
}

pub trait ReceiptProps: Clone + Sized + PartialEq + Default + 'static {
    fn new() -> Self;
    
    fn new_summary(_receipts: &[&Self]) -> Option<Self> {
        None
    }
    
    fn new_from_string_record(record: StringRecord) -> Self;

    fn get_all_fields(&self) -> Vec<(&'static str, Option<String>)>;
    
    fn get_date(&self) -> Option<NaiveDate>;

    fn get_security_code(&self) -> &str {
        ""
    }

    fn get_security_name(&self) -> &str {
        ""
    }

    fn search(&self, _query: &str) -> bool {
        true
    }

    fn is_view_search() -> bool {
        true
    }

    fn view_summary(receipts: &[Self]) -> Html;

    fn view(&self, tr_class: Option<String>) -> Html {
        html! {
            <tr class={tr_class}>
                { for self.get_all_fields().iter().map(|(key, value)| {
                    let value = value.as_deref().unwrap_or("");
                    let value = formater::format_value(key, value);
                    let style = "overflow-wrap: break-word; white-space: normal;";
                    let mut class = "text-nowrap".to_string();
                    if value.starts_with("¥ -") {
                        class = format!("{} text-danger", class);
                    }
                    html! {
                        <td class={class} style={style}>
                            {value}
                        </td>
                    }
                })}
            </tr>
        }
    }

    fn render_summary_th_td(key: &str, value: i32) -> Html {
        let style = "max-width: 30px;";
        let mut class = "text-nowrap".to_string();
        let value = &format!("{value}");
        let value = formater::format_value(key, value);
        if value.starts_with("¥ -") {
            class = format!("{} text-danger", class);
        }
        html! {
        <>
            <th class="bg-info text-white text-nowrap" style="max-width: 20px;">{HEADERS.get(key).unwrap_or(&key)}</th>
            <td class={class} style={style}>{value}</td>
        </>
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use csv::StringRecord;
    use std::str::FromStr;

    #[derive(Clone, PartialEq, Debug, Default)]
    struct MockReceipt {
        date: Option<NaiveDate>,
        name: Option<String>,
        amount: Option<i32>,
        security_code: Option<String>,
        security_name: Option<String>,
    }

    impl ReceiptProps for MockReceipt {
        fn new() -> Self {
            Self::default()
        }

        fn new_from_string_record(record: StringRecord) -> Self {
            Self {
                date: record.get(0).and_then(|d| NaiveDate::from_str(d).ok()),
                name: record.get(1).map(String::from),
                amount: record.get(2).and_then(|a| a.parse().ok()),
                security_code: record.get(3).map(String::from),
                security_name: record.get(4).map(String::from),
            }
        }

        fn get_all_fields(&self) -> Vec<(&'static str, Option<String>)> {
            vec![
                ("date", self.date.map(|d| d.to_string())),
                ("name", self.name.clone()),
                ("amount", self.amount.map(|a| a.to_string())),
                ("security_code", self.security_code.clone()),
                ("security_name", self.security_name.clone()),
            ]
        }

        fn get_date(&self) -> Option<NaiveDate> {
            self.date
        }

        fn get_security_code(&self) -> &str {
            self.security_code.as_deref().unwrap_or("")
        }

        fn get_security_name(&self) -> &str {
            self.security_name.as_deref().unwrap_or("")
        }

        fn view_summary(_receipts: &[Self]) -> Html {
            html! { <tr><td>{"サンプルサマリー"}</td></tr> }
        }

        fn search(&self, query: &str) -> bool {
            self.security_code.as_deref() == Some(query)
        }
    }

    #[test]
    fn test_mock_receipt_implementation() {
        let mock = MockReceipt {
            date: Some(NaiveDate::from_ymd_opt(2023, 1, 1).unwrap()),
            name: Some("テスト".to_string()),
            amount: Some(1000),
            security_code: Some("1234".to_string()),
            security_name: Some("テスト証券".to_string()),
        };

        assert_eq!(mock.get_date(), Some(NaiveDate::from_ymd_opt(2023, 1, 1).unwrap()));
        assert_eq!(mock.get_security_code(), "1234");
        assert_eq!(mock.get_security_name(), "テスト証券");
        assert!(mock.search("1234"));
        assert!(!mock.search("5678"));
    }

    #[test]
    fn test_new_from_string_record() {
        let record = StringRecord::from(vec!["2023-01-01", "テスト", "1000", "1234", "テスト証券"]);
        let mock = MockReceipt::new_from_string_record(record);

        assert_eq!(mock.date, Some(NaiveDate::from_ymd_opt(2023, 1, 1).unwrap()));
        assert_eq!(mock.name, Some("テスト".to_string()));
        assert_eq!(mock.amount, Some(1000));
        assert_eq!(mock.security_code, Some("1234".to_string()));
        assert_eq!(mock.security_name, Some("テスト証券".to_string()));
    }

    #[test]
    fn test_get_all_fields() {
        let mock = MockReceipt {
            date: Some(NaiveDate::from_ymd_opt(2023, 1, 1).unwrap()),
            name: Some("テスト".to_string()),
            amount: Some(1000),
            security_code: Some("1234".to_string()),
            security_name: Some("テスト証券".to_string()),
        };

        let fields = mock.get_all_fields();
        assert_eq!(fields.len(), 5);
        assert_eq!(fields[0].0, "date");
        assert_eq!(fields[0].1, Some("2023-01-01".to_string()));
        assert_eq!(fields[1].0, "name");
        assert_eq!(fields[1].1, Some("テスト".to_string()));
    }
}
