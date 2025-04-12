use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct ButtonProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or(String::from("primary"))]
    pub variant: String,
    #[prop_or(String::from("md"))]
    pub size: String,
    #[prop_or(false)]
    pub full_width: bool,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
    #[prop_or(false)]
    pub disabled: bool,
    #[prop_or_default]
    pub r#type: String,
}

#[function_component]
pub fn Button(props: &ButtonProps) -> Html {
    let base_classes = classes!("btn");
    let variant_class = classes!(format!("btn-{}", props.variant));
    let size_class = if props.size != "md" {
        classes!(format!("btn-{}", props.size))
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
        variant_class,
        size_class,
        width_class,
        props.class.clone()
    );

    html! {
        <button
            class={combined_classes}
            onclick={props.onclick.clone()}
            disabled={props.disabled}
            type={props.r#type.clone()}
        >
            { for props.children.iter() }
        </button>
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn button_renders_with_default_props() {
        let props = ButtonProps {
            children: Children::new(vec![html! { "Test Button" }]),
            variant: "primary".to_string(),
            size: "md".to_string(),
            full_width: false,
            class: classes!(),
            onclick: Callback::from(|_| {}),
            disabled: false,
            r#type: "button".to_string(),
        };

        let _rendered = Button(&props);
        // In a real test, we would assert on the rendered HTML
    }
}
