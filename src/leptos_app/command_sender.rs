use leptos_bevy_canvas::prelude::{LeptosChannelMessageSender, LeptosMessageSender};
use ui_event::command::Commands;

/// Context type providing the channel to send commands to Bevy.
#[derive(Clone)]
pub struct CommandSender {
    sender: LeptosMessageSender<Commands>,
}

impl CommandSender {
    pub fn new(sender: LeptosMessageSender<Commands>) -> Self {
        CommandSender { sender }
    }

    /// Send a command to backend
    pub fn send(&self, command: Commands) {
        let _ = self.sender.send(command);
    }
}
