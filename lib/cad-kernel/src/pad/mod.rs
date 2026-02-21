#[cfg(test)]
mod tests;

use crate::sketcher::{JordanCurve, Sketcher};
use cad_base::{
    feature::{
        AttachedTarget, Evaluate, EvaluateError, Feature, FeatureContext,
        operation::{Operation, Pad, PadDirection},
    },
    id::{EdgeId, VertexId},
    plane::Plane,
    point::Point,
    solid::{
        Solid, SolidBuilder,
        edge::Edge,
        face::{Face, PlanarSurface},
    },
    vector3::Vector3,
};
use color_eyre::eyre::Result;
use epsilon::DefaultEpsilon;
use solver::{environment::Environment, equation::Evaluate as _};

/// The kernel for pad operation.
#[derive(Debug, Clone)]
pub struct PadKernel;

/// Compute moved vertex and register it.
fn compute_moved_face(
    builder: &mut SolidBuilder,
    curve: &JordanCurve,
    plane: &Plane,
    length: f32,
) -> (Vec<VertexId>, Vec<EdgeId>) {
    let moved_points: Vec<_> = curve
        .points
        .iter()
        .map(|p| {
            let vec: Vector3 = p.into();

            Point::from_vector3(&(vec + (*plane.normal * length))).into()
        })
        .collect();

    let vertex_ids = builder.add_vertices(&moved_points);

    let edges: Vec<_> = curve
        .edges
        .iter()
        .filter_map(|(start, end)| {
            let start = vertex_ids.get(*start)?;
            let end = vertex_ids.get(*end)?;

            Some(Edge::new(*start, *end).expect("Must be success"))
        })
        .collect();

    assert!(
        edges.len() == curve.edges.len(),
        "Must keep same number of edges from sketch"
    );

    let edge_ids = builder.add_edges(&edges);

    builder.add_faces(&[Face::Planar(
        PlanarSurface::new(&edge_ids, plane).expect("should be valid"),
    )]);

    (vertex_ids, edge_ids)
}

/// Compute faces surrounding of the solid
fn compute_surrounding_faces(
    builder: &mut SolidBuilder,
    first: &(Vec<VertexId>, Vec<EdgeId>),
    second: &(Vec<VertexId>, Vec<EdgeId>),
) {
    let (_, f_edge) = first;
    let (_, s_edge) = second;

    assert!(
        f_edge.len() == s_edge.len(),
        "Must same first and second face's edges {} <> {}",
        f_edge.len(),
        s_edge.len()
    );

    for idx in 0..f_edge.len() {
        let f_edge_id = f_edge[idx];
        let s_edge_id = s_edge[idx];

        // make edge from f_start to s_start, and edge from f_end to s_end, and then make face from these 4 edges.
        let f_edge = builder.get_edge(&f_edge_id).expect("Must be exist").clone();
        let s_edge = builder.get_edge(&s_edge_id).expect("Must be exist").clone();

        let new_edge_f = builder
            .get_edge_by_pair(&f_edge.start, &s_edge.start)
            .unwrap_or_else(|| {
                let e = Edge::new(*f_edge.start, *s_edge.start).expect("Must be success");
                builder.add_edges(&[e])[0]
            });

        let new_edge_e = builder
            .get_edge_by_pair(&f_edge.end, &s_edge.end)
            .unwrap_or_else(|| {
                let e = Edge::new(*f_edge.end, *s_edge.end).expect("Must be success");
                builder.add_edges(&[e])[0]
            });

        // compute normal vector of the face
        let edge1 = (
            &**builder.get_vertex(&f_edge.start).expect("Must be exist"),
            &**builder.get_vertex(&f_edge.end).expect("Must be exist"),
        );
        let edge2 = (
            &**builder.get_vertex(&f_edge.start).expect("Must be exist"),
            &**builder.get_vertex(&s_edge.start).expect("Must be exist"),
        );

        let plane =
            Plane::<DefaultEpsilon>::new(edge1, edge2).expect("This plane must be creatable");
        let face = Face::Planar(
            PlanarSurface::new(&[f_edge_id, s_edge_id, new_edge_f, new_edge_e], &plane)
                .expect("This face must be creatable"),
        );
        builder.add_faces(&[face]);
    }
}

/// Get the plane if the points need to move align the plane. If no need, return None
fn get_first_plane(pad: &Pad, plane: &Plane) -> Option<Plane> {
    match *pad.direction {
        PadDirection::Normal => None,
        PadDirection::InveredNormal => None,
        PadDirection::Symmetric => Some(plane.clone()),
    }
}

/// Get the plane if the points need to move align the second plane
fn get_second_plane(pad: &Pad, plane: &Plane) -> Plane {
    match *pad.direction {
        PadDirection::Normal => plane.clone(),
        PadDirection::InveredNormal => plane.normal_inverted(),
        PadDirection::Symmetric => plane.normal_inverted(),
    }
}

#[tracing::instrument(err)]
fn compute_pad<'a>(
    pad: &Pad,
    _feature: &Feature,
    context: &FeatureContext<'a>,
) -> Result<Vec<Solid>, EvaluateError> {
    if context.sketches.len() != 1 {
        return Err(EvaluateError::InsufficientSketch);
    }

    let sketcher = Sketcher::new(context.sketches[0], &context.target[0])
        .map_err(|e| EvaluateError::HaveSomeInvalidSketches(e.into()))?;
    let curves = sketcher
        .calculate_jordan_corves::<DefaultEpsilon>()
        .map_err(|e| EvaluateError::HaveSomeInvalidSketches(e.into()))?;

    let plane = match context.target[0] {
        AttachedTarget::Plane(plane) => plane,
        AttachedTarget::Face(_face) => todo!(),
    };

    let mut ret = Vec::new();

    // make solid from curve.
    // 1. register initial face and vertices, edges.
    // 2. copies moved vertiecs with operation, and then register it and get new face.
    // 3. and then, same index point makes new edge, and 4 edges makes a face.
    let length = (*pad.size)
        .evaluate(&Environment::empty())
        .expect("This equation must not to use variable now");
    for curve in &curves {
        let mut solid = SolidBuilder::default();

        let first_planes;
        {
            // register initial face
            let first_plane = get_first_plane(pad, plane);
            first_planes = compute_moved_face(
                &mut solid,
                curve,
                first_plane.as_ref().unwrap_or(plane),
                match *pad.direction {
                    PadDirection::Normal => 0.0,
                    PadDirection::InveredNormal => 0.0,
                    PadDirection::Symmetric => length,
                },
            );
        }

        let second_planes;
        {
            // register second face
            let second_plane = get_second_plane(pad, plane);
            second_planes = compute_moved_face(&mut solid, curve, &second_plane, length);
        }

        compute_surrounding_faces(&mut solid, &first_planes, &second_planes);
        ret.push(solid.build())
    }

    Ok(ret)
}

/// Implementation of pad kernel
impl Evaluate for PadKernel {
    fn evaluate<'a>(
        feature: &Feature,
        context: &FeatureContext<'a>,
    ) -> Result<Vec<Solid>, EvaluateError> {
        match &(*feature.operation) {
            Operation::Pad(pad) => compute_pad(pad, feature, context),
        }
    }
}
