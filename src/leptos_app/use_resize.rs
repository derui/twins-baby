use leptos::prelude::*;

use crate::leptos_app::resize_nob::NOB_AREA;

#[derive(Debug, Clone)]
pub struct UseResize {
    /// Sizes of areas
    pub sizes: (Signal<u32>, Signal<u32>, Signal<u32>),
    pub first_movement: WriteSignal<Option<i32>>,
    pub third_movement: WriteSignal<Option<i32>>,
}

fn apply_movement(current: u32, movement: i32, range: (u32, u32)) -> u32 {
    let moved = i32::max(0, current as i32 + movement) as u32;
    moved.clamp(range.0, range.1)
}

/// Resizing 3-column/row view. This hook is specialized for
/// central place size is not strict.
pub fn use_resize(initial: (u32, u32), window_size: Signal<u32>) -> UseResize {
    let (first_movement, set_first_movement) = signal(None);
    let (third_movement, set_third_movement) = signal::<Option<i32>>(None);
    let (first_size, set_first_size) = signal(initial.0);
    let (third_size, set_third_size) = signal(initial.1);
    let second_size = Signal::derive(move || {
        let window = window_size.get();

        window - first_size.get() - third_size.get()
    });

    let first_range = Signal::derive(move || {
        let window = window_size.get();

        (NOB_AREA, window - third_size.get() - NOB_AREA)
    });

    let third_range = Signal::derive(move || {
        let window = window_size.get();

        (first_size.get() + NOB_AREA, window - NOB_AREA)
    });

    Effect::new(move || {
        if let Some(movement) = first_movement.get() {
            let first_range = first_range.get();

            set_first_size.update(|v| {
                *v = apply_movement(*v, movement, first_range);
            });
        }
    });

    Effect::new(move || {
        if let Some(movement) = third_movement.get() {
            let third_range = third_range.get();

            set_third_size.update(|v| {
                // moving third place on right = positive, size should be decrease
                *v = apply_movement(*v, -movement, third_range);
            })
        }
    });

    UseResize {
        sizes: (first_size.into(), second_size, third_size.into()),
        first_movement: set_first_movement,
        third_movement: set_third_movement,
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use crate::leptos_app::test_leptos::with_leptos_owner;

    use super::*;

    // Layout used to derive expected ranges:
    //   window=1000, first=200, third=300, NOB_AREA=16
    //   first_range  = (NOB_AREA, window - third - NOB_AREA) = (16, 684)
    //   third_range  = (first + NOB_AREA, window - NOB_AREA) = (216, 984)

    #[test]
    fn zero_movement_keeps_current_value() {
        // Arrange
        let current = 200u32;
        let range = (16u32, 684u32);

        // Act
        let result = apply_movement(current, 0, range);

        // Assert
        assert_eq!(result, 200);
    }

    #[test]
    fn positive_movement_within_range_applies_delta() {
        // Arrange
        let current = 200u32;
        let range = (16u32, 684u32);

        // Act: 200 + 50 = 250, within [16, 684]
        let result = apply_movement(current, 50, range);

        // Assert
        assert_eq!(result, 250);
    }

    #[test]
    fn negative_movement_within_range_applies_delta() {
        // Arrange
        let current = 300u32;
        let range = (216u32, 984u32);

        // Act: 300 - 50 = 250, within [216, 984]
        let result = apply_movement(current, -50, range);

        // Assert
        assert_eq!(result, 250);
    }

    #[test]
    fn movement_clamped_at_minimum() {
        // Arrange
        let current = 200u32;
        let range = (16u32, 684u32);

        // Act: 200 - 190 = 10 < 16, clamped to range min
        let result = apply_movement(current, -190, range);

        // Assert
        assert_eq!(result, 16);
    }

    #[test]
    fn movement_clamped_at_maximum() {
        // Arrange
        let current = 200u32;
        let range = (16u32, 684u32);

        // Act: 200 + 500 = 700 > 684, clamped to range max
        let result = apply_movement(current, 500, range);

        // Assert
        assert_eq!(result, 684);
    }

    #[test]
    fn negative_sum_clamped_to_minimum() {
        // Arrange: when i32 sum goes negative, i32::max ensures non-negative
        let current = 5u32;
        let range = (16u32, 684u32);

        // Act: 5 - 10 = -5, i32::max(0, -5) = 0, clamped to range min
        let result = apply_movement(current, -10, range);

        // Assert
        assert_eq!(result, 16);
    }

    #[tokio::test]
    async fn hook_initial_sizes_match_expected() {
        with_leptos_owner(async {
            // Arrange
            let (window_size, _set_window) = signal(1000u32);

            // Act
            let hook = use_resize((200, 300), window_size.into());
            any_spawner::Executor::tick().await;

            // Assert: second = window - first - third = 1000 - 200 - 300 = 500
            let (first, second, third) = hook.sizes;
            assert_eq!(first.get_untracked(), 200);
            assert_eq!(second.get_untracked(), 500);
            assert_eq!(third.get_untracked(), 300);
        })
        .await;
    }

    // Hook tests use layout: window=1000, first=200, third=300, NOB_AREA=16
    //   first_range  = (16, 684)
    //   third_range  = (216, 984)

    #[tokio::test]
    async fn hook_positive_first_movement_updates_first_and_second() {
        with_leptos_owner(async {
            // Arrange
            let (window_size, _set_window) = signal(1000u32);
            let hook = use_resize((200, 300), window_size.into());
            any_spawner::Executor::tick().await;

            // Act: first = clamp(200 + 50, 16, 684) = 250
            hook.first_movement.set(Some(50));
            any_spawner::Executor::tick().await;

            // Assert: second = 1000 - 250 - 300 = 450
            let (first, second, third) = hook.sizes;
            assert_eq!(first.get_untracked(), 250);
            assert_eq!(second.get_untracked(), 450);
            assert_eq!(third.get_untracked(), 300);
        })
        .await;
    }

    #[tokio::test]
    async fn hook_negative_first_movement_updates_first_and_second() {
        with_leptos_owner(async {
            // Arrange
            let (window_size, _set_window) = signal(1000u32);
            let hook = use_resize((200, 300), window_size.into());
            any_spawner::Executor::tick().await;

            // Act: first = clamp(200 - 100, 16, 684) = 100
            hook.first_movement.set(Some(-100));
            any_spawner::Executor::tick().await;

            // Assert: second = 1000 - 100 - 300 = 600
            let (first, second, third) = hook.sizes;
            assert_eq!(first.get_untracked(), 100);
            assert_eq!(second.get_untracked(), 600);
            assert_eq!(third.get_untracked(), 300);
        })
        .await;
    }

    #[tokio::test]
    async fn hook_positive_third_movement_updates_third_and_second() {
        with_leptos_owner(async {
            // Arrange
            let (window_size, _set_window) = signal(1000u32);
            let hook = use_resize((200, 300), window_size.into());
            any_spawner::Executor::tick().await;

            // Act: positive movement decreases third size (movement is inverted)
            // third = clamp(300 - 50, 216, 984) = 250
            hook.third_movement.set(Some(50));
            any_spawner::Executor::tick().await;

            // Assert: second = 1000 - 200 - 250 = 550
            let (first, second, third) = hook.sizes;
            assert_eq!(first.get_untracked(), 200);
            assert_eq!(second.get_untracked(), 550);
            assert_eq!(third.get_untracked(), 250);
        })
        .await;
    }

    #[tokio::test]
    async fn hook_negative_third_movement_updates_third_and_second() {
        with_leptos_owner(async {
            // Arrange
            let (window_size, _set_window) = signal(1000u32);
            let hook = use_resize((200, 300), window_size.into());
            any_spawner::Executor::tick().await;

            // Act: negative movement increases third size (movement is inverted)
            // third = clamp(300 + 50, 216, 984) = 350
            hook.third_movement.set(Some(-50));
            any_spawner::Executor::tick().await;

            // Assert: second = 1000 - 200 - 350 = 450
            let (first, second, third) = hook.sizes;
            assert_eq!(first.get_untracked(), 200);
            assert_eq!(second.get_untracked(), 450);
            assert_eq!(third.get_untracked(), 350);
        })
        .await;
    }

    #[tokio::test]
    async fn hook_first_movement_clamped_at_minimum() {
        with_leptos_owner(async {
            // Arrange
            let (window_size, _set_window) = signal(1000u32);
            let hook = use_resize((200, 300), window_size.into());
            any_spawner::Executor::tick().await;

            // Act: first = clamp(200 - 200, 16, 684) = 16 (clamped to min)
            hook.first_movement.set(Some(-200));
            any_spawner::Executor::tick().await;

            // Assert: second = 1000 - 16 - 300 = 684
            let (first, second, third) = hook.sizes;
            assert_eq!(first.get_untracked(), 16);
            assert_eq!(second.get_untracked(), 684);
            assert_eq!(third.get_untracked(), 300);
        })
        .await;
    }

    #[tokio::test]
    async fn hook_first_movement_clamped_at_maximum() {
        with_leptos_owner(async {
            // Arrange
            let (window_size, _set_window) = signal(1000u32);
            let hook = use_resize((200, 300), window_size.into());
            any_spawner::Executor::tick().await;

            // Act: first = clamp(200 + 500, 16, 684) = 684 (clamped to max)
            hook.first_movement.set(Some(500));
            any_spawner::Executor::tick().await;

            // Assert: second = 1000 - 684 - 300 = 16
            let (first, second, third) = hook.sizes;
            assert_eq!(first.get_untracked(), 684);
            assert_eq!(second.get_untracked(), 16);
            assert_eq!(third.get_untracked(), 300);
        })
        .await;
    }

    #[tokio::test]
    async fn hook_third_movement_clamped_at_minimum() {
        with_leptos_owner(async {
            // Arrange
            let (window_size, _set_window) = signal(1000u32);
            let hook = use_resize((200, 300), window_size.into());
            any_spawner::Executor::tick().await;

            // Act: positive movement decreases third (inverted)
            // third = clamp(300 - 100, 216, 984) = 216 (clamped to min)
            hook.third_movement.set(Some(100));
            any_spawner::Executor::tick().await;

            // Assert: second = 1000 - 200 - 216 = 584
            let (first, second, third) = hook.sizes;
            assert_eq!(first.get_untracked(), 200);
            assert_eq!(second.get_untracked(), 584);
            assert_eq!(third.get_untracked(), 216);
        })
        .await;
    }

    #[tokio::test]
    async fn hook_window_resize_updates_second_size() {
        with_leptos_owner(async {
            // Arrange
            let (window_size, set_window) = signal(1000u32);
            let hook = use_resize((200, 300), window_size.into());
            any_spawner::Executor::tick().await;

            // Act: expand window from 1000 to 1200
            set_window.set(1200);
            any_spawner::Executor::tick().await;

            // Assert: first and third unchanged, second = 1200 - 200 - 300 = 700
            let (first, second, third) = hook.sizes;
            assert_eq!(first.get_untracked(), 200);
            assert_eq!(second.get_untracked(), 700);
            assert_eq!(third.get_untracked(), 300);
        })
        .await;
    }

    #[tokio::test]
    async fn hook_setting_movement_to_none_does_not_trigger_change() {
        with_leptos_owner(async {
            // Arrange
            let (window_size, _set_window) = signal(1000u32);
            let hook = use_resize((200, 300), window_size.into());
            any_spawner::Executor::tick().await;

            // Act: set movement to None (no-op)
            hook.first_movement.set(None);
            any_spawner::Executor::tick().await;

            // Assert: sizes remain unchanged
            let (first, second, third) = hook.sizes;
            assert_eq!(first.get_untracked(), 200);
            assert_eq!(second.get_untracked(), 500);
            assert_eq!(third.get_untracked(), 300);
        })
        .await;
    }

    #[tokio::test]
    async fn hook_sequential_movements_apply_from_current_position() {
        with_leptos_owner(async {
            // Arrange
            let (window_size, _set_window) = signal(1000u32);
            let hook = use_resize((200, 300), window_size.into());
            any_spawner::Executor::tick().await;

            // Act: first movement of +50
            hook.first_movement.set(Some(50));
            any_spawner::Executor::tick().await;

            // Assert: first = 200 + 50 = 250
            let (first, second, third) = hook.sizes;
            assert_eq!(first.get_untracked(), 250);
            assert_eq!(second.get_untracked(), 450);
            assert_eq!(third.get_untracked(), 300);

            // Act: reset to None, then second movement of +30
            hook.first_movement.set(None);
            any_spawner::Executor::tick().await;
            hook.first_movement.set(Some(30));
            any_spawner::Executor::tick().await;

            // Assert: first = 250 + 30 = 280 (applied from current position)
            assert_eq!(first.get_untracked(), 280);
            assert_eq!(second.get_untracked(), 420);
            assert_eq!(third.get_untracked(), 300);
        })
        .await;
    }

    #[tokio::test]
    async fn hook_third_sequential_movements_apply_from_current_position() {
        with_leptos_owner(async {
            // Arrange
            let (window_size, _set_window) = signal(1000u32);
            let hook = use_resize((200, 300), window_size.into());
            any_spawner::Executor::tick().await;

            // Act: first movement of -50 (negative increases third size when inverted)
            hook.third_movement.set(Some(-50));
            any_spawner::Executor::tick().await;

            // Assert: third = 300 + 50 = 350 (movement inverted: -1 * -50 = 50)
            let (first, second, third) = hook.sizes;
            assert_eq!(first.get_untracked(), 200);
            assert_eq!(second.get_untracked(), 450);
            assert_eq!(third.get_untracked(), 350);

            // Act: reset to None, then second movement of -30
            hook.third_movement.set(None);
            any_spawner::Executor::tick().await;
            hook.third_movement.set(Some(-30));
            any_spawner::Executor::tick().await;

            // Assert: third = 350 + 30 = 380 (movement inverted: -1 * -30 = 30, applied from current position)
            assert_eq!(first.get_untracked(), 200);
            assert_eq!(second.get_untracked(), 420);
            assert_eq!(third.get_untracked(), 380);
        })
        .await;
    }

    #[tokio::test]
    async fn hook_setting_same_movement_value_retriggers_effect() {
        with_leptos_owner(async {
            // Arrange
            let (window_size, _set_window) = signal(1000u32);
            let hook = use_resize((200, 300), window_size.into());
            any_spawner::Executor::tick().await;

            // Act: set movement to Some(50)
            hook.first_movement.set(Some(50));
            any_spawner::Executor::tick().await;

            // Assert: first = 250
            let (first, second, third) = hook.sizes;
            assert_eq!(first.get_untracked(), 250);

            // Act: set same value again (Leptos effects trigger on every .set() call)
            hook.first_movement.set(Some(50));
            any_spawner::Executor::tick().await;

            // Assert: first = 300 (movement applied again from current position)
            assert_eq!(first.get_untracked(), 300);
            assert_eq!(second.get_untracked(), 400);
            assert_eq!(third.get_untracked(), 300);
        })
        .await;
    }

    #[tokio::test]
    async fn hook_first_then_third_movements_independent() {
        with_leptos_owner(async {
            // Arrange
            let (window_size, _set_window) = signal(1000u32);
            let hook = use_resize((200, 300), window_size.into());
            any_spawner::Executor::tick().await;

            // Act: move first by +50 (increases first size directly)
            hook.first_movement.set(Some(50));
            any_spawner::Executor::tick().await;

            // Assert: first = 250, second = 450, third = 300
            let (first, second, third) = hook.sizes;
            assert_eq!(first.get_untracked(), 250);
            assert_eq!(second.get_untracked(), 450);
            assert_eq!(third.get_untracked(), 300);

            // Act: reset first movement and move third
            hook.first_movement.set(None);
            any_spawner::Executor::tick().await;

            hook.third_movement.set(Some(-20));
            any_spawner::Executor::tick().await;

            // Assert: first = 250, second = 430, third = 320
            assert_eq!(first.get_untracked(), 250);
            assert_eq!(second.get_untracked(), 430);
            assert_eq!(third.get_untracked(), 320);
        })
        .await;
    }

    #[tokio::test]
    async fn hook_movement_respects_updated_ranges_after_window_resize() {
        with_leptos_owner(async {
            // Arrange
            let (window_size, set_window) = signal(1000u32);
            let hook = use_resize((200, 300), window_size.into());
            any_spawner::Executor::tick().await;

            // Act: expand window to 1500
            set_window.set(1500);
            any_spawner::Executor::tick().await;

            // Assert: first = 200, second = 1000, third = 300
            let (first, second, third) = hook.sizes;
            assert_eq!(first.get_untracked(), 200);
            assert_eq!(second.get_untracked(), 1000);
            assert_eq!(third.get_untracked(), 300);

            // Act: move first by +600
            // New first_range = (16, 1500 - 300 - 16) = (16, 1184)
            hook.first_movement.set(Some(600));
            any_spawner::Executor::tick().await;

            // Assert: first = clamp(200 + 600, 16, 1184) = 800
            assert_eq!(first.get_untracked(), 800);
            assert_eq!(second.get_untracked(), 400);
            assert_eq!(third.get_untracked(), 300);
        })
        .await;
    }
}
