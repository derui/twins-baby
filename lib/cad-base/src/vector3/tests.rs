mod construction {
    use approx::assert_relative_eq;

    use crate::vector3::Vector3;

    #[test]
    fn new_creates_vector_with_given_components() {
        // Arrange
        let x = 1.0;
        let y = 2.0;
        let z = 3.0;

        // Act
        let v = Vector3::new(x, y, z);

        // Assert
        assert_relative_eq!(v.x, 1.0);
        assert_relative_eq!(v.y, 2.0);
        assert_relative_eq!(v.z, 3.0);
    }

    #[test]
    fn from_tuple_creates_vector() {
        // Arrange
        let tuple = (1.0, 2.0, 3.0);

        // Act
        let v: Vector3 = tuple.into();

        // Assert
        assert_relative_eq!(v.x, 1.0);
        assert_relative_eq!(v.y, 2.0);
        assert_relative_eq!(v.z, 3.0);
    }

    #[test]
    fn into_tuple_converts_vector() {
        // Arrange
        let v = Vector3::new(1.0, 2.0, 3.0);

        // Act
        let tuple: (f32, f32, f32) = v.into();

        // Assert
        assert_eq!(tuple, (1.0, 2.0, 3.0));
    }
}

mod dot_product {

    use approx::assert_relative_eq;

    use crate::vector3::Vector3;

    #[test]
    fn dot_product_of_orthogonal_vectors_is_zero() {
        // Arrange
        let v1 = Vector3::new(1.0, 0.0, 0.0);
        let v2 = Vector3::new(0.0, 1.0, 0.0);

        // Act
        let result = v1.dot(&v2);

        // Assert
        assert_relative_eq!(result, 0.0);
    }

    #[test]
    fn dot_product_of_parallel_vectors() {
        // Arrange
        let v1 = Vector3::new(1.0, 0.0, 0.0);
        let v2 = Vector3::new(2.0, 0.0, 0.0);

        // Act
        let result = v1.dot(&v2);

        // Assert
        assert_relative_eq!(result, 2.0);
    }

    #[test]
    fn dot_product_general_case() {
        // Arrange
        let v1 = Vector3::new(1.0, 2.0, 3.0);
        let v2 = Vector3::new(4.0, 5.0, 6.0);

        // Act
        let result = v1.dot(&v2);

        // Assert
        assert_relative_eq!(result, 32.0);
    }
}

mod norm2 {
    use approx::assert_relative_eq;

    use crate::vector3::Vector3;

    #[test]
    fn norm2_of_unit_vector_is_one() {
        // Arrange
        let v = Vector3::new(1.0, 0.0, 0.0);

        // Act
        let result = v.norm2();

        // Assert
        assert_relative_eq!(result, 1.0);
    }

    #[test]
    fn norm2_of_zero_vector_is_zero() {
        // Arrange
        let v = Vector3::new(0.0, 0.0, 0.0);

        // Act
        let result = v.norm2();

        // Assert
        assert_relative_eq!(result, 0.0);
    }

    #[test]
    fn norm2_general_case() {
        // Arrange
        let v = Vector3::new(1.0, 2.0, 3.0);

        // Act
        let result = v.norm2();

        // Assert
        assert_relative_eq!(result, 14.0);
    }
}

mod cross_product {
    use approx::assert_relative_eq;

    use crate::vector3::Vector3;

    #[test]
    fn cross_product_of_unit_x_and_unit_y_is_unit_z() {
        // Arrange
        let v1 = Vector3::new(1.0, 0.0, 0.0);
        let v2 = Vector3::new(0.0, 1.0, 0.0);

        // Act
        let result = v1.cross(&v2);

        // Assert
        assert_relative_eq!(result.x, 0.0);
        assert_relative_eq!(result.y, 0.0);
        assert_relative_eq!(result.z, 1.0);
    }

    #[test]
    fn cross_product_of_parallel_vectors_is_zero() {
        // Arrange
        let v1 = Vector3::new(1.0, 2.0, 3.0);
        let v2 = Vector3::new(2.0, 4.0, 6.0);

        // Act
        let result = v1.cross(&v2);

        // Assert
        assert_relative_eq!(result.x, 0.0);
        assert_relative_eq!(result.y, 0.0);
        assert_relative_eq!(result.z, 0.0);
    }

