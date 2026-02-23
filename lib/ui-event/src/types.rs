use color_eyre::eyre;
use smol_str::SmolStr;

/// Tool selection for sketch mode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SketchTool {
    Line,
    Circle,
    Rectangle,
}

/// Kind of perspective
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
pub enum PerspectiveKind {
    #[default]
    Feature,
    Sketch,
}

impl PerspectiveKind {
    /// Create a `PerspectiveKind` from a string
    #[tracing::instrument]
    pub fn from_string(str: &str) -> eyre::Result<PerspectiveKind> {
        match str {
            "Feature" => Ok(PerspectiveKind::Feature),
            "Sketch" => Ok(PerspectiveKind::Sketch),
            _ => Err(eyre::eyre!("Invalid perspective kind: {}", str)),
        }
    }
}

impl ToString for PerspectiveKind {
    fn to_string(&self) -> String {
        match self {
            PerspectiveKind::Feature => "Feature".to_string(),
            PerspectiveKind::Sketch => "Sketch".to_string(),
        }
    }
}

impl std::str::FromStr for PerspectiveKind {
    type Err = eyre::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_string(s)
    }
}

/// Mouse button representation.
///
/// NOTE: we have bevy's mouse input, but it is for bevy, this is for our system
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MouseButton {
    Left,
    Right,
    Center,
}

/// Key representation from keyboard event.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NotifiedKey(pub SmolStr);

/// Button state representation for keyboard and other input events.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ButtonState {
    Pressed,
    Released,
}
