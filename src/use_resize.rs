use leptos::prelude::*;

use crate::resize_nob::NOB_AREA;

#[derive(Debug, Clone)]
pub struct UseResize {
    /// Sizes of areas
    sizes: (Signal<u32>, Signal<u32>, Signal<u32>),
    first_movement: WriteSignal<i32>,
    third_movement: WriteSignal<i32>
}

/// Resizing 3-column/row view. This hook is specialized for
/// central place size is not strict.
pub fn use_resize(
    initial: (u32, u32),
    window_size: Signal<u32>,
) -> UseResize {
    let (first_movement, set_first_movement) = signal(0_i32);
    let (third_movement, set_third_movement) = signal(0_i32);
    let (first_size, set_first_size) = signal(initial.0);
    let (third_size, set_third_size) = signal(initial.1);
    let second_size = Signal::derive(move || {
        let window = window_size.get();

        window - first_size.get() - third_size.get()
    });

    let first_range = Signal::derive(move || {
        let window = window_size.get();

        (NOB_AREA / 2, window - third_size.get() - NOB_AREA / 2)
    });

    let third_range = Signal::derive(move || {
        let window = window_size.get();

        (first_size.get() + NOB_AREA / 2, window - NOB_AREA / 2)
    });

    Effect::new(move || {
        let movement = first_movement.get();

        set_first_size.update(|v| {
            let moved = (*v as i32 + movement) as u32;
            
            *v = moved.clamp(first_range.get().0, first_range.get().1);
        })
    });

    Effect::new(move || {
        let movement = third_movement.get();

        set_third_size.update(|v| {
            let moved = (*v as i32 + movement) as u32;
            
            *v = moved.clamp(third_range.get().0, third_range.get().1);
        })
    });
    
    UseResize {
        sizes: (first_size.into(), second_size, third_size.into())
        , first_movement: set_first_movement, third_movement: set_third_movement }
}
