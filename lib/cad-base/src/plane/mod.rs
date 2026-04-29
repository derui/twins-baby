#[cfg(test)]
mod tests;

use std::{collections::HashMap, marker::PhantomData};

use color_eyre::eyre::Result;
use epsilon::{DefaultEpsilon, Epsilon, approx_zero};
use immutable::Im;
use tracing::instrument;

use crate::{
    id::{IdStore, PlaneId},
    point::Point,
    sketch::Point2,
    vector3::Vector3,
};

/// A perspective for Plane. This has planes in Bodies, but can get by id
pub struct PlanePerspective<E: Epsilon = DefaultEpsilon> {
    /// All planes in application
    planes: HashMap<PlaneId, Plane<E>>,

    /// Id generator for store
    id_gen: IdStore<PlaneId>,
}

impl<E: Epsilon> PlanePerspective<E> {
    /// Create a new perspective
    pub fn new() -> Self {
        PlanePerspective {
            planes: HashMap::new(),
            id_gen: IdStore::of(),
        }
    }

    /// Add a plane to perspective and get the id
    pub fn add_plane(&mut self, plane: Plane<E>) -> PlaneId {
        let id = self.id_gen.generate();
        self.planes.insert(id, plane);
        id
    }

    /// Get a reference to a plane by id
    pub fn get(&self, id: &PlaneId) -> Option<&Plane<E>> {
        self.planes.get(id)
    }

    /// Get a mutable reference to a plane by id
    pub fn get_mut(&mut self, id: &PlaneId) -> Option<&mut Plane<E>> {
        self.planes.get_mut(id)
    }

    /// Remove the plane
    pub fn remove(&mut self, id: &PlaneId) -> Option<Plane<E>> {
        self.planes.remove(id)
    }
}

/// Simple plane definition.
#[derive(Debug, Clone, PartialEq)]
pub struct Plane<E: Epsilon = DefaultEpsilon> {
    /// normal vector of the vector
    pub normal: Im<Vector3>,

    /// point on the plane
    pub r0: Im<Point>,

    _data: PhantomData<E>,
}

impl<E: Epsilon> Plane<E> {
    /// Create a new plane that makes 2 edges and crossed the 2 edges.
    #[instrument(err)]
    pub fn new(edge1: (&Point, &Point), edge2: (&Point, &Point)) -> Result<Self> {
        let v1 = Vector3::from_points(edge1.0, edge1.1);
        let v2 = Vector3::from_points(edge2.0, edge2.1);

        let crossed = v1.cross(&v2);

        // If crossed vector near 0, edges are same
        if crossed.norm2().abs() < 1e-5 {
            Err(color_eyre::eyre::eyre!(
                "Can not define plane from same edges"
            ))
        } else {
            Ok(Plane {
                normal: crossed.unit().into(),
                r0: edge1.0.clone().into(),
                _data: PhantomData,
            })
        }
    }

    /// Get a new [Plane] with parametric arguments
    pub fn new_with_parametric(normal: &Vector3, r: &Point) -> Self {
        Plane {
            normal: normal.clone().unit().into(),
            r0: r.clone().into(),
            _data: PhantomData,
        }
    }

    /// A new XY-plane. It contains origin and Z-unit vector.
    pub fn new_xy() -> Self {
        Self::new_with_parametric(&Vector3::new_z_unit(), &Point::zero())
    }

    /// A new XZ-plane. It contains origin and Y-unit vector.
    pub fn new_xz() -> Self {
        Self::new_with_parametric(&Vector3::new_y_unit(), &Point::zero())
    }

    /// A new YZ-plane. It contains origin and X-unit vector.
    pub fn new_yz() -> Self {
        Self::new_with_parametric(&Vector3::new_x_unit(), &Point::zero())
    }

    /// Get normal-inverted plane
    pub fn normal_inverted(&self) -> Self {
        let inverted = *self.normal * -1;
        Plane::new_with_parametric(&inverted, &self.r0)
    }

    /// Check the [point] on the plane or not
    pub fn is_on_plane(&self, point: &Point) -> bool {
        let r0: Vector3 = (*self.r0).clone().into();
        let r: Vector3 = point.into();

        let ret = self.normal.dot(&(r0 - r));

        approx_zero::<E>(ret.abs())
    }

    /// Get the nearest vector to avoid shrink cross
    fn nearest_normal(&self) -> Vector3 {
        if self.normal.x <= self.normal.y && self.normal.x <= self.normal.z {
            Vector3::new_x_unit()
        } else if self.normal.y <= self.normal.x && self.normal.y <= self.normal.z {
            Vector3::new_y_unit()
        } else {
            Vector3::new_z_unit()
        }
    }

    /// Project [`Point2`] to [`Point`] on this plane
    pub fn point_from_2d(&self, point: &Point2) -> Point {
        let e1 = self.normal.cross(&self.nearest_normal()).unit();
        let e2 = self.normal.cross(&e1);

        let u = e1 * *point.x;
        let v = e2 * *point.y;

        let r = Vector3::from(&*self.r0) + u + v;
        Point::from_vector3(&r)
    }
}
