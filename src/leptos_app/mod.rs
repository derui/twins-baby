mod resize_nob;
#[cfg(test)]
mod test_leptos;
mod use_resize;

use leptos::prelude::*;
use leptos_bevy_canvas::prelude::*;

use crate::{bevy_app::init_bevy_app, events::LoggingEvent};
use resize_nob::{ResizeXNob, ResizeYNob};
use use_resize::use_resize;

#[component]
pub fn App() -> impl IntoView {
    // Get initial window dimensions
    let initial_width = window()
        .inner_width()
        .ok()
        .and_then(|w: leptos::wasm_bindgen::JsValue| w.as_f64())
        .unwrap_or(1200.0) as u32;
    let initial_height = window()
        .inner_height()
        .ok()
        .and_then(|h: leptos::wasm_bindgen::JsValue| h.as_f64())
        .unwrap_or(800.0) as u32;

    let (window_width, _set_window_width) = signal(initial_width);
    let (window_height, _set_window_height) = signal(initial_height);

    // Initialize resize hooks for columns and rows
    let col_resize = use_resize((240, 240), window_width.into());
    let row_resize = use_resize((120, 120), window_height.into());

    // Create movement signals for nobs (i32)
    let (col_first_move, set_col_first_move) = signal(0i32);
    let (col_third_move, set_col_third_move) = signal(0i32);
    let (row_first_move, set_row_first_move) = signal(0i32);
    let (row_third_move, set_row_third_move) = signal(0i32);

    // Connect nob movements to resize hooks (convert i32 to Option<i32>)
    Effect::new(move || {
        let delta = col_first_move.get();
        if delta != 0 {
            col_resize.first_movement.set(Some(delta));
            set_col_first_move.set(0);
        }
    });

    Effect::new(move || {
        let delta = col_third_move.get();
        if delta != 0 {
            col_resize.third_movement.set(Some(delta));
            set_col_third_move.set(0);
        }
    });

    Effect::new(move || {
        let delta = row_first_move.get();
        if delta != 0 {
            row_resize.first_movement.set(Some(delta));
            set_row_first_move.set(0);
        }
    });

    Effect::new(move || {
        let delta = row_third_move.get();
        if delta != 0 {
            row_resize.third_movement.set(Some(delta));
            set_row_third_move.set(0);
        }
    });

    // Build grid templates with dynamic sizes
    let grid_cols_template = Signal::derive(move || {
        let (first, _, third) = col_resize.sizes;
        format!(
            "grid-cols-[{}px_16px_minmax(600px,1fr)_16px_{}px]",
            first.get(),
            third.get()
        )
    });

    let grid_rows_template = Signal::derive(move || {
        let (first, _, third) = row_resize.sizes;
        format!(
            "grid-rows-[{}px_16px_minmax(480px,1fr)_16px_{}px]",
            first.get(),
            third.get()
        )
    });

    let whole_class = move || {
        format!(
            "grid items-center p-5 mx-auto h-full w-full {} {}",
            grid_rows_template.get(),
            grid_cols_template.get()
        )
    };

    view! {
        <div class=whole_class>
            // Row 1: PerspectiveIsland (spans all 5 columns)
            <div class="col-span-5">
                <PerspectiveIsland />
            </div>

            // Row 2: Y nob between top and middle
            <div class="col-span-5 relative">
                <ResizeYNob movement=set_row_first_move />
            </div>

            // Row 3: Main content row with X nobs
            <CenterResizableRow
                set_col_first_move=set_col_first_move
                set_col_third_move=set_col_third_move
            />

            // Row 4: Y nob between middle and bottom
            <div class="col-span-5 relative">
                <ResizeYNob movement=set_row_third_move />
            </div>

            // Row 5: InfoIsland (spans all 5 columns)
            <div class="col-span-5">
                <InfoIsland />
            </div>
        </div>
    }
}

/// A component for the center resizable row with horizontal nobs.
#[component]
pub fn CenterResizableRow(
    set_col_first_move: WriteSignal<i32>,
    set_col_third_move: WriteSignal<i32>,
) -> impl IntoView {
    let (_log_receiver, log_sender) = message_b2l::<LoggingEvent>();

    view! {
        <FeatureIsland />

        <div class="relative">
            <ResizeXNob movement=set_col_first_move />
        </div>

        <BevyCanvas init=move || { init_bevy_app(log_sender) } {..} />

        <div class="relative">
            <ResizeXNob movement=set_col_third_move />
        </div>

        <SupportIsland />
    }
}

/// A component for perspective island.
#[component]
pub fn PerspectiveIsland() -> impl IntoView {
    view! { <div class="flex flex-row h-full w-full col-span-5"></div> }
}

/// A component for feature island.
#[component]
pub fn FeatureIsland() -> impl IntoView {
    view! { <div class="flex flex-col h-full w-full"></div> }
}

/// A component for support island.
#[component]
pub fn SupportIsland() -> impl IntoView {
    view! { <div class="flex flex-col h-full w-full"></div> }
}

/// A component for info island.
#[component]
pub fn InfoIsland() -> impl IntoView {
    view! { <div class="flex flex-row h-full w-full col-span-5"></div> }
}
