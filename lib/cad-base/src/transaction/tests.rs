use pretty_assertions::assert_eq;
use rstest::rstest;

use super::*;

// Test for new()

#[test]
fn test_new_creates_history_with_initial_state() {
    // Arrange
    let initial = 42;
    let max_history = 10;

    // Act
    let history = SnapshotHistory::new(initial, max_history);

    // Assert
    assert_eq!(*history.state(), 42);
}

#[test]
fn test_new_with_min_max_history() {
    // Arrange
    let initial = "state";
    let max_history = 1;

    // Act
    let history = SnapshotHistory::new(initial, max_history);

    // Assert
    assert_eq!(*history.state(), "state");
}

#[test]
#[should_panic(expected = "History size must be greater than 0")]
fn test_new_with_zero_max_history_panics() {
    // Arrange
    let initial = 42;
    let max_history = 0;

    // Act
    SnapshotHistory::new(initial, max_history);
}

// Tests for save()

#[test]
fn test_save_updates_current_state() {
    // Arrange
    let mut history = SnapshotHistory::new(1, 10);

    // Act
    history.save_snapshot();
    *history.state_mut() = 2;

    // Assert
    assert_eq!(*history.state(), 2);
}

#[test]
fn test_save_allows_undo() {
    // Arrange
    let mut history = SnapshotHistory::new(1, 10);

    // Act
    history.save_snapshot();
    *history.state_mut() = 2;
    let can_undo = history.undo();

    // Assert
    assert!(can_undo);
    assert_eq!(*history.state(), 1);
}

#[test]
fn test_save_clears_redo_stack() {
    // Arrange
    let mut history = SnapshotHistory::new(1, 10);
    history.save_snapshot();
    *history.state_mut() = 2;
    history.save_snapshot();
    *history.state_mut() = 3;
    history.undo(); // Now redo is available

    // Act
    history.save_snapshot();
    *history.state_mut() = 4;
    let can_redo = history.redo();

    // Assert - redo should not be available
    assert!(!can_redo);
    assert_eq!(*history.state(), 4);
}

#[test]
fn test_save_multiple_times_maintains_order() {
    // Arrange
    let mut history = SnapshotHistory::new(0, 10);

    // Act
    for i in 1..=5 {
        history.save_snapshot();
        *history.state_mut() = i;
    }

    // Assert - verify by undoing in reverse order
    assert_eq!(*history.state(), 5);
    history.undo();
    assert_eq!(*history.state(), 4);
    history.undo();
    assert_eq!(*history.state(), 3);
    history.undo();
    assert_eq!(*history.state(), 2);
    history.undo();
    assert_eq!(*history.state(), 1);
    history.undo();
    assert_eq!(*history.state(), 0);
}

#[rstest]
#[case(1, vec![1, 2, 3], vec![2])]
#[case(2, vec![1, 2, 3], vec![2, 1])]
#[case(3, vec![1, 2, 3, 4, 5], vec![4, 3, 2])]
#[case(5, vec![1, 2, 3], vec![2, 1, 0])]
fn test_save_respects_max_history(
    #[case] max_history: usize,
    #[case] pushes: Vec<i32>,
    #[case] expected_undo_sequence: Vec<i32>,
) {
    // Arrange
    let mut history = SnapshotHistory::new(0, max_history);

    // Act
    for &value in &pushes {
        history.save_snapshot();
        *history.state_mut() = value;
    }

    // Assert - verify by undoing and checking sequence
    for &expected in &expected_undo_sequence {
        assert!(history.undo());
        assert_eq!(*history.state(), expected);
    }
    // No more undos should be possible
    assert!(!history.undo());
}

#[test]
fn test_max_history_boundary() {
    // Arrange
    let mut history = SnapshotHistory::new(0, 2);

    // Act - push exactly max_history items
    history.save_snapshot();
    *history.state_mut() = 1;
    history.save_snapshot();
    *history.state_mut() = 2;

    // Assert - should be able to undo max_history times
    assert!(history.undo()); // 2 -> 1
    assert!(history.undo()); // 1 -> 0
    assert!(!history.undo()); // Cannot undo past initial
    assert_eq!(*history.state(), 0);
}

