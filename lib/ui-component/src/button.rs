use leptos::{component, ev::MouseEvent, prelude::*};
use ui_headless::button::use_button;

/// Create button. This button is `icon-based` button, the label is bottom of the button, with fits icon size.
#[component]
pub fn Button(
    #[prop(optional)] disabled: Option<bool>,
    #[prop(optional)] tabindex: Option<i32>,
    #[prop(optional)] on_click: Option<Callback<MouseEvent>>,
    #[prop(optional)] icon: Option<impl IntoView>,
    #[prop(optional)] label: Option<ReadSignal<String>>,
) -> impl IntoView {
    let state = use_button(disabled.unwrap_or(false));

    let attrs = (*state.attrs).get();
    view! {
        <button
            disabled=*attrs.disabled
            tabindex=tabindex
            on:click=move |ev| {
                let Some(handler) = on_click else {
                    return;
                };
                handler.run(ev)
            }
            class="inline-flex flex-col items-center w-fit"
        >
            {icon}
            <Show when=move || label.is_some()>
                <span class="overflow-hidden h-[16px]">{label.unwrap().get()}</span>
            </Show>
        </button>
    }
}
