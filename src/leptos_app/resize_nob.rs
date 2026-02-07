use leptos::{IntoView, component, ev::MouseEvent, prelude::*, view};

// nob area size. all size must be [px].
pub const NOB_AREA: u32 = 16;
// primary button of mouse
const PRIMARY_BUTTON: u16 = 1;

/// [ResizeXNob] is the nob for resizing X-axis between nobs
///
/// # Arguments
/// * `movement` - Signal to write the movement delta
/// * `class` - Optional additional CSS classes
#[component]
pub fn ResizeXNob(movement: WriteSignal<i32>, #[prop(optional)] class: String) -> impl IntoView {
    let class = move || format!("absolute transparent h-full {}", class);

    let style = move || format!("width: {}px", NOB_AREA);

    // handling mouse move. current and range is based on `nob` 's position,
    let mouse_move = move |ev: MouseEvent| {
        let buttons = ev.buttons();
        // 1 == primary button
        if buttons & PRIMARY_BUTTON == 0 {
            return;
        }

        let moved = ev.movement_x();
        movement.set(moved);
    };

    view! {
        <div
            class=class
            style=style
            on:mousemove=mouse_move
        ></div>
    }
}

/// [ResizeYNob] is the nob for resizing Y-axis between nobs
///
/// # Arguments
/// * `movement` - Signal to write the movement delta
/// * `class` - Optional additional CSS classes
#[component]
pub fn ResizeYNob(movement: WriteSignal<i32>, #[prop(optional)] class: String) -> impl IntoView {
    let class = move || format!("absolute transparent -translate-y-1/2 w-full {}", class);

    let style = move || format!("height: {}px", NOB_AREA);

    // handling mouse move. current and range is based on `nob` 's position,
    let mouse_move = move |ev: MouseEvent| {
        let buttons = ev.buttons();
        // 1 == primary button
        if buttons & PRIMARY_BUTTON == 0 {
            return;
        }

        let moved = ev.movement_y();
        movement.set(moved);
    };

    view! {
        <div
            class=class
            style=style
            on:mousemove=mouse_move
        ></div>
    }
}
