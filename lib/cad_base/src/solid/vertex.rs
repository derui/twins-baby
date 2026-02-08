use immutable::Im;

/// Immutable vertex for solid.
#[derive(Debug, Clone, PartialEq)]
pub struct Vertex {
    pub x: Im<f32>,
    pub y: Im<f32>,
    pub z: Im<f32>,
}

impl Vertex {
    /// Get new vertex
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vertex {
            x: x.into(),
            y: y.into(),
            z: z.into(),
        }
    }

    /// Get new vertex with modified x, y, or z
    ///
    /// # Examples
    /// ```
    /// let v1 = Vertex::new(1.0, 2.0, 3.0);
    /// let v2 = v1.with_x(4.0);
    /// assert_eq!(*v2.x, 4.0);
    /// assert_eq!(*v2.y, 2.0);
    /// assert_eq!(*v2.z, 3.0);
    /// ```
    pub fn with_x(&self, x: f32) -> Self {
        Vertex {
            x: x.into(),
            y: self.y.clone(),
            z: self.z.clone(),
        }
    }

    /// Get new vertex with modified y
    ///
    /// # Examples
    /// ```
    /// let v1 = Vertex::new(1.0, 2.0, 3.0);
    /// let v2 = v1.with_y(4.0);
    /// assert_eq!(*v2.x, 1.0);
    /// assert_eq!(*v2.y, 4.0);
    /// assert_eq!(*v2.z, 3.0);
    /// ```
    pub fn with_y(&self, y: f32) -> Self {
        Vertex {
            y: y.into(),
            x: self.x.clone(),
            z: self.z.clone(),
        }
    }

    /// Get new vertex with modified z
    ///
    /// # Examples
    /// ```
    /// let v1 = Vertex::new(1.0, 2.0, 3.0);
    /// let v2 = v1.with_z(4.0);
    /// assert_eq!(*v2.x, 1.0);
    /// assert_eq!(*v2.y, 2.0);
    /// assert_eq!(*v2.z, 4.0);
    /// ```
    pub fn with_z(&self, z: f32) -> Self {
        Vertex {
            z: z.into(),
            x: self.x.clone(),
            y: self.y.clone(),
        }
    }
}

// Default must be origin point.
impl Default for Vertex {
    fn default() -> Self {
        Vertex::new(0.0, 0.0, 0.0)
    }
}

impl From<(f32, f32, f32)> for Vertex {
    fn from(v: (f32, f32, f32)) -> Self {
        Vertex::new(v.0, v.1, v.2)
    }
}

impl From<Vertex> for (f32, f32, f32) {
    fn from(v: Vertex) -> Self {
        (*v.x, *v.y, *v.z)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    #[rstest]
    #[case(0.0, 0.0, 0.0)]
    #[case(1.0, 2.0, 3.0)]
    #[case(-1.5, 0.0, 99.9)]
    fn new_creates_vertex_with_given_coordinates(#[case] x: f32, #[case] y: f32, #[case] z: f32) {
        // Act
        let v = Vertex::new(x, y, z);

        // Assert
        assert_relative_eq!(*v.x, x);
        assert_relative_eq!(*v.y, y);
        assert_relative_eq!(*v.z, z);
    }

    #[test]
    fn with_x_returns_new_vertex_with_only_x_changed() {
        // Arrange
        let v = Vertex::new(1.0, 2.0, 3.0);

        // Act
        let v2 = v.with_x(10.0);

        // Assert
        assert_relative_eq!(*v2.x, 10.0);
        assert_relative_eq!(*v2.y, 2.0);
        assert_relative_eq!(*v2.z, 3.0);
    }

    #[test]
    fn with_y_returns_new_vertex_with_only_y_changed() {
        // Arrange
        let v = Vertex::new(1.0, 2.0, 3.0);

        // Act
        let v2 = v.with_y(10.0);

        // Assert
        assert_relative_eq!(*v2.x, 1.0);
        assert_relative_eq!(*v2.y, 10.0);
        assert_relative_eq!(*v2.z, 3.0);
    }

    #[test]
    fn with_z_returns_new_vertex_with_only_z_changed() {
        // Arrange
        let v = Vertex::new(1.0, 2.0, 3.0);

        // Act
        let v2 = v.with_z(10.0);

        // Assert
        assert_relative_eq!(*v2.x, 1.0);
        assert_relative_eq!(*v2.y, 2.0);
        assert_relative_eq!(*v2.z, 10.0);
    }

    #[test]
    fn default_is_origin() {
        // Act
        let v = Vertex::default();

        // Assert
        assert_relative_eq!(*v.x, 0.0);
        assert_relative_eq!(*v.y, 0.0);
        assert_relative_eq!(*v.z, 0.0);
    }

    #[test]
    fn from_tuple_creates_vertex() {
        // Act
        let v: Vertex = (1.0_f32, 2.0_f32, 3.0_f32).into();

        // Assert
        assert_relative_eq!(*v.x, 1.0);
        assert_relative_eq!(*v.y, 2.0);
        assert_relative_eq!(*v.z, 3.0);
    }

    #[test]
    fn into_tuple_extracts_coordinates() {
        // Arrange
        let v = Vertex::new(1.0, 2.0, 3.0);

        // Act
        let (x, y, z): (f32, f32, f32) = v.into();

        // Assert
        assert_relative_eq!(x, 1.0);
        assert_relative_eq!(y, 2.0);
        assert_relative_eq!(z, 3.0);
    }

    #[test]
    fn equality_for_same_coordinates() {
        // Arrange
        let v1 = Vertex::new(1.0, 2.0, 3.0);
        let v2 = Vertex::new(1.0, 2.0, 3.0);

        // Assert
        assert_eq!(v1, v2);
    }

    #[test]
    fn inequality_for_different_coordinates() {
        // Arrange
        let v1 = Vertex::new(1.0, 2.0, 3.0);
        let v2 = Vertex::new(1.0, 2.0, 4.0);

        // Assert
        assert_ne!(v1, v2);
    }

    #[test]
    fn clone_produces_equal_vertex() {
        // Arrange
        let v1 = Vertex::new(1.0, 2.0, 3.0);

        // Act
        let v2 = v1.clone();

        // Assert
        assert_eq!(v1, v2);
    }
}
