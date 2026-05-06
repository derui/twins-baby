use bevy::prelude::*;
use cad_base::{body::PlaneRef, id::BodyId};

/// A marker compoment
#[derive(Debug, Component, PartialEq, Eq, Clone, Copy)]
pub struct BodyBasePlane(pub BodyId, pub PlaneRef);