    #[test]
    fn cross_product_is_anti_commutative() {
        // Arrange
        let v1 = Vector3::new(1.0, 2.0, 3.0);
        let v2 = Vector3::new(4.0, 5.0, 6.0);

        // Act
        let result1 = v1.cross(&v2);
        let result2 = v2.cross(&v1);

        // Assert
        assert_relative_eq!(result1.x, -result2.x);
        assert_relative_eq!(result1.y, -result2.y);
        assert_relative_eq!(result1.z, -result2.z);
    }
}

mod addition {
    use crate::vector3::Vector3;

    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn add_ref_and_ref() {
        // Arrange
        let v1 = Vector3::new(1.0, 2.0, 3.0);
        let v2 = Vector3::new(4.0, 5.0, 6.0);

        // Act
        let result = &v1 + &v2;

        // Assert
        assert_relative_eq!(result.x, 5.0);
        assert_relative_eq!(result.y, 7.0);
        assert_relative_eq!(result.z, 9.0);
    }

    #[test]
    fn add_ref_and_owned() {
        // Arrange
        let v1 = Vector3::new(1.0, 2.0, 3.0);
        let v2 = Vector3::new(4.0, 5.0, 6.0);

        // Act
        let result = &v1 + v2;

        // Assert
        assert_relative_eq!(result.x, 5.0);
        assert_relative_eq!(result.y, 7.0);
        assert_relative_eq!(result.z, 9.0);
    }

    #[test]
    fn add_owned_and_ref() {
        // Arrange
        let v1 = Vector3::new(1.0, 2.0, 3.0);
        let v2 = Vector3::new(4.0, 5.0, 6.0);

        // Act
        let result = v1 + &v2;

        // Assert
        assert_relative_eq!(result.x, 5.0);
        assert_relative_eq!(result.y, 7.0);
        assert_relative_eq!(result.z, 9.0);
    }

    #[test]
    fn add_owned_and_owned() {
        // Arrange
        let v1 = Vector3::new(1.0, 2.0, 3.0);
        let v2 = Vector3::new(4.0, 5.0, 6.0);

        // Act
        let result = v1 + v2;

        // Assert
        assert_relative_eq!(result.x, 5.0);
        assert_relative_eq!(result.y, 7.0);
        assert_relative_eq!(result.z, 9.0);
    }

    #[test]
    fn add_with_zero_vector() {
        // Arrange
        let v1 = Vector3::new(1.0, 2.0, 3.0);
        let zero = Vector3::new(0.0, 0.0, 0.0);

        // Act
        let result = v1 + zero;

        // Assert
        assert_relative_eq!(result.x, 1.0);
        assert_relative_eq!(result.y, 2.0);
        assert_relative_eq!(result.z, 3.0);
    }
}

mod subtraction {
    use crate::vector3::Vector3;

    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn sub_ref_and_ref() {
        // Arrange
        let v1 = Vector3::new(4.0, 5.0, 6.0);
        let v2 = Vector3::new(1.0, 2.0, 3.0);

        // Act
        let result = &v1 - &v2;

        // Assert
        assert_relative_eq!(result.x, 3.0);
        assert_relative_eq!(result.y, 3.0);
        assert_relative_eq!(result.z, 3.0);
    }

    #[test]
    fn sub_ref_and_owned() {
        // Arrange
        let v1 = Vector3::new(4.0, 5.0, 6.0);
        let v2 = Vector3::new(1.0, 2.0, 3.0);

        // Act
        let result = &v1 - v2;

        // Assert
        assert_relative_eq!(result.x, 3.0);
        assert_relative_eq!(result.y, 3.0);
        assert_relative_eq!(result.z, 3.0);
    }

    #[test]
    fn sub_owned_and_ref() {
        // Arrange
        let v1 = Vector3::new(4.0, 5.0, 6.0);
        let v2 = Vector3::new(1.0, 2.0, 3.0);

        // Act
        let result = v1 - &v2;

        // Assert
        assert_relative_eq!(result.x, 3.0);
        assert_relative_eq!(result.y, 3.0);
        assert_relative_eq!(result.z, 3.0);
    }

    #[test]
    fn sub_owned_and_owned() {
        // Arrange
        let v1 = Vector3::new(4.0, 5.0, 6.0);
        let v2 = Vector3::new(1.0, 2.0, 3.0);

        // Act
        let result = v1 - v2;

        // Assert
        assert_relative_eq!(result.x, 3.0);
        assert_relative_eq!(result.y, 3.0);
        assert_relative_eq!(result.z, 3.0);
    }

