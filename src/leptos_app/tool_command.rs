use leptos_bevy_canvas::prelude::MessageSenderL2B;

use crate::events::SketchToolEvent;

/// Context type providing the channel to send sketch tool events to Bevy.
#[derive(Clone)]
pub struct ToolCommand(pub MessageSenderL2B<SketchToolEvent>);