// Tests for undo()

#[test]
fn test_undo_restores_previous_state() {
    // Arrange
    let mut history = SnapshotHistory::new(1, 10);
    history.save_snapshot();
    *history.state_mut() = 2;

    // Act
    let result = history.undo();

    // Assert
    assert!(result);
    assert_eq!(*history.state(), 1);
}

#[test]
fn test_undo_returns_false_when_no_history() {
    // Arrange
    let mut history = SnapshotHistory::new(1, 10);

    // Act
    let result = history.undo();

    // Assert
    assert!(!result);
    assert_eq!(*history.state(), 1);
}

#[test]
fn test_undo_enables_redo() {
    // Arrange
    let mut history = SnapshotHistory::new(1, 10);
    history.save_snapshot();
    *history.state_mut() = 2;

    // Act
    history.undo();
    let can_redo = history.redo();

    // Assert
    assert!(can_redo);
    assert_eq!(*history.state(), 2);
}

#[test]
fn test_undo_multiple_times() {
    // Arrange
    let mut history = SnapshotHistory::new(0, 10);
    history.save_snapshot();
    *history.state_mut() = 1;
    history.save_snapshot();
    *history.state_mut() = 2;
    history.save_snapshot();
    *history.state_mut() = 3;

    // Act
    let result1 = history.undo();
    let result2 = history.undo();

    // Assert
    assert!(result1);
    assert!(result2);
    assert_eq!(*history.state(), 1);
}

#[test]
fn test_undo_until_initial_state() {
    // Arrange
    let mut history = SnapshotHistory::new(0, 10);
    history.save_snapshot();
    *history.state_mut() = 1;
    history.save_snapshot();
    *history.state_mut() = 2;

    // Act & Assert
    history.undo();
    assert_eq!(*history.state(), 1);
    history.undo();
    assert_eq!(*history.state(), 0);
    let result = history.undo();
    assert!(!result);
    assert_eq!(*history.state(), 0);
}

// Tests for redo()

#[test]
fn test_redo_restores_undone_state() {
    // Arrange
    let mut history = SnapshotHistory::new(1, 10);
    history.save_snapshot();
    *history.state_mut() = 2;
    history.undo();

    // Act
    let result = history.redo();

    // Assert
    assert!(result);
    assert_eq!(*history.state(), 2);
}

#[test]
fn test_redo_returns_false_when_no_redo_available() {
    // Arrange
    let mut history = SnapshotHistory::new(1, 10);

    // Act
    let result = history.redo();

    // Assert
    assert!(!result);
    assert_eq!(*history.state(), 1);
}

#[test]
fn test_redo_enables_undo() {
    // Arrange
    let mut history = SnapshotHistory::new(1, 10);
    history.save_snapshot();
    *history.state_mut() = 2;
    history.undo();

    // Act
    history.redo();
    let can_undo = history.undo();

    // Assert
    assert!(can_undo);
    assert_eq!(*history.state(), 1);
}

#[test]
fn test_redo_multiple_times() {
    // Arrange
    let mut history = SnapshotHistory::new(0, 10);
    history.save_snapshot();
    *history.state_mut() = 1;
    history.save_snapshot();
    *history.state_mut() = 2;
    history.save_snapshot();
    *history.state_mut() = 3;
    history.undo();
    history.undo();

    // Act
    let result1 = history.redo();
    let result2 = history.redo();

    // Assert
    assert!(result1);
    assert!(result2);
    assert_eq!(*history.state(), 3);
}

