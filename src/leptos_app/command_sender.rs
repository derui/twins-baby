use leptos_bevy_canvas::prelude::{LeptosChannelMessageSender, LeptosMessageSender};
use ui_event::command::Commands;

/// Context type providing the channel to send commands to Bevy.
#[derive(Clone)]
pub struct CommandSender(LeptosMessageSender<Commands>);

impl CommandSender {
    pub fn new(sender: LeptosMessageSender<Commands>) -> Self {
        CommandSender(sender)
    }

    /// Send a command to backend
    pub fn send(&mut self, command: Commands) {
        let _ = self.0.send(command);
    }
}
