use std::fmt::Display;

use cad_base_macro::MakeId;
use color_eyre::eyre;

/// Identifier of the command.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, MakeId)]
pub struct CommandId(u64);

/// Tool of feature
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FeatureTool {
    /// Create the body
    Body,
    /// Create sketch
    Sketch,
}

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

impl Display for PerspectiveKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            PerspectiveKind::Feature => "Feature".to_string(),
            PerspectiveKind::Sketch => "Sketch".to_string(),
        };

        f.write_str(&str)
    }
}

impl std::str::FromStr for PerspectiveKind {
    type Err = eyre::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_string(s)
    }
}
