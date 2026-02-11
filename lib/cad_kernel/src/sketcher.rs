use cad_base::{
    feature::AttachedTarget, point::Point, sketch::{AttachableTarget, Sketch}
};
use color_eyre::eyre::{Result, eyre};
use epsilon::Epsilon;

/// struct of representation of Jordan Curve.
///
/// Currently, all sketch needs to construct valid jordan curves.
pub(crate) struct JordanCurve {
    /// 3D points of curve
    pub points: Vec<Point>,
    /// Edges of points indices. first is start, second is end.
    edges: Vec<(usize, usize)>,
}

/// Sketcher derives closed surface that is basement of the kernel.
#[derive(Debug, Clone)]
pub(crate) struct Sketcher<'a> {
    /// Target sketch
    sketch: &'a Sketch,

    /// attached target of the sketcher
    target: &'a AttachedTarget<'a>,
}

#[derive(Debug, Clone, thiserror::Error)]
pub(crate) enum SketcherError {
    #[error("The sketch does not have any edges")]
    SketchNotHaveEdge,
    
    #[error("The sketch has orphan point")]
    SketchHasOrphanPoint,

    #[error("The sketch has a curve that is not jordan curve")]
    SketchHasNoJordanCurve,
}

impl Sketcher<'_> {
    /// Get a new Sketcher
    pub fn new<'a>(sketch: &'a Sketch, target: &'a AttachedTarget<'a>) -> Result<Sketcher<'a>> {
        let sketch_target: &AttachableTarget = &sketch.attach_target;

        match (sketch_target, target) {
            (AttachableTarget::Plane(_), AttachedTarget::Plane(_)) => Ok(()),
            (AttachableTarget::Plane(_), AttachedTarget::Face(_)) => {
                Err(eyre!("Invalid combination"))
            }
            (AttachableTarget::Face(_), AttachedTarget::Plane(_)) => {
                Err(eyre!("Invalid combination"))
            }
            (AttachableTarget::Face(_), AttachedTarget::Face(_)) => Ok(()),
        }?;

        Ok(Sketcher { sketch, target })
    }

    pub fn calculate_jordan_corve<E: Epsilon>(&self) -> Result<Vec<JordanCurve>, SketcherError> {
        let Ok(edges) = self.sketch.resolve_edges() else {
            return Err(SketcherError::SketchNotHaveEdge)
        };
        // flatten and distinct nearly same points as same index
        let mut all_points = edges.iter().map(|f| [f.start.clone(), f.end.clone()]).flatten().collect::<Vec<_>>();
        all_points.dedup_by(|o1, o2| o1.approx_eq::<E>(o2));

        // make adjacent list
        let adj: Vec<(usize, usize)> = Vec::new();
        todo!()
    }
}
