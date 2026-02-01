use leptos::{IntoView, component, ev::MouseEvent, prelude::*, view};

// nob size. all size must be [px].
const NOB_WIDTH: u32 = 8;

/// [ResizeXNob] is the nob for resizing X-axis between nobs
///
/// # Arguments
/// * `initial` - initial position for center of nob
/// * `range` - allow range of nob movement. This range should be left to right between movement this nob.
/// * `movement` - get relative movement of nob..
#[component]
pub fn ResizeXNob(
    initial: u32,
    range: Signal<(u32, u32)>,
    movement: WriteSignal<i32>,
) -> impl IntoView {
    let (current, set_current) = signal(initial + NOB_WIDTH / 2);
    let real_range = Signal::derive(move || {
        let (left, right) = range.get();

        (left + NOB_WIDTH / 2, right - NOB_WIDTH / 2)
    });

    let class = move || {
        let cur = current.get();

        format!(
            "flex-0 w-[16px] flex-col items-center justify-center x-[{}px]",
            cur
        )
    };

    // handling mouse move. current and range is based on `nob` 's position,
    let mouse_move = move |ev: MouseEvent| {
        let moved = ev.movement_x();
        let (left, right) = real_range.get();
        let current = current.get() + NOB_WIDTH / 2;

        let mut moved_current = current as i32 + moved;
        moved_current = moved_current.clamp(left as i32, right as i32);

        // after moved, set it as right
        if moved_current as u32 != current {
            movement.set(moved);
            set_current.set(moved_current as u32);
        }
    };

    view! {
        <div class=class>
            <span class="border-1 rounded bg-gray h-[40px] w-[8px]" on:mousemove=mouse_move></span>
        </div>
    }
}

/// [ResizeYNob] is the nob for resizing Y-axis between nobs
///
/// # Arguments
/// * `initial` - initial position for center of nob
/// * `range` - allow range of nob movement. This range should be top to bottom between movement this nob.
/// * `movement` - get a position to update position of the nob.
#[component]
pub fn ResizeYNob(
    initial: u32,
    range: Signal<(u32, u32)>,
    movement: WriteSignal<i32>,
) -> impl IntoView {
    let (current, set_current) = signal(initial + NOB_WIDTH / 2);
    let real_range = Signal::derive(move || {
        let (top, bottom) = range.get();

        (top + NOB_WIDTH / 2, bottom - NOB_WIDTH / 2)
    });

    let class = move || {
        let cur = current.get();

        format!(
            "flex-0 h-[16px] flex-row items-center justify-center y-[{}px]",
            cur
        )
    };

    // handling mouse move. current and range is based on `nob` 's position,
    let mouse_move = move |ev: MouseEvent| {
        let moved = ev.movement_y();
        let (top, bottom) = real_range.get();
        let current = current.get() + NOB_WIDTH / 2;

        let mut moved_current = current as i32 + moved;
        moved_current = moved_current.clamp(top as i32, bottom as i32);

        // after moved, set it as right
        if moved_current as u32 != current {
            movement.set(moved);
            set_current.set(moved_current as u32);
        }
    };

    view! {
        <div class=class>
            <span class="border-1 rounded bg-gray w-[40px] h-[8px]" on:mousemove=mouse_move></span>
        </div>
    }
}
