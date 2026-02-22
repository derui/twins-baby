use bevy::ecs::message::Message;
use coarsetime::Instant;

/// Tool selection for sketch mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SketchTool {
    Line,
    Circle,
    Rectangle,
}

/// An event to change the active sketch tool
#[derive(Message, Debug, Clone)]
pub struct SketchToolEvent {
    pub tool: SketchTool,
}

/// An event to notice canvas resize
#[derive(Message, Debug, Clone)]
pub struct CanvasResizeEvent {
    pub width: u32,
    pub height: u32,
}

/// Kind of perspective
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PerspectiveKind {
    #[default]
    Feature,
    Sketch,
}

/// Log levels for logging events
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogLevel {
    Debug,
    Info,
    Warning,
    Error,
}

/// An event for logging messages
#[derive(Message, Debug, Clone)]
pub struct LoggingEvent {
    pub log_level: LogLevel,
    pub text: String,
    pub timestamp: Instant,
}

impl LoggingEvent {
    /// Create a new logging event
    ///
    /// # Arguments
    /// * `log_level` - The log level of the event
    /// * `text` - The log message
    ///
    /// # Returns
    /// * `LoggingEvent` - The created logging event
    fn new(log_level: LogLevel, text: &str) -> Self {
        Self::new_at(log_level, text, Instant::now())
    }

    /// Create a new logging event with a specific timestamp
    ///
    /// # Arguments
    /// * `log_level` - The log level of the event
    /// * `text` - The log message
    /// * `timestamp` - The timestamp of the event
    ///
    /// # Returns
    /// * `LoggingEvent` - The created logging event
    fn new_at(log_level: LogLevel, text: &str, timestamp: Instant) -> Self {
        Self {
            log_level,
            text: String::from(text),
            timestamp,
        }
    }

    /// Create a new info logging event
    pub fn info(text: &str) -> Self {
        Self::info_at(text, Instant::now())
    }

    /// Create a new info logging event with a specific timestamp
    pub fn info_at(text: &str, timestamp: Instant) -> Self {
        Self::new_at(LogLevel::Info, text, timestamp)
    }

    /// Create a new debug logging event
    pub fn debug(text: &str) -> Self {
        Self::debug_at(text, Instant::now())
    }

    /// Create a new debug logging event with a specific timestamp
    pub fn debug_at(text: &str, timestamp: Instant) -> Self {
        Self::new_at(LogLevel::Debug, text, timestamp)
    }

    /// Create a new warning logging event
    pub fn warning(text: &str) -> Self {
        Self::warning_at(text, Instant::now())
    }

    /// Create a new warning logging event with a specific timestamp
    pub fn warning_at(text: &str, timestamp: Instant) -> Self {
        Self::new_at(LogLevel::Warning, text, timestamp)
    }

    /// Create a new error logging event
    pub fn error(text: &str) -> Self {
        Self::error_at(text, Instant::now())
    }

    /// Create a new error logging event with a specific timestamp
    pub fn error_at(text: &str, timestamp: Instant) -> Self {
        Self::new_at(LogLevel::Error, text, timestamp)
    }
}
