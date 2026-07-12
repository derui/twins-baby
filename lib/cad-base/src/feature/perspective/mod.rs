#[cfg(test)]
mod tests;

use std::collections::HashMap;

use color_eyre::eyre::{Result, eyre};
use tracing::instrument;

use crate::{
    feature::{Evaluate, EvaluateError, Feature, FeatureContext, operation::Operation},
    id::{FeatureId, IdStore, SketchId, SolidId},
    solid::{Solid, SolidReader},
    transaction::Baseline,
};

/// A struct of feature perspective.
///
/// This stores ALL features in the application.
#[derive(Debug, Clone)]
pub struct FeaturePerspective {
    features: HashMap<FeatureId, Feature>,

    feature_id_gen: IdStore<FeatureId>,
    solid_id_gen: IdStore<SolidId>,
}

impl Default for FeaturePerspective {
    fn default() -> Self {
        Self {
            features: Default::default(),
            feature_id_gen: IdStore::of(),
            solid_id_gen: IdStore::of(),
        }
    }
}

impl SolidReader for FeaturePerspective {
    fn read_solid(&self, id: SolidId) -> Option<&Solid> {
        self.features
            .values()
            .find_map(|f| (*f.solids).as_ref().and_then(|m| m.get(&id)))
    }
}

impl SolidReader for Baseline {
    fn read_solid(&self, id: SolidId) -> Option<&Solid> {
        self.read::<FeaturePerspective>()?.read_solid(id)
    }
}

impl FeaturePerspective {
    /// Create a new perspective
    pub fn new() -> Self {
        Self::default()
    }

    /// Get a feature reference
    pub fn get(&self, id: &FeatureId) -> Option<&Feature> {
        self.features.get(id)
    }

    /// Get a feature mutable reference
    pub fn get_mut(&mut self, id: &FeatureId) -> Option<&mut Feature> {
        self.features.get_mut(id)
    }

    /// Add a feature with operation
    pub fn add_feature(&mut self, sketch: &SketchId, operation: &Operation) -> FeatureId {
        let id = self.feature_id_gen.generate();
        self.features.insert(
            id,
            Feature::new(&id.to_string(), *sketch, operation).expect("must be success"),
        );
        id
    }

    /// Remove a feature by id
    pub fn remove_feature(&mut self, id: &FeatureId) -> Option<Feature> {
        self.features.remove(id)
    }

    /// Evaluate the feature with id, minting solid ids for produced solids.
    pub fn evaluate_feature<'a, E: Evaluate>(
        &mut self,
        id: &FeatureId,
        context: &'a FeatureContext<'a>,
    ) -> Result<(), EvaluateError> {
        let Some(feature) = self.features.get_mut(id) else {
            return Err(EvaluateError::FeatureNotFound);
        };

        feature.evaluate::<E>(context, &mut self.solid_id_gen)
    }

    /// Rename a feature by id
    #[instrument(err)]
    pub fn rename_feature(&mut self, id: &FeatureId, new_name: &str) -> Result<()> {
        if self.features.values().any(|f| *f.name == *new_name.trim()) {
            return Err(eyre!(
                "Feature with name '{}' already exists",
                new_name.trim()
            ));
        }
        let feature = self
            .features
            .get_mut(id)
            .ok_or_else(|| eyre!("Feature with id '{}' not found", id))?;

        feature.set_name(new_name)
    }
}
