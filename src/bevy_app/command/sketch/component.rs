use bevy::math::{USizeVec3, Vec3};
use color_eyre::eyre::eyre;
use immutable::Im;

/// The step definition for mouse operation to create geometry in a sketch.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GeometryOperationStep {
    /// Add a point via mouse
    Point,
}

/// Defines an operation used to create geometry within a sketch.
///
/// This structure encapsulates the necessary information to instruct a CAD kernel
/// on how to perform a geometric construction, such as joining, subtracting,
/// or intersecting existing sketches elements.
#[derive(Debug, Clone)]
pub struct GeometryOperation {
    /// Steps of the operation
    pub steps: Im<Vec<GeometryOperationStep>>,

    /// Result of mouse operations each steps
    step_result: Vec<Vec3>,

    /// current step
    current_step: usize,
}

impl GeometryOperation {
    /// Create a new GeometryOperation with the given steps.
    ///
    /// # Errors
    /// Returns an error if the steps are empty.
    pub fn new(steps: &[GeometryOperationStep]) -> eyre::Result<Self> {
        let steps = Vec::from(steps);

        if steps.is_empty() {
            return Err(eyre!("Operation steps must not be empty"));
        }

        Ok(GeometryOperation {
            steps: steps.into(),
            step_result: vec![],
            current_step: 0,
        })
    }
}
