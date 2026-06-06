use bevy::math::Vec3;
use color_eyre::eyre::eyre;
use immutable::Im;
use ui_event::SketchGeometryOperation;

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

    /// Create a new [GeometryOperation] with the event
    pub fn from_geometry(geometry_type: SketchGeometryOperation) -> Self {
        match geometry_type {
            SketchGeometryOperation::LineSegment => {
                Self::new(&[GeometryOperationStep::Point, GeometryOperationStep::Point])
                    .expect("should be able to create operation by event")
            }
            SketchGeometryOperation::Rectangle => {
                Self::new(&[GeometryOperationStep::Point, GeometryOperationStep::Point])
                    .expect("should be able to create operation by event")
            }
        }
    }

    /// Forward the operation by one step with the given point.
    ///
    /// # Errors
    /// Returns an error if the operation is already completed.
    ///
    /// # Arguments
    /// * `point` - The point obtained from the mouse operation for the current step.
    pub fn forward_step(&mut self, point: Vec3) -> eyre::Result<()> {
        if self.current_step >= self.steps.len() {
            return Err(eyre!("Can not forward anymore"));
        }

        self.step_result.push(point);
        self.current_step += 1;

        Ok(())
    }

    /// Backward the operation by one step.
    ///
    /// # Errors
    /// Returns an error if the operation is already at the initial step.
    ///
    /// # Returns
    /// The point that was removed from the current step result.
    ///
    /// # Arguments
    /// * `point` - The point obtained from the mouse operation for the current step.
    pub fn backward_step(&mut self) -> eyre::Result<Vec3> {
        if self.current_step == 0 {
            return Err(eyre!("Can not backward anymore"));
        }

        let vec = self
            .step_result
            .pop()
            .expect("step_result should not be empty when current_step > 0");
        self.current_step -= 1;

        Ok(vec)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::math::Vec3;
    use eyre::Result;
    use pretty_assertions::assert_eq;

    #[test]
    fn new_returns_error_for_empty_steps() {
        // Arrange
        let steps: &[GeometryOperationStep] = &[];

        // Act
        let result = GeometryOperation::new(steps);

        // Assert
        assert!(result.is_err());
    }

    #[test]
    fn new_succeeds_with_valid_steps() {
        // Arrange
        let steps = &[GeometryOperationStep::Point];

        // Act
        let result = GeometryOperation::new(steps);

        // Assert
        assert!(result.is_ok());
    }

    #[test]
    fn forward_step_advances_through_steps() -> Result<()> {
        // Arrange
        let steps = &[GeometryOperationStep::Point, GeometryOperationStep::Point];
        let mut op = GeometryOperation::new(steps)?;
        let point = Vec3::new(1.0, 2.0, 3.0);

        // Act
        let result = op.forward_step(point);

        // Assert
        assert!(result.is_ok());

        Ok(())
    }

    #[test]
    fn forward_step_returns_error_when_all_steps_completed() -> Result<()> {
        // Arrange
        let steps = &[GeometryOperationStep::Point];
        let mut op = GeometryOperation::new(steps)?;
        op.forward_step(Vec3::ZERO)?;

        // Act
        let result = op.forward_step(Vec3::ONE);

        // Assert
        assert!(result.is_err());

        Ok(())
    }

    #[test]
    fn backward_step_returns_error_at_initial_step() -> Result<()> {
        // Arrange
        let steps = &[GeometryOperationStep::Point];
        let mut op = GeometryOperation::new(steps)?;

        // Act
        let result = op.backward_step();

        // Assert
        assert!(result.is_err());

        Ok(())
    }

    #[test]
    fn backward_step_returns_previously_forwarded_point() -> Result<()> {
        // Arrange
        let steps = &[GeometryOperationStep::Point];
        let mut op = GeometryOperation::new(steps)?;
        let point = Vec3::new(1.0, 2.0, 3.0);
        op.forward_step(point)?;

        // Act
        let returned = op.backward_step()?;

        // Assert
        assert_eq!(returned, point);

        Ok(())
    }

    #[test]
    fn forward_and_backward_are_symmetric() -> Result<()> {
        // Arrange
        let steps = &[GeometryOperationStep::Point, GeometryOperationStep::Point];
        let mut op = GeometryOperation::new(steps)?;
        let p1 = Vec3::new(1.0, 0.0, 0.0);
        let p2 = Vec3::new(0.0, 1.0, 0.0);
        op.forward_step(p1)?;
        op.forward_step(p2)?;

        // Act
        let r2 = op.backward_step()?;
        let r1 = op.backward_step()?;

        // Assert
        assert_eq!(r2, p2);
        assert_eq!(r1, p1);

        Ok(())
    }

    #[test]
    fn can_forward_again_after_backward() -> Result<()> {
        // Arrange
        let steps = &[GeometryOperationStep::Point];
        let mut op = GeometryOperation::new(steps)?;
        op.forward_step(Vec3::ZERO)?;
        op.backward_step()?;

        // Act
        let result = op.forward_step(Vec3::ONE);

        // Assert
        assert!(result.is_ok());

        Ok(())
    }

    #[test]
    fn from_geometry_line_segment_creates_two_point_steps() {
        use ui_event::SketchGeometryOperation;

        // Arrange / Act
        let op = GeometryOperation::from_geometry(SketchGeometryOperation::LineSegment);

        // Assert
        assert_eq!(
            op.steps.as_slice(),
            &[GeometryOperationStep::Point, GeometryOperationStep::Point]
        );
    }

    #[test]
    fn from_geometry_rectangle_creates_four_point_steps() {
        use ui_event::SketchGeometryOperation;

        // Arrange / Act
        let op = GeometryOperation::from_geometry(SketchGeometryOperation::Rectangle);

        // Assert
        assert_eq!(
            op.steps.as_slice(),
            &[GeometryOperationStep::Point, GeometryOperationStep::Point,]
        );
    }
}
