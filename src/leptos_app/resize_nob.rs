use leptos::{IntoView, component, ev::MouseEvent, prelude::*, view};

// nob area size. all size must be [px].
pub const NOB_AREA: u32 = 16;

/// [ResizeXNob] is the nob for resizing X-axis between nobs
///
/// # Arguments
/// * `movement` - Signal to write the movement delta
/// * `class` - Optional additional CSS classes
#[component]
pub fn ResizeXNob(movement: WriteSignal<i32>, #[prop(optional)] class: String) -> impl IntoView {
    let (enable_move, set_enable_move) = signal(false);

    let class = move || {
        format!(
            "absolute transparent w-[{}px] -translate-x-1/2 h-full {}",
            NOB_AREA, class
        )
    };

    let mouse_down = move |_: MouseEvent| {
        set_enable_move.set(true);
    };

    let mouse_up = move |_: MouseEvent| {
        set_enable_move.set(false);
    };

    // handling mouse move. current and range is based on `nob` 's position,
    let mouse_move = move |ev: MouseEvent| {
        if !enable_move.get() {
            return;
        }

        let moved = ev.movement_x();
        movement.set(moved);
    };

    view! {
        <div class=class on:mousemove=mouse_move on:mousedown=mouse_down on:mouseup=mouse_up>
        </div>
    }
}

/// [ResizeYNob] is the nob for resizing Y-axis between nobs
///
/// # Arguments
/// * `movement` - Signal to write the movement delta
/// * `class` - Optional additional CSS classes
#[component]
pub fn ResizeYNob(movement: WriteSignal<i32>, #[prop(optional)] class: String) -> impl IntoView {
    let (enable_move, set_enable_move) = signal(false);

    let class = move || {
        format!(
            "absolute transparent h-[{}px] -translate-y-1/2 w-full {}",
            NOB_AREA, class
        )
    };

    let mouse_down = move |_: MouseEvent| {
        set_enable_move.set(true);
    };

    let mouse_up = move |_: MouseEvent| {
        set_enable_move.set(false);
    };

    // handling mouse move. current and range is based on `nob` 's position,
    let mouse_move = move |ev: MouseEvent| {
        if !enable_move.get() {
            return;
        }

        let moved = ev.movement_y();
        movement.set(moved);
    };

    view! {
        <div class=class on:mousemove=mouse_move on:mousedown=mouse_down on:mouseup=mouse_up>
        </div>
    }
}
