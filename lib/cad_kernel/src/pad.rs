use cad_base::{
    feature::{
        Evaluate, EvaluateError, Feature, FeatureContext,
        operation::{Operation, Pad},
    },
    solid::Solid,
};
use color_eyre::eyre::Result;

/// The kernel for pad operation.
#[derive(Debug, Clone)]
pub struct PadKernel;

fn compute_pad<'a>(
    _pad: &Pad,
    _feature: &Feature,
    context: &FeatureContext<'a>,
) -> Result<Solid, EvaluateError> {
    if context.sketches.len() != 1 {
        return Err(EvaluateError::InsufficientSketch);
    }

    let _sketch = context.sketches[0];

    todo!()
}

/// Implementation of pad kernel
impl Evaluate for PadKernel {
    fn evaluate<'a>(
        feature: &Feature,
        context: &FeatureContext<'a>,
    ) -> Result<Solid, EvaluateError> {
        match &(*feature.operation) {
            Operation::Pad(pad) => compute_pad(pad, feature, context),
        }
    }
}
