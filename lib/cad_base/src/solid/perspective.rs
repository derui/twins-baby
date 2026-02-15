use std::collections::HashMap;

use crate::{
    id::{IdStore, SolidId},
    solid::Solid,
};

/// Perspective for Solid
pub struct SolidPerspective {
    // Solid that created by feature
    solids: HashMap<SolidId, Solid>,

    solid_id_gen: IdStore<SolidId>,
}

impl Default for SolidPerspective {
    fn default() -> Self {
        Self {
            solids: Default::default(),
            solid_id_gen: IdStore::of(),
        }
    }
}

impl SolidPerspective {
    /// Get new [SolidPerspective]
    pub fn new() -> Self {
        Self::default()
    }

    /// Get a solid reference
    pub fn get(&self, id: &SolidId) -> Option<&Solid> {
        self.solids.get(id)
    }

    /// Get a solid mutable reference
    pub fn get_mut(&mut self, id: &SolidId) -> Option<&mut Solid> {
        self.solids.get_mut(id)
    }

    /// Add a solid
    pub fn add(&mut self, solid: Solid) -> SolidId {
        let id = self.solid_id_gen.generate();

        self.solids.insert(id, solid);
        id
    }

    /// Remove a solid by [id]
    pub fn remove(&mut self, id: &SolidId) -> Option<Solid> {
        self.solids.remove(id)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        id::{IdStore, SolidId},
        solid::{SolidBuilder, SolidPerspective},
    };

    fn make_solid() -> crate::solid::Solid {
        SolidBuilder::default().build()
    }

    #[test]
    fn add_returns_different_ids_for_each_solid() {
        // Arrange
        let mut perspective = SolidPerspective::new();

        // Act
        let id1 = perspective.add(make_solid());
        let id2 = perspective.add(make_solid());

        // Assert
        assert_ne!(id1, id2);
    }

    #[test]
    fn get_returns_some_for_existing_id() {
        // Arrange
        let mut perspective = SolidPerspective::new();
        let id = perspective.add(make_solid());

        // Act
        let result = perspective.get(&id);

        // Assert
        assert!(result.is_some());
    }

    #[test]
    fn get_returns_none_for_nonexistent_id() {
        // Arrange
        let perspective = SolidPerspective::new();
        let mut id_store: IdStore<SolidId> = IdStore::of();
        let id = id_store.generate();

        // Act
        let result = perspective.get(&id);

        // Assert
        assert!(result.is_none());
    }

    #[test]
    fn get_mut_returns_some_for_existing_id() {
        // Arrange
        let mut perspective = SolidPerspective::new();
        let id = perspective.add(make_solid());

        // Act
        let result = perspective.get_mut(&id);

        // Assert
        assert!(result.is_some());
    }

    #[test]
    fn get_mut_returns_none_for_nonexistent_id() {
        // Arrange
        let mut perspective = SolidPerspective::new();
        let mut id_store: IdStore<SolidId> = IdStore::of();
        let id = id_store.generate();

        // Act
        let result = perspective.get_mut(&id);

        // Assert
        assert!(result.is_none());
    }

    #[test]
    fn remove_returns_some_for_existing_id() {
        // Arrange
        let mut perspective = SolidPerspective::new();
        let id = perspective.add(make_solid());

        // Act
        let result = perspective.remove(&id);

        // Assert
        assert!(result.is_some());
    }

    #[test]
    fn remove_makes_solid_unavailable_afterwards() {
        // Arrange
        let mut perspective = SolidPerspective::new();
        let id = perspective.add(make_solid());

        // Act
        perspective.remove(&id);
        let result = perspective.get(&id);

        // Assert
        assert!(result.is_none());
    }

    #[test]
    fn remove_returns_none_for_nonexistent_id() {
        // Arrange
        let mut perspective = SolidPerspective::new();
        let mut id_store: IdStore<SolidId> = IdStore::of();
        let id = id_store.generate();

        // Act
        let result = perspective.remove(&id);

        // Assert
        assert!(result.is_none());
    }
}
