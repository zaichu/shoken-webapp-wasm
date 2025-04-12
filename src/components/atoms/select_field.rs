use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub struct SelectOption {
    pub value: String,
    pub label: String,
}

#[derive(Properties, PartialEq)]
pub struct SelectFieldProps {
    #[prop_or_default]
    pub id: String,
    #[prop_or_default]
    pub label: Option<String>,
    pub options: Vec<SelectOption>,
    #[prop_or_default]
    pub error: Option<String>,
    #[prop_or(false)]
    pub full_width: bool,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub value: String,
    #[prop_or_default]
    pub onchange: Callback<Event>,
    #[prop_or(false)]
    pub disabled: bool,
    #[prop_or_default]
    pub name: String,
    #[prop_or_default]
    pub empty_option_label: Option<String>,
}

#[function_component]
pub fn SelectField(props: &SelectFieldProps) -> Html {
    let select_id = if !props.id.is_empty() {
        props.id.clone()
    } else {
        format!("select-{}", gloo::utils::format::random_id())
    };

    let base_classes = classes!("form-select");
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
        <div class={classes!("mb-3", width_class)}>
            {
                if let Some(label) = &props.label {
                    html! {
                        <label for={select_id.clone()} class="form-label">{ label }</label>
                    }
                } else {
                    html! {}
                }
            }
            <select
                id={select_id}
                class={combined_classes}
                value={props.value.clone()}
                onchange={props.onchange.clone()}
                disabled={props.disabled}
                name={props.name.clone()}
            >
                {
                    if let Some(empty_label) = &props.empty_option_label {
                        html! {
                            <option value="">{ empty_label }</option>
                        }
                    } else {
                        html! {}
                    }
                }
                {
                    props.options.iter().map(|option| {
                        html! {
                            <option value={option.value.clone()}>{ &option.label }</option>
                        }
                    }).collect::<Html>()
                }
            </select>
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
    fn select_field_renders_with_options() {
        let options = vec![
            SelectOption {
                value: "option1".to_string(),
                label: "Option 1".to_string(),
            },
            SelectOption {
                value: "option2".to_string(),
                label: "Option 2".to_string(),
            },
        ];

        let props = SelectFieldProps {
            id: "test-select".to_string(),
            label: Some("Select an option".to_string()),
            options,
            error: None,
            full_width: false,
            class: classes!(),
            value: "".to_string(),
            onchange: Callback::from(|_| {}),
            disabled: false,
            name: "test-select".to_string(),
            empty_option_label: Some("-- Select --".to_string()),
        };

        let _rendered = SelectField(&props);
        // In a real test, we would assert on the rendered HTML
    }
}
