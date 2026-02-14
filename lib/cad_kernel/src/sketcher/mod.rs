mod graph;

#[cfg(test)]
mod tests;

use cad_base::{
    feature::AttachedTarget,
    point::Point,
    sketch::{AttachableTarget, Sketch},
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
    pub edges: Vec<(usize, usize)>,
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

    /// Calculate Jordan Corves from the sketch.
    ///
    /// If the sketch has any incorrect curves or segment, this fail with error.
    pub fn calculate_jordan_corves<E: Epsilon>(&self) -> Result<Vec<JordanCurve>, SketcherError> {
        let Ok(edges) = self.sketch.resolve_edges() else {
            return Err(SketcherError::SketchNotHaveEdge);
        };

        // make adjacent list
        let Ok(graph) = graph::Graph::new::<E>(&edges) else {
            return Err(SketcherError::SketchNotHaveEdge);
        };

        let Some(curves) = graph.jordan_curves() else {
            return Err(SketcherError::SketchHasNoJordanCurve);
        };

        let mut ret: Vec<JordanCurve> = vec![];
        for curve in &curves {
            let edges = Vec::from_iter((0..(curve.len() - 1)).map(|v| (v, v + 1)));

            let plane = match self.target {
                AttachedTarget::Plane(plane) => *plane,
                AttachedTarget::Face(_face) => todo!("Plane from face does not implement now"),
            };

            // make them as JordanCurve on the plane
            ret.push(JordanCurve {
                points: curve.iter().map(|p| plane.point_from_2d(p)).collect(),
                edges,
            });
        }

        Ok(ret)
    }
}
