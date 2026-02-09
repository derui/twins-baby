use std::collections::HashMap;

use color_eyre::eyre::{Result, eyre};

use crate::{
    feature::{Feature, operation::Operation},
    id::{FeatureId, IdStore, SketchId},
};

/// A struct of feature perspective.
///
/// This stores ALL features in the application.
#[derive(Debug, Clone)]
pub struct FeaturePerspective {
    features: HashMap<FeatureId, Feature>,

    feature_id_gen: IdStore<FeatureId>,
}

impl Default for FeaturePerspective {
    fn default() -> Self {
        Self {
            features: Default::default(),
            feature_id_gen: IdStore::of(),
        }
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

    /// Rename a feature by id
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

#[cfg(test)]
mod tests;
