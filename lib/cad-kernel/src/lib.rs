use cad_base::{
    feature::{Evaluate, EvaluateError, Feature, FeatureContext, operation::Operation},
    solid::Solid,
};

use crate::pad::PadKernel;

mod pad;
mod sketcher;

/// Kernel for operation. this empty struct only use for static dispatch.
#[derive(Debug)]
pub struct OperationKernel;

impl Evaluate for OperationKernel {
    fn evaluate<'a>(
        feature: &Feature,
        context: &FeatureContext<'a>,
    ) -> color_eyre::eyre::Result<Vec<Solid>, EvaluateError> {
        match *feature.operation {
            Operation::Pad(_) => PadKernel::evaluate(feature, context),
        }
    }
}
