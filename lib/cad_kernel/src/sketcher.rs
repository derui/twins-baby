use cad_base::{
    feature::AttachedTarget,
    sketch::{AttachableTarget, Sketch},
};
use color_eyre::eyre::{Result, eyre};

/// Sketcher derives closed surface that is basement of the kernel.
#[derive(Debug, Clone)]
pub(crate) struct Sketcher<'a> {
    /// Target sketch
    sketch: &'a Sketch,

    /// attached target of the sketcher
    target: &'a AttachedTarget<'a>,
}

impl Sketcher<'_> {
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
}
