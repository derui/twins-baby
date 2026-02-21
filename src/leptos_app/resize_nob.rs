use leptos::{IntoView, component, ev::MouseEvent, prelude::*, view};

// nob area size. all size must be [px].
pub const NOB_AREA: u32 = 16;
// primary button of mouse
const PRIMARY_BUTTON: u16 = 1;

/// [ResizeXNob] is the nob for resizing X-axis between nobs
///
/// # Arguments
/// * `movement` - Signal to write the accumulated movement delta on release
/// * `class` - Optional additional CSS classes
#[component]
pub fn ResizeXNob(movement: WriteSignal<i32>, #[prop(optional)] class: String) -> impl IntoView {
    let class = move || format!("absolute transparent h-full {}", class);

    let style = move || format!("width: {}px", NOB_AREA);

    let (is_dragging, set_is_dragging) = signal(false);
    let (accumulated_x, set_accumulated_x) = signal(0);
    let (ghost_x, set_ghost_x) = signal(0);

    let mouse_down = move |ev: MouseEvent| {
        let buttons = ev.buttons();
        if buttons & PRIMARY_BUTTON != 0 {
            set_is_dragging.set(true);
            set_accumulated_x.set(0);
            set_ghost_x.set(ev.client_x());
        }
    };

    // Attach global mousemove listener when dragging
    let _ = window_event_listener(leptos::ev::mousemove, move |ev: MouseEvent| {
        if !is_dragging.get() {
            return;
        }

        let moved = ev.movement_x();
        set_accumulated_x.update(|acc| *acc += moved);
        set_ghost_x.update(|x| *x += moved);
    });

    // Attach global mouseup listener to handle release anywhere
    let _ = window_event_listener(leptos::ev::mouseup, move |_ev: MouseEvent| {
        if is_dragging.get() {
            leptos::logging::log!("moved {}", accumulated_x.get());
            movement.set(accumulated_x.get());
            set_is_dragging.set(false);
            set_accumulated_x.set(0);
        }
    });

    let ghost_style = move || {
        format!(
            "left: {}px; top: {}px; width: {}px; height: 100vh;",
            ghost_x.get(),
            0,
            NOB_AREA
        )
    };

    view! {
        <div class=class style=style on:mousedown=mouse_down></div>
        {move || {
            is_dragging
                .get()
                .then(|| {
                    view! {
                        <div
                            class="fixed pointer-events-none bg-blue-400/50 z-50"
                            style=ghost_style
                        ></div>
                    }
                })
        }}
    }
}

/// [ResizeYNob] is the nob for resizing Y-axis between nobs
///
/// # Arguments
/// * `movement` - Signal to write the accumulated movement delta on release
/// * `class` - Optional additional CSS classes
#[component]
pub fn ResizeYNob(movement: WriteSignal<i32>, #[prop(optional)] class: String) -> impl IntoView {
    let class = move || format!("absolute transparent -translate-y-1/2 w-full {}", class);

    let style = move || format!("height: {}px", NOB_AREA);

    let (is_dragging, set_is_dragging) = signal(false);
    let (accumulated_y, set_accumulated_y) = signal(0);
    let (ghost_y, set_ghost_y) = signal(0);

    let mouse_down = move |ev: MouseEvent| {
        let buttons = ev.buttons();
        if buttons & PRIMARY_BUTTON != 0 {
            set_is_dragging.set(true);
            set_accumulated_y.set(0);
            set_ghost_y.set(ev.client_y());
        }
    };

    // Attach global mousemove listener when dragging
    let _ = window_event_listener(leptos::ev::mousemove, move |ev: MouseEvent| {
        if !is_dragging.get() {
            return;
        }

        let moved = ev.movement_y();
        set_accumulated_y.update(|acc| *acc += moved);
        set_ghost_y.update(|y| *y += moved);
    });

    // Attach global mouseup listener to handle release anywhere
    let _ = window_event_listener(leptos::ev::mouseup, move |_ev: MouseEvent| {
        if is_dragging.get() {
            movement.set(accumulated_y.get());
            set_is_dragging.set(false);
            set_accumulated_y.set(0);
        }
    });

    let ghost_style = move || {
        format!(
            "left: {}px; top: {}px; width: 100vw; height: {}px;",
            0,
            ghost_y.get(),
            NOB_AREA
        )
    };

    view! {
        <div class=class style=style on:mousedown=mouse_down></div>
        {move || {
            is_dragging
                .get()
                .then(|| {
                    view! {
                        <div
                            class="fixed pointer-events-none bg-blue-400/50 z-50"
                            style=ghost_style
                        ></div>
                    }
                })
        }}
    }
}
