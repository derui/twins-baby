use std::ops::Deref;

use bevy::prelude::*;
use cad_base::{body::PlaneRef, id::BodyId};

/// A marker compoment
#[derive(Debug, Component, PartialEq, Eq, Clone, Copy)]
pub struct BodyBasePlane(PlaneRef);

impl From<PlaneRef> for BodyBasePlane {
    fn from(value: PlaneRef) -> Self {
        Self(value)
    }
}

impl Deref for BodyBasePlane {
    type Target = PlaneRef;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
