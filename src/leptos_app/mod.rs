mod app_state;
mod command_sender;
mod component;
mod resize_nob;
mod ui_action;
mod ui_state;
mod use_perspective;
mod use_resize;

use leptos::{context::Provider, prelude::*};
use leptos_bevy_canvas::prelude::*;
use ui_event::{
    PerspectiveKind,
    command::Commands,
    intent::{CanvasResizeIntent, Intents},
};

use crate::{
    bevy_app::{BevyAppSettings, init_bevy_app},
    leptos_app::{
        app_state::AppStore,
        command_sender::CommandSender,
        component::{FeatureIsland, InfoIsland, PerspectiveIsland, SupportIsland},
        resize_nob::NOB_AREA,
        ui_state::UiStore,
    },
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
    let (notification_sender, notification_receiver) = message_l2b::<Intents>();
    let (command_sender, _command_receiver) = message_l2b::<Commands>();
    let store = AppStore::new();
    provide_context(CommandSender::new(command_sender));
    provide_context(store);
    provide_context(UiStore::new(&store));

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

    let resize_sender = notification_sender.clone();
    Effect::new(move || {
        let width = col_resize.sizes.1;
        let height = row_resize.sizes.1;

        let _ = resize_sender.send(
            CanvasResizeIntent {
                width: (width.get() - DEAD_ZONES).into(),
                height: (height.get() - DEAD_ZONES).into(),
            }
            .into(),
        );
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
        <Provider value=PerspectiveKind::default()>
            <div class="grid items-center p-5 mx-auto h-full w-full bg-black/80" style=grid_style>
                // Row 1: PerspectiveIsland (spans all 5 columns)
                <PerspectiveIsland />

                // Row 2: Y nob between top and middle
                <div class="col-span-5 relative">
                    <ResizeYNob movement=set_row_first_move />
                </div>

                // Row 3: Main content row with X nobs
                <CenterResizableRow
                    set_col_first_move=set_col_first_move
                    set_col_third_move=set_col_third_move
                    notification_receiver=notification_receiver
                />

                // Row 4: Y nob between middle and bottom
                <div class="col-span-5 relative">
                    <ResizeYNob movement=set_row_third_move />
                </div>

                // Row 5: InfoIsland (spans all 5 columns)
                <InfoIsland />
            </div>
        </Provider>
    }
}

/// A component for the center resizable row with horizontal nobs.
#[component]
pub fn CenterResizableRow(
    set_col_first_move: WriteSignal<i32>,
    set_col_third_move: WriteSignal<i32>,
    notification_receiver: BevyMessageReceiver<Intents>,
) -> impl IntoView {
    view! {
        <FeatureIsland />

        <div class="relative h-full">
            <ResizeXNob movement=set_col_first_move />
        </div>

        <div class="h-full w-full" tabindex="0">
            <BevyCanvas
                init=move || {
                    init_bevy_app(BevyAppSettings {
                        intent: notification_receiver,
                    })
                }
                {..}
            />
        </div>

        <div class="relative h-full">
            <ResizeXNob movement=set_col_third_move />
        </div>

        <SupportIsland />
    }
}
