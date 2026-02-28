use leptos_bevy_canvas::prelude::LeptosMessageSender;
use ui_event::command::Commands;

/// Context type providing the channel to send sketch tool events to Bevy.
#[derive(Clone)]
pub struct ToolCommand(pub LeptosMessageSender<Commands>);
