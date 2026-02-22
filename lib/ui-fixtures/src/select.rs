use leptos::prelude::*;

use ui_component::select::SelectBox;

fn fruit_item_view(item: String) -> AnyView {
    view! {
        <div class="w-full px-3 py-2 text-white bg-gray-800 hover:bg-gray-700 transition-colors">
            {item}
        </div>
    }
    .into_any()
}

fn fruit_selected_view(sel: Option<String>) -> AnyView {
    view! {
        <span class="text-white border-white/10 bg-black/50 hover:bg-black/70 backdrop-blur-md">
            {sel.unwrap_or("Select a fruit...".to_string())}
        </span>
    }
    .into_any()
}

#[component]
pub fn SelectFixtures() -> impl IntoView {
    let items = vec![
        "Apple".to_string(),
        "Banana".to_string(),
        "Cherry".to_string(),
        "Date".to_string(),
    ];

    let items2 = items.clone();

    view! {
        <div class="flex flex-col gap-8 p-8 bg-gray-900 min-h-screen">
            <div data-fixture="select-default">
                <h3 class="text-white mb-2">"Default (no selection)"</h3>
                <SelectBox
                    items=items
                    item_view=fruit_item_view
                    selected_view=fruit_selected_view
                    button_class="border border-white/10 bg-black/50 hover:bg-black/70 backdrop-blur-md rounded-md px-3 py-2"
                />
            </div>

            <div data-fixture="select-with-initial">
                <h3 class="text-white mb-2">"With initial selection"</h3>
                <SelectBox
                    items=items2
                    item_view=fruit_item_view
                    selected_view=fruit_selected_view
                    initial_selected="Cherry".to_string()
                    button_class="border border-white/10 bg-black/50 hover:bg-black/70 backdrop-blur-md rounded-md px-3 py-2"
                />
            </div>
        </div>
    }
}
