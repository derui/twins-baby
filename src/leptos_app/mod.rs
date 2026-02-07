mod resize_nob;
#[cfg(test)]
mod test_leptos;
mod use_resize;
mod use_perspective;

use leptos::prelude::*;
use leptos_bevy_canvas::prelude::*;

use crate::{
    bevy_app::init_bevy_app,
    events::{CanvasResizeEvent, LoggingEvent}, leptos_app::resize_nob::NOB_AREA,
};
use resize_nob::{ResizeXNob, ResizeYNob};
use use_resize::use_resize;

const DEAD_ZONES: u32 = NOB_AREA * 2 + 20 * 2;

/// Builds a CSS grid-template-columns property with dynamic sizes.
fn build_grid_cols_css(first: Signal<u32>, third: Signal<u32>) -> Signal<String> {
    Signal::derive(move || {
        format!(
            "{}px 16px minmax(600px, 1fr) 16px {}px",
            first.get(),
            third.get()
        )
    })
}

/// Builds a CSS grid-template-rows property with dynamic sizes.
fn build_grid_rows_css(first: Signal<u32>, third: Signal<u32>) -> Signal<String> {
    Signal::derive(move || {
        format!(
            "{}px 16px minmax(480px, 1fr) 16px {}px",
            first.get(),
            third.get()
        )
    })
}

#[component]
pub fn App() -> impl IntoView {
    // Get initial window dimensions
    let (resize_sender, receiver) = message_l2b::<CanvasResizeEvent>();
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

    Effect::new(move || {
        let width = col_resize.sizes.1;
        let height = row_resize.sizes.1;

        let _ = resize_sender.send(CanvasResizeEvent {
            width: width.get() - DEAD_ZONES,
            height: height.get() - DEAD_ZONES,
        });
    });

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
    let grid_cols_css = build_grid_cols_css(col_resize.sizes.0, col_resize.sizes.2);
    let grid_rows_css = build_grid_rows_css(row_resize.sizes.0, row_resize.sizes.2);

    let grid_style = move || {
        format!(
            "grid-template-columns: {}; grid-template-rows: {};",
            grid_cols_css.get(),
            grid_rows_css.get()
        )
    };

    view! {
        <div class="grid items-center p-5 mx-auto h-full w-full" style=grid_style>
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
                resize_sender=receiver
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
    resize_sender: BevyMessageReceiver<CanvasResizeEvent>,
) -> impl IntoView {
    let (_log_receiver, log_sender) = message_b2l::<LoggingEvent>();

    view! {
        <FeatureIsland />

        <div class="relative h-full">
            <ResizeXNob movement=set_col_first_move />
        </div>

        <BevyCanvas init=move || { init_bevy_app(log_sender, resize_sender) } {..} />

        <div class="relative h-full">
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
