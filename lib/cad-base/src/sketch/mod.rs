mod propagation;
mod registrar;
mod shape;
#[cfg(test)]
mod tests;

use std::collections::HashMap;

use anyhow::Result;
use solver::{environment::Environment, variable::Variable};

use crate::{
    edge::Edge,
    id::{DefaultIdGenerator, EdgeId, GenerateId, PlaneId, PointId, SketchId},
    point::Point,
};

/// A internal information of edge.
///
/// Sketch do not have original edge information as is, need in sketch operation.
#[derive(Debug, Clone)]
struct EdgeExtraction {
    start_point_id: PointId,
    end_point_id: PointId,
    length_variable_name: String,
}

/// The sketch of base of modeling.
///
/// [Sketch] has these values:
///
/// - point and edges in the sketchs
/// - variables for all points in this sketch
/// - attached Plane with plane id.
/// - constraints equations for points (not implemented yet)
///
/// All points and edges as unique in the sketch.
#[derive(Debug, Clone)]
pub struct Sketch {
    id: SketchId,
    edge_id_gen: Box<dyn GenerateId<EdgeId>>,
    point_id_gen: Box<dyn GenerateId<PointId>>,

    /// Points in this sketch
    points: HashMap<PointId, Point>,

    /// Edges in this sketch
    edges: HashMap<EdgeId, EdgeExtraction>,

    /// variables for solver
    variables: Environment,

    /// A plane atteched to sketch
    attached_plane: PlaneId,
}

/// The builder structure of the sketch
#[derive(Debug)]
pub struct SketchBuilder {
    pub edge_id_gen: Box<dyn GenerateId<EdgeId>>,
    pub point_id_gen: Box<dyn GenerateId<PointId>>,
    pub attached_plane: Option<PlaneId>,
}

impl Default for SketchBuilder {
    fn default() -> Self {
        Self {
            edge_id_gen: Box::new(DefaultIdGenerator::default()),
            point_id_gen: Box::new(DefaultIdGenerator::default()),
            attached_plane: None,
        }
    }
}

impl Sketch {
    /// Create a new sketch with builder
    ///
    /// ```rust
    /// Sketch::new(SketchId::new(1), SketchBuilder {
    ///   attached_plane: Some(PlaneId(3)),
    ///   ..Default::default()
    /// })
    /// ```
    pub fn new(id: SketchId, builder: SketchBuilder) -> Result<Self> {
        let Some(attached_plane) = builder.attached_plane else {
            return Err(anyhow::anyhow!("Must set attached_plane"));
        };

        Ok(Sketch {
            id,
            edge_id_gen: builder.edge_id_gen,
            point_id_gen: builder.point_id_gen,
            points: HashMap::new(),
            edges: HashMap::new(),
            variables: Environment::empty(),
            attached_plane,
        })
    }

    /// Return reference of variables in the sketch
    pub fn variables(&self) -> &Environment {
        &self.variables
    }

    /// Add a point to sketch.
    ///
    /// # Summary
    /// Add a point to sketch and define variables from the point.
    pub fn add_point(&mut self, point: &Point) -> PointId {
        let id = self.point_id_gen.generate();

        self.add_point_raw(id, point);
        id
    }

    /// Remove the point from sketch
    ///
    /// # Summary
    /// Remove the point from sketch, and remove variables are related the point
    pub fn remove_point(&mut self, id: &PointId) -> Option<Point> {
        let id_value = id.id();
        self.variables.remove_variable(&format!("x{}", id_value));
        self.variables.remove_variable(&format!("y{}", id_value));
        self.variables.remove_variable(&format!("z{}", id_value));

        self.points.remove(id)
    }

    fn add_point_raw(&mut self, id: PointId, point: &Point) {
        self.points.insert(id, *point);

        // make variables with id.
        let id = id.id();
        self.variables
            .add_variable(Variable::new(&format!("x{}", id), *point.x()));
        self.variables
            .add_variable(Variable::new(&format!("y{}", id), *point.y()));
        self.variables
            .add_variable(Variable::new(&format!("z{}", id), *point.z()));
    }

    /// Add a edge to sketch.
    ///
    /// # Summary
    /// Add a edge to sketch and define variables from the edge.
    pub fn add_edge(&mut self, edge: &Edge) -> EdgeId {
        let id = self.edge_id_gen.generate();
        let start_id = self.point_id_gen.generate();
        let end_id = self.point_id_gen.generate();

        let extraction = EdgeExtraction {
            start_point_id: start_id,
            end_point_id: end_id,
            length_variable_name: format!("edge_{}", id.id()),
        };

        self.add_point_raw(start_id, edge.start());
        self.add_point_raw(end_id, edge.end());

        // define variable with current length of edge
        self.variables
            .add_variable(Variable::new(&extraction.length_variable_name, edge.len()));
        self.edges.insert(id, extraction);
        id
    }

    /// Remove the edge from the sketch
    ///
    /// # Summary
    /// Remove the edge of [id] from this sketch, and remove variables related of this.
    pub fn remove_edge(&mut self, id: &EdgeId) -> Option<Edge> {
        let Some(edge) = self.edges.remove(id) else {
            return None;
        };

        // remove points
        let Some(start) = self.remove_point(&edge.start_point_id) else {
            return None;
        };
        let Some(end) = self.remove_point(&edge.end_point_id) else {
            return None;
        };
        self.variables.remove_variable(&format!("edge_{}", id.id()));

        Some(Edge::new(start, end).expect("Should be valid edge"))
    }
}
