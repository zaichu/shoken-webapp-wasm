use yew::prelude::*;
use web_sys::HtmlInputElement;

#[derive(Properties, PartialEq, Clone)]
pub struct InputFieldProps {
    #[prop_or_default]
    pub id: String,
    #[prop_or_default]
    pub label: Option<String>,
    #[prop_or_default]
    pub error: Option<String>,
    #[prop_or(false)]
    pub full_width: bool,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub value: String,
    #[prop_or(String::from("text"))]
    pub r#type: String,
    #[prop_or_default]
    pub placeholder: String,
    #[prop_or_default]
    pub oninput: Callback<InputEvent>,
    #[prop_or(false)]
    pub disabled: bool,
    #[prop_or_default]
    pub name: String,
}

#[function_component]
pub fn InputField(props: &InputFieldProps) -> Html {
    let input_id = if !props.id.is_empty() {
        props.id.clone()
    } else {
        format!("input-{}", gloo::utils::format::random_id())
    };

    let base_classes = classes!("form-control");
    let error_class = if props.error.is_some() {
        classes!("is-invalid")
    } else {
        classes!()
    };
    let width_class = if props.full_width {
        classes!("w-100")
    } else {
        classes!()
    };

    let combined_classes = classes!(
        base_classes,
        error_class,
        width_class.clone(),
        props.class.clone()
    );

    html! {
        <div class={width_class}>
            {
                if let Some(label) = &props.label {
                    html! {
                        <label for={input_id.clone()} class="form-label">{ label }</label>
                    }
                } else {
                    html! {}
                }
            }
            <input
                id={input_id}
                class={combined_classes}
                type={props.r#type.clone()}
                value={props.value.clone()}
                placeholder={props.placeholder.clone()}
                oninput={props.oninput.clone()}
                disabled={props.disabled}
                name={props.name.clone()}
            />
            {
                if let Some(error) = &props.error {
                    html! {
                        <div class="invalid-feedback">{ error }</div>
                    }
                } else {
                    html! {}
                }
            }
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn input_field_renders_with_default_props() {
        let props = InputFieldProps {
            id: "".to_string(),
            label: None,
            error: None,
            full_width: false,
            class: classes!(),
            value: "".to_string(),
            r#type: "text".to_string(),
            placeholder: "".to_string(),
            oninput: Callback::from(|_| {}),
            disabled: false,
            name: "".to_string(),
        };

        let _rendered = InputField(&props);
        // In a real test, we would assert on the rendered HTML
    }

    #[wasm_bindgen_test]
    fn input_field_renders_with_label_and_error() {
        let props = InputFieldProps {
            id: "test-input".to_string(),
            label: Some("Test Label".to_string()),
            error: Some("This field is required".to_string()),
            full_width: true,
            class: classes!("custom-class"),
            value: "Test Value".to_string(),
            r#type: "text".to_string(),
            placeholder: "Enter text...".to_string(),
            oninput: Callback::from(|_| {}),
            disabled: false,
            name: "test".to_string(),
        };

        let _rendered = InputField(&props);
        // In a real test, we would assert on the rendered HTML
    }
}