#[test]
fn test_redo_exhausts_all_redos() {
    // Arrange
    let mut history = SnapshotHistory::new(0, 10);
    history.save_snapshot();
    *history.state_mut() = 1;
    history.save_snapshot();
    *history.state_mut() = 2;
    history.undo();
    history.undo();

    // Act & Assert
    assert!(history.redo()); // 0 -> 1
    assert_eq!(*history.state(), 1);
    assert!(history.redo()); // 1 -> 2
    assert_eq!(*history.state(), 2);
    assert!(!history.redo()); // No more redos
    assert_eq!(*history.state(), 2);
}

// Integration tests

#[test]
fn test_undo_then_redo_cycle() {
    // Arrange
    let mut history = SnapshotHistory::new(1, 10);
    history.save_snapshot();
    *history.state_mut() = 2;
    history.save_snapshot();
    *history.state_mut() = 3;

    // Act
    history.undo();
    assert_eq!(*history.state(), 2);
    history.undo();
    assert_eq!(*history.state(), 1);
    history.redo();

    // Assert
    assert_eq!(*history.state(), 2);
}

#[test]
fn test_push_after_undo_clears_redo() {
    // Arrange
    let mut history = SnapshotHistory::new(1, 10);
    history.save_snapshot();
    *history.state_mut() = 2;
    history.save_snapshot();
    *history.state_mut() = 3;
    history.undo();

    // Act
    history.save_snapshot();
    *history.state_mut() = 4;

    // Assert
    assert_eq!(*history.state(), 4);
    assert!(!history.redo()); // Redo should not be available
}

#[test]
fn test_complex_history_manipulation() {
    // Arrange
    let mut history = SnapshotHistory::new(0, 5);

    // Act & Assert - Build history
    history.save_snapshot();
    *history.state_mut() = 1;
    history.save_snapshot();
    *history.state_mut() = 2;
    history.save_snapshot();
    *history.state_mut() = 3;
    assert_eq!(*history.state(), 3);

    // Undo twice
    history.undo();
    history.undo();
    assert_eq!(*history.state(), 1);

    // Redo once
    history.redo();
    assert_eq!(*history.state(), 2);

    // Push new state (should clear remaining redo)
    history.save_snapshot();
    *history.state_mut() = 10;
    assert_eq!(*history.state(), 10);
    assert!(!history.redo()); // No redo available

    // Verify undo sequence
    history.undo();
    assert_eq!(*history.state(), 2);
    history.undo();
    assert_eq!(*history.state(), 1);
    history.undo();
    assert_eq!(*history.state(), 0);
}

#[test]
fn test_snapshot_works_with_custom_types() {
    // Arrange
    #[derive(Clone, Debug, PartialEq)]
    struct State {
        value: i32,
    }

    let initial = State { value: 1 };
    let mut history = SnapshotHistory::new(initial, 10);

    // Act
    history.save_snapshot();
    history.state_mut().value = 2;
    history.undo();

    // Assert
    assert_eq!(history.state().value, 1);
}

#[test]
fn test_alternating_undo_redo() {
    // Arrange
    let mut history = SnapshotHistory::new(1, 10);
    history.save_snapshot();
    *history.state_mut() = 2;
    history.save_snapshot();
    *history.state_mut() = 3;

    // Act & Assert - alternate between undo and redo
    history.undo();
    assert_eq!(*history.state(), 2);
    history.redo();
    assert_eq!(*history.state(), 3);
    history.undo();
    assert_eq!(*history.state(), 2);
    history.undo();
    assert_eq!(*history.state(), 1);
    history.redo();
    assert_eq!(*history.state(), 2);
}

#[test]
fn test_single_undo_redo_cycle() {
    // Arrange
    let mut history = SnapshotHistory::new(1, 10);
    history.save_snapshot();
    *history.state_mut() = 2;

    // Act & Assert
    let can_undo = history.undo();
    assert!(can_undo);
    assert_eq!(*history.state(), 1);

    let can_redo = history.redo();
    assert!(can_redo);
    assert_eq!(*history.state(), 2);
}
