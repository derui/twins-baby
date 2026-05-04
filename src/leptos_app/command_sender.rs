use std::sync::{
    Arc,
    atomic::{AtomicU64, Ordering},
};

use leptos_bevy_canvas::prelude::{LeptosChannelMessageSender, LeptosMessageSender};
use ui_event::{CommandId, Correlation, command::Commands};

/// A generator for command id
#[derive(Debug, Clone)]
struct CommandIdGen {
    id_gen: Arc<AtomicU64>,
}

impl CommandIdGen {
    pub fn new() -> Self {
        CommandIdGen {
            id_gen: Arc::new(AtomicU64::new(1)),
        }
    }

    fn gen_id(&self) -> CommandId {
        let id = self.id_gen.fetch_add(1, Ordering::Relaxed);
        id.into()
    }
}

/// Context type providing the channel to send commands to Bevy.
#[derive(Clone)]
pub struct CommandSender {
    sender: LeptosMessageSender<Correlation<Commands>>,

    id_gen: CommandIdGen,
}

impl CommandSender {
    pub fn new(sender: LeptosMessageSender<Correlation<Commands>>) -> Self {
        CommandSender {
            sender,
            id_gen: CommandIdGen::new(),
        }
    }

    /// Send a command to backend
    pub fn send(&self, command: Commands) {
        let id = self.id_gen.gen_id();

        let _ = self.sender.send(Correlation::new(id, command));
    }
}
