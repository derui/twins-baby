use bevy::prelude::*;
use cad_base::body::PlaneRef;

/// A marker compoment
#[derive(Debug, Component, PartialEq, Eq, Clone, Copy)]
pub struct BodyBasePlane(pub PlaneRef);
