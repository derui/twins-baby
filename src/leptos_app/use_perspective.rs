use leptos::prelude::{Effect, Get, Set, Signal, WriteSignal, provide_context, signal, use_context};
use leptos_bevy_canvas::prelude::{LeptosChannelMessageSender, LeptosMessageSender};

use crate::events::{PerspectiveChangeEvent, PerspectiveKind};

/// This module provides a hook to manage global **perspective** of the app.
pub struct UsePerspective(Signal<PerspectiveKind>, WriteSignal<PerspectiveKind>);

/// Get a hook of perspective. The hook can:
///
/// - get current perspective as reactive
/// - set a perspective in global, including beby
///
/// This hook requires wrapping with `Provider` with [PerspectiveKind] value. 
pub fn use_perspective(sender: LeptosMessageSender<PerspectiveChangeEvent>) -> UsePerspective {
    let (value, set_value) = signal::<PerspectiveKind>(PerspectiveKind::default());

    Effect::new(move || {
        let value = value.get();
         provide_context(value);
        let _ = sender.send(PerspectiveChangeEvent {next: value
        });
    });

    Effect::new(move || {
        let Some(v) = use_context() else {return};

        if v != value.get() {
            set_value.set(v);
        }
    });

    UsePerspective(value.into(), set_value)
}
