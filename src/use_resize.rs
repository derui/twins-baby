use leptos::prelude::*;

use crate::resize_nob::NOB_AREA;

#[derive(Debug, Clone)]
pub struct UseResize {
    /// Sizes of areas
    sizes: (Signal<u32>, Signal<u32>, Signal<u32>),
    first_movement: WriteSignal<i32>,
    third_movement: WriteSignal<i32>
}

fn apply_movement(current: u32, movement: i32, range: (u32, u32)) -> u32 {
    let moved = (current as i32 + movement) as u32;
    moved.clamp(range.0, range.1)
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
            *v = apply_movement(*v, movement, first_range.get());
        })
    });

    Effect::new(move || {
        let movement = third_movement.get();

        set_third_size.update(|v| {
            *v = apply_movement(*v, movement, third_range.get());
        })
    });
    
    UseResize {
        sizes: (first_size.into(), second_size, third_size.into())
        , first_movement: set_first_movement, third_movement: set_third_movement }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    // Layout used to derive expected ranges:
    //   window=1000, first=200, third=300, NOB_AREA=16
    //   first_range  = (NOB_AREA/2, window - third - NOB_AREA/2) = (8, 692)
    //   third_range  = (first + NOB_AREA/2, window - NOB_AREA/2) = (208, 992)

    #[test]
    fn zero_movement_keeps_current_value() {
        // Arrange
        let current = 200u32;
        let range = (8u32, 692u32);

        // Act
        let result = apply_movement(current, 0, range);

        // Assert
        assert_eq!(result, 200);
    }

    #[test]
    fn positive_movement_within_range_applies_delta() {
        // Arrange
        let current = 200u32;
        let range = (8u32, 692u32);

        // Act: 200 + 50 = 250, within [8, 692]
        let result = apply_movement(current, 50, range);

        // Assert
        assert_eq!(result, 250);
    }

    #[test]
    fn negative_movement_within_range_applies_delta() {
        // Arrange
        let current = 300u32;
        let range = (208u32, 992u32);

        // Act: 300 - 50 = 250, within [208, 992]
        let result = apply_movement(current, -50, range);

        // Assert
        assert_eq!(result, 250);
    }

    #[test]
    fn movement_clamped_at_minimum() {
        // Arrange
        let current = 200u32;
        let range = (8u32, 692u32);

        // Act: 200 - 195 = 5 < 8, clamped to range min
        let result = apply_movement(current, -195, range);

        // Assert
        assert_eq!(result, 8);
    }

    #[test]
    fn movement_clamped_at_maximum() {
        // Arrange
        let current = 200u32;
        let range = (8u32, 692u32);

        // Act: 200 + 600 = 800 > 692, clamped to range max
        let result = apply_movement(current, 600, range);

        // Assert
        assert_eq!(result, 692);
    }

    #[test]
    fn negative_sum_wraps_u32_and_clamps_to_maximum() {
        // Arrange: when i32 sum goes negative, `as u32` wraps to a large value
        let current = 5u32;
        let range = (8u32, 692u32);

        // Act: 5 - 10 = -5, wraps to u32::MAX - 4, clamped to range max
        let result = apply_movement(current, -10, range);

        // Assert
        assert_eq!(result, 692);
    }
}
