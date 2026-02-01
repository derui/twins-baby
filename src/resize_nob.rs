use leptos::{IntoView, component, ev::MouseEvent, prelude::*, view};

/// [ResizeXNob] is the nob for resizing X-axis between nobs
///
/// # Arguments
/// * `initial` - initial position for center of nob
/// * `changed_position` - get a position to update position of the nob.
#[component]
pub fn ResizeXNob(initial: u32, movement: WriteSignal<i32>) -> impl IntoView {
    let (current, set_current) = signal(initial);

    let class = move || {
        let cur = current.get();

        format!(
            "flex-0 w-[16px] flex-col items-center justify-center x-[{}px]",
            cur
        )
    };

    let mouse_move = move |ev: MouseEvent| {
        let moved = ev.movement_x();

        if current.get() as i32 + moved < 0 {
            set_current.set(0);
        } else {
            movement.set(moved);
            set_current.set((current.get() as i32 + moved) as u32);
        }
    };

    view! {
        <div class=class>
            <span class="border-1 rounded bg-gray h-5 w-[8px]" on:mousemove=mouse_move></span>
        </div>
    }
}