    #[test]
    fn sub_same_vector_gives_zero() {
        // Arrange
        let v = Vector3::new(1.0, 2.0, 3.0);

        // Act
        let result = &v - &v;

        // Assert
        assert_relative_eq!(result.x, 0.0);
        assert_relative_eq!(result.y, 0.0);
        assert_relative_eq!(result.z, 0.0);
    }
}

mod scalar_multiplication {
    use crate::vector3::Vector3;

    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn mul_ref_by_f32() {
        // Arrange
        let v = Vector3::new(1.0, 2.0, 3.0);

        // Act
        let result = &v * 2.0;

        // Assert
        assert_relative_eq!(result.x, 2.0);
        assert_relative_eq!(result.y, 4.0);
        assert_relative_eq!(result.z, 6.0);
    }

    #[test]
    fn mul_owned_by_f32() {
        // Arrange
        let v = Vector3::new(1.0, 2.0, 3.0);

        // Act
        let result = v * 2.0;

        // Assert
        assert_relative_eq!(result.x, 2.0);
        assert_relative_eq!(result.y, 4.0);
        assert_relative_eq!(result.z, 6.0);
    }

    #[test]
    fn mul_ref_by_i32() {
        // Arrange
        let v = Vector3::new(1.0, 2.0, 3.0);

        // Act
        let result = &v * 2;

        // Assert
        assert_relative_eq!(result.x, 2.0);
        assert_relative_eq!(result.y, 4.0);
        assert_relative_eq!(result.z, 6.0);
    }

    #[test]
    fn mul_owned_by_i32() {
        // Arrange
        let v = Vector3::new(1.0, 2.0, 3.0);

        // Act
        let result = v * 2;

        // Assert
        assert_relative_eq!(result.x, 2.0);
        assert_relative_eq!(result.y, 4.0);
        assert_relative_eq!(result.z, 6.0);
    }

    #[test]
    fn mul_by_zero_gives_zero_vector() {
        // Arrange
        let v = Vector3::new(1.0, 2.0, 3.0);

        // Act
        let result = v * 0.0;

        // Assert
        assert_relative_eq!(result.x, 0.0);
        assert_relative_eq!(result.y, 0.0);
        assert_relative_eq!(result.z, 0.0);
    }
}

mod scalar_division {
    use crate::vector3::Vector3;

    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn div_ref_by_f32() {
        // Arrange
        let v = Vector3::new(2.0, 4.0, 6.0);

        // Act
        let result = &v / 2.0;

        // Assert
        assert_relative_eq!(result.x, 1.0);
        assert_relative_eq!(result.y, 2.0);
        assert_relative_eq!(result.z, 3.0);
    }

    #[test]
    fn div_owned_by_f32() {
        // Arrange
        let v = Vector3::new(2.0, 4.0, 6.0);

        // Act
        let result = v / 2.0;

        // Assert
        assert_relative_eq!(result.x, 1.0);
        assert_relative_eq!(result.y, 2.0);
        assert_relative_eq!(result.z, 3.0);
    }

    #[test]
    fn div_ref_by_i32() {
        // Arrange
        let v = Vector3::new(2.0, 4.0, 6.0);

        // Act
        let result = &v / 2;

        // Assert
        assert_relative_eq!(result.x, 1.0);
        assert_relative_eq!(result.y, 2.0);
        assert_relative_eq!(result.z, 3.0);
    }

    #[test]
    fn div_owned_by_i32() {
        // Arrange
        let v = Vector3::new(2.0, 4.0, 6.0);

        // Act
        let result = v / 2;

        // Assert
        assert_relative_eq!(result.x, 1.0);
        assert_relative_eq!(result.y, 2.0);
        assert_relative_eq!(result.z, 3.0);
    }

    #[test]
    fn div_by_zero_gives_infinity() {
        // Arrange
        let v = Vector3::new(1.0, 2.0, 3.0);

        // Act
        let result = v / 0.0;

        // Assert
        assert!(result.x.is_infinite());
        assert!(result.y.is_infinite());
        assert!(result.z.is_infinite());
    }
}

