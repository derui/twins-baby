use std::sync::{
    Arc,
    atomic::{AtomicU64, Ordering},
};

use leptos_bevy_canvas::prelude::{LeptosChannelMessageSender, LeptosMessageSender};
use ui_event::{CommandId, command::Commands};

/// Context type providing the channel to send commands to Bevy.
#[derive(Clone)]
pub struct CommandSender {
    sender: LeptosMessageSender<Commands>,
    id_counter: Arc<AtomicU64>,
}

impl CommandSender {
    pub fn new(sender: LeptosMessageSender<Commands>) -> Self {
        CommandSender {
            sender,
            id_counter: Arc::new(AtomicU64::new(0)),
        }
    }

    /// Send a command to backend
    pub fn send(&self, command: impl FnOnce(CommandId) -> Commands) {
        let id: CommandId = self.id_counter.fetch_add(1, Ordering::Relaxed).into();

        let command = command(id);
        let _ = self.sender.send(command);
    }
}
