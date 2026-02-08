use immutable::Im;
use solver::equation::Equation;

/// Operation definition. Each operations have some special parameters for its own.
#[derive(Debug, Clone, PartialEq)]
pub enum Operation {
    Pad(Pad),
}

/// Direction of Pad
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum PadDirection {
    #[default]
    Normal,
    /// Inverted normal
    InveredNormal,
    /// Both side
    Symmetric,
}

/// Operation
#[derive(Debug, Clone, PartialEq)]
pub struct Pad {
    /// Pad direction. Normal is
    pub direction: Im<PadDirection>,

    /// The equation to compute size of pad.
    pub size: Im<Equation>,

    _immutable: (),
}

impl Pad {
    /// Get new operation
    pub fn new(equation: &Equation) -> Self {
        Pad {
            direction: (PadDirection::default()).into(),
            size: equation.clone().into(),
            _immutable: (),
        }
    }

    /// Update direction with [direction]
    pub fn change_direction(&mut self, direction: &PadDirection) {
        self.direction = direction.clone().into();
    }

    /// Update size of padding
    pub fn change_size(&mut self, equation: &Equation) {
        self.size = equation.clone().into()
    }
}

// A simpler factory
impl From<Pad> for Operation {
    fn from(pad: Pad) -> Self {
        Operation::Pad(pad)
    }
}