mod equality {
    use crate::vector3::Vector3;

    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn equal_vectors_are_equal() {
        // Arrange
        let v1 = Vector3::new(1.0, 2.0, 3.0);
        let v2 = Vector3::new(1.0, 2.0, 3.0);

        // Act & Assert
        assert_eq!(v1, v2);
    }

    #[test]
    fn different_vectors_are_not_equal() {
        // Arrange
        let v1 = Vector3::new(1.0, 2.0, 3.0);
        let v2 = Vector3::new(1.0, 2.0, 4.0);

        // Act & Assert
        assert_ne!(v1, v2);
    }
}

mod copy_clone {
    use crate::vector3::Vector3;

    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn vector_is_copyable() {
        // Arrange
        let v1 = Vector3::new(1.0, 2.0, 3.0);

        // Act
        let v2 = v1;
        let v3 = v1; // v1 is still usable after copy

        // Assert
        assert_eq!(v2, v3);
    }

    #[test]
    fn vector_is_cloneable() {
        // Arrange
        let v1 = Vector3::new(1.0, 2.0, 3.0);

        // Act
        let v2 = v1.clone();

        // Assert
        assert_eq!(v1, v2);
    }
}

mod unit {
    use crate::vector3::Vector3;

    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn unit_of_x_axis_vector() {
        // Arrange
        let v = Vector3::new(3.0, 0.0, 0.0);

        // Act
        let result = v.unit();

        // Assert
        assert_relative_eq!(result.x, 1.0);
        assert_relative_eq!(result.y, 0.0);
        assert_relative_eq!(result.z, 0.0);
    }

    #[test]
    fn unit_of_general_vector() {
        // Arrange
        // (1, 2, 2) has norm = sqrt(1 + 4 + 4) = 3
        let v = Vector3::new(1.0, 2.0, 2.0);

        // Act
        let result = v.unit();

        // Assert
        assert_relative_eq!(result.x, 1.0 / 3.0);
        assert_relative_eq!(result.y, 2.0 / 3.0);
        assert_relative_eq!(result.z, 2.0 / 3.0);
    }

    #[test]
    fn unit_of_already_unit_vector() {
        // Arrange
        let v = Vector3::new(1.0, 0.0, 0.0);

        // Act
        let result = v.unit();

        // Assert
        assert_relative_eq!(result.x, 1.0);
        assert_relative_eq!(result.y, 0.0);
        assert_relative_eq!(result.z, 0.0);
    }

    #[test]
    fn unit_of_negative_vector() {
        // Arrange
        let v = Vector3::new(-3.0, 0.0, 0.0);

        // Act
        let result = v.unit();

        // Assert
        assert_relative_eq!(result.x, -1.0);
        assert_relative_eq!(result.y, 0.0);
        assert_relative_eq!(result.z, 0.0);
    }
}

mod from_edge {
    use super::*;
    use crate::{edge::Edge, point::Point, vector3::Vector3};
    use approx::assert_relative_eq;

    fn p(x: f32, y: f32, z: f32) -> Point {
        Point::new(x, y, z)
    }

    #[test]
    fn from_edge_along_x_axis() {
        // Arrange
        let edge = Edge::new(p(0.0, 0.0, 0.0), p(3.0, 0.0, 0.0)).unwrap();

        // Act
        let result = Vector3::from_edge(&edge);

        // Assert
        assert_relative_eq!(result.x, 3.0);
        assert_relative_eq!(result.y, 0.0);
        assert_relative_eq!(result.z, 0.0);
    }

    #[test]
    fn from_edge_general_case() {
        // Arrange
        let edge = Edge::new(p(1.0, 2.0, 3.0), p(4.0, 6.0, 8.0)).unwrap();

        // Act
        let result = Vector3::from_edge(&edge);

        // Assert
        assert_relative_eq!(result.x, 3.0);
        assert_relative_eq!(result.y, 4.0);
        assert_relative_eq!(result.z, 5.0);
    }

    #[test]
    fn from_edge_negative_direction() {
        // Arrange
        let edge = Edge::new(p(3.0, 0.0, 0.0), p(0.0, 0.0, 0.0)).unwrap();

        // Act
        let result = Vector3::from_edge(&edge);

        // Assert
        assert_relative_eq!(result.x, -3.0);
        assert_relative_eq!(result.y, 0.0);
        assert_relative_eq!(result.z, 0.0);
    }
}
