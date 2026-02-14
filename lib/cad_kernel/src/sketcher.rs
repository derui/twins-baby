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

    /// Calculate Jordan Corves from the sketch.
    ///
    /// If the sketch has any incorrect curves or segment, this fail with error.
    pub fn calculate_jordan_corves<E: Epsilon>(&self) -> Result<Vec<JordanCurve>, SketcherError> {
        let Ok(edges) = self.sketch.resolve_edges() else {
            return Err(SketcherError::SketchNotHaveEdge);
        };

        // make adjacent list
        let Ok(_graph) = graph::Graph::new::<E>(&edges) else {
            return Err(SketcherError::SketchNotHaveEdge);
        };

        todo!()
    }
}

mod graph {
    use std::collections::HashSet;

    use cad_base::sketch::{Point2, edge::SketchEdge};
    use color_eyre::eyre::{Result, eyre};
    use epsilon::Epsilon;

    /// A internal Graph representation
    pub struct Graph {
        // adjacent list. An index is the start point, and value is next indices from the start.
        adj: Vec<Vec<usize>>,
        points: Vec<Point2>,
    }

    impl Graph {
        /// Create a new [`Graph`]
        pub fn new<E: Epsilon>(edges: &[SketchEdge]) -> Result<Self> {
            if edges.is_empty() {
                return Err(eyre!("Edges must be greater than 0"));
            }

            // flatten and distinct nearly same points as same index
            let mut all_points = edges
                .iter()
                .flat_map(|f| [f.start.clone(), f.end.clone()])
                .collect::<Vec<_>>();
            all_points.sort_by(|o1, o2| o1.approx_total_cmp::<E>(o2));
            all_points.dedup_by(|o1, o2| o1.approx_eq::<E>(o2));

            // make adjacent list
            let mut adj: Vec<Vec<usize>> = vec![vec![]; all_points.len()];

            for edge in edges {
                let start = all_points
                    .iter()
                    .position(|v| v.approx_eq::<E>(&edge.start));
                let end = all_points.iter().position(|v| v.approx_eq::<E>(&edge.end));

                if let (Some(start), Some(end)) = (start, end) {
                    if let Some(ends) = adj.get_mut(start) {
                        ends.push(end);
                    } else {
                        adj[start] = vec![end];
                    }
                }
            }

            if adj.is_empty() {
                return Err(eyre!("Points in edge are not identical"));
            }

            Ok(Self {
                adj,
                points: all_points.into_iter().map(|v| (*v).clone()).collect(),
            })
        }

        /// Get all closed loops. Detect the branch in the loop, the loop and related points ignores.
        fn closed_loops(&self) -> Option<Vec<Vec<Point2>>> {
            let mut loops = vec![];
            let indices: HashSet<usize> =
                HashSet::from_iter(self.adj.iter().enumerate().map(|(i, _)| i));
            let mut through_points: HashSet<usize> = HashSet::new();

            let mut start = 0;
            let mut in_loop = vec![];
            while through_points.len() < indices.len() {
                in_loop.push(start);
                through_points.insert(start);

                let nexts = self.adj.get(start).expect("Should be success");

                // when detecting branch, ignore it.
                if nexts.len() != 1 {
                    return None;
                }

                // Detecting the closed loop, reset
                if in_loop[0] == nexts[0] {
                    let points: Vec<Point2> = in_loop
                        .iter()
                        .filter_map(|v| self.points.get(*v))
                        .cloned()
                        .collect();
                    loops.push(points);
                    in_loop.clear();

                    let diff: Vec<_> = indices.difference(&through_points).collect();
                    if let Some(next) = diff.first() {
                        start = **next;
                    }

                    // if no diff == all point has been throughed, continue and break.
                } else {
                    // Go next loop with next.
                    start = nexts[0];
                }
            }

            Some(loops)
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use cad_base::sketch::{Point2, edge::SketchEdge};
        use epsilon::DefaultEpsilon;
        use pretty_assertions::assert_eq;

        fn make_edge(start: (f32, f32), end: (f32, f32)) -> SketchEdge {
            SketchEdge::new(&Point2::new(start.0, start.1), &Point2::new(end.0, end.1))
        }

        #[test]
        fn test_empty_edges_returns_error() {
            // Arrange
            let edges: Vec<SketchEdge> = vec![];

            // Act
            let result = Graph::new::<DefaultEpsilon>(&edges);

            // Assert
            assert!(result.is_err());
        }

        #[test]
        fn test_triangle_gives_single_loop_of_three_points() {
            // Arrange - A(0,0) -> B(1,0) -> C(0,1) -> A(0,0)
            let edges = vec![
                make_edge((0.0, 0.0), (1.0, 0.0)),
                make_edge((1.0, 0.0), (0.0, 1.0)),
                make_edge((0.0, 1.0), (0.0, 0.0)),
            ];

            // Act
            let graph = Graph::new::<DefaultEpsilon>(&edges).expect("should build graph");
            let result = graph.closed_loops();

            // Assert
            let loops = result.expect("should have loops");
            assert_eq!(loops.len(), 1);
            assert_eq!(loops[0].len(), 3);
        }

        #[test]
        fn test_square_gives_single_loop_of_four_points() {
            // Arrange - A(0,0) -> B(1,0) -> C(1,1) -> D(0,1) -> A(0,0)
            let edges = vec![
                make_edge((0.0, 0.0), (1.0, 0.0)),
                make_edge((1.0, 0.0), (1.0, 1.0)),
                make_edge((1.0, 1.0), (0.0, 1.0)),
                make_edge((0.0, 1.0), (0.0, 0.0)),
            ];

            // Act
            let graph = Graph::new::<DefaultEpsilon>(&edges).expect("should build graph");
            let result = graph.closed_loops();

            // Assert
            let loops = result.expect("should have loops");
            assert_eq!(loops.len(), 1);
            assert_eq!(loops[0].len(), 4);
        }

        #[test]
        fn test_branching_node_returns_none() {
            // Arrange - A(0,0) has two outgoing edges to B and C
            let edges = vec![
                make_edge((0.0, 0.0), (1.0, 0.0)),
                make_edge((0.0, 0.0), (0.0, 1.0)),
            ];

            // Act
            let graph = Graph::new::<DefaultEpsilon>(&edges).expect("should build graph");
            let result = graph.closed_loops();

            // Assert
            assert!(result.is_none());
        }

        #[test]
        fn test_two_separate_triangles_give_two_loops() {
            // Arrange - two disconnected triangles
            let edges = vec![
                make_edge((0.0, 0.0), (1.0, 0.0)),
                make_edge((1.0, 0.0), (0.0, 1.0)),
                make_edge((0.0, 1.0), (0.0, 0.0)),
                make_edge((3.0, 0.0), (4.0, 0.0)),
                make_edge((4.0, 0.0), (3.0, 1.0)),
                make_edge((3.0, 1.0), (3.0, 0.0)),
            ];

            // Act
            let graph = Graph::new::<DefaultEpsilon>(&edges).expect("should build graph");
            let result = graph.closed_loops();

            // Assert
            let loops = result.expect("should have loops");
            assert_eq!(loops.len(), 2);
            assert_eq!(loops[0].len(), 3);
            assert_eq!(loops[1].len(), 3);
        }
    }
}
