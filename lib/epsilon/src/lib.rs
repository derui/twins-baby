/// Trait for definition of epsilon for value margin
pub trait Epsilon {
    const EPSILON: f32;
}

/// Default precision of Epsilon
pub struct DefaultEpsilon;
impl Epsilon for DefaultEpsilon {
    const EPSILON: f32 = 1e-5;
}

/// High precision of Epsilon
pub struct HighPrecisionEpsitlon;
impl Epsilon for HighPrecisionEpsitlon {
    const EPSILON: f32 = 1e-9;
}

/// Low precision of Epsilon
pub struct LowPrecisionEpsitlon;
impl Epsilon for LowPrecisionEpsitlon {
    const EPSILON: f32 = 1e-3;
}

/// helper with epsilon
pub fn approx_eq<E: Epsilon>(a: f32, b: f32) -> bool {
    (a - b).abs() < E::EPSILON
}

/// helper with epsilon
pub fn approx_zero<E: Epsilon>(v: f32) -> bool {
    v.abs() < E::EPSILON
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    #[test]
    fn test_default_epsilon_value() {
        // Arrange & Act
        let epsilon = DefaultEpsilon::EPSILON;

        // Assert
        assert_eq!(epsilon, 1e-5);
    }

    #[test]
    fn test_high_precision_epsilon_value() {
        // Arrange & Act
        let epsilon = HighPrecisionEpsitlon::EPSILON;

        // Assert
        assert_eq!(epsilon, 1e-9);
    }

    #[test]
    fn test_low_precision_epsilon_value() {
        // Arrange & Act
        let epsilon = LowPrecisionEpsitlon::EPSILON;

        // Assert
        assert_eq!(epsilon, 1e-3);
    }

    #[rstest]
    #[case(1.0, 1.0, true)]
    #[case(0.0, 0.0, true)]
    #[case(-1.0, -1.0, true)]
    #[case(1.0, 1.000001, true)] // within epsilon
    #[case(1.0, 0.999999, true)] // within epsilon
    #[case(1.0, 1.00002, false)] // outside epsilon
    #[case(1.0, 0.99998, false)] // outside epsilon
    #[case(100.0, 100.000001, true)] // large values within epsilon
    #[case(-50.0, -50.000001, true)] // negative values within epsilon
    #[case(0.0, 0.000001, true)] // near zero within epsilon
    fn test_approx_eq_with_default_epsilon(#[case] a: f32, #[case] b: f32, #[case] expected: bool) {
        // Arrange - inputs provided by rstest

        // Act
        let result = approx_eq::<DefaultEpsilon>(a, b);

        // Assert
        assert_eq!(result, expected);
    }

    #[rstest]
    #[case(1.0, 1.0 + 5e-11, true)] // within high precision epsilon (note: f32 has limited precision)
    #[case(1.0, 1.0 + 0.0000000001, true)] // still within due to f32 precision limits
    #[case(0.0, 5e-11, true)] // near zero within high precision
    fn test_approx_eq_with_high_precision_epsilon(
        #[case] a: f32,
        #[case] b: f32,
        #[case] expected: bool,
    ) {
        // Arrange - inputs provided by rstest

        // Act
        let result = approx_eq::<HighPrecisionEpsitlon>(a, b);

        // Assert
        assert_eq!(result, expected);
    }

    #[rstest]
    #[case(1.0, 1.0005, true)] // within low precision epsilon
    #[case(1.0, 1.002, false)] // outside low precision epsilon
    #[case(0.0, 0.0005, true)] // near zero within low precision
    fn test_approx_eq_with_low_precision_epsilon(
        #[case] a: f32,
        #[case] b: f32,
        #[case] expected: bool,
    ) {
        // Arrange - inputs provided by rstest

        // Act
        let result = approx_eq::<LowPrecisionEpsitlon>(a, b);

        // Assert
        assert_eq!(result, expected);
    }

    #[test]
    fn test_approx_eq_symmetry() {
        // Arrange
        let a = 1.0;
        let b = 1.000001;

        // Act
        let result_ab = approx_eq::<DefaultEpsilon>(a, b);
        let result_ba = approx_eq::<DefaultEpsilon>(b, a);

        // Assert
        assert_eq!(result_ab, result_ba);
    }

    #[rstest]
    #[case(0.0, true)]
    #[case(0.000001, true)] // within epsilon
    #[case(-0.000001, true)] // negative within epsilon
    #[case(0.00002, false)] // outside epsilon
    #[case(-0.00002, false)] // negative outside epsilon
    #[case(1.0, false)] // clearly not zero
    #[case(-1.0, false)] // negative clearly not zero
    fn test_approx_zero_with_default_epsilon(#[case] value: f32, #[case] expected: bool) {
        // Arrange - input provided by rstest

        // Act
        let result = approx_zero::<DefaultEpsilon>(value);

        // Assert
        assert_eq!(result, expected);
    }

    #[rstest]
    #[case(0.0, true)]
    #[case(0.0000000001, true)]
    #[case(0.0000001, false)]
    fn test_approx_zero_with_high_precision_epsilon(#[case] value: f32, #[case] expected: bool) {
        // Arrange - input provided by rstest

        // Act
        let result = approx_zero::<HighPrecisionEpsitlon>(value);

        // Assert
        assert_eq!(result, expected);
    }

    #[rstest]
    #[case(0.0, true)]
    #[case(0.0005, true)] // within low precision epsilon
    #[case(0.002, false)] // outside low precision epsilon
    #[case(-0.0005, true)] // negative within low precision epsilon
    fn test_approx_zero_with_low_precision_epsilon(#[case] value: f32, #[case] expected: bool) {
        // Arrange - input provided by rstest

        // Act
        let result = approx_zero::<LowPrecisionEpsitlon>(value);

        // Assert
        assert_eq!(result, expected);
    }

    #[test]
    fn test_approx_zero_boundary_at_exact_epsilon() {
        // Arrange
        let value = DefaultEpsilon::EPSILON;

        // Act
        let result = approx_zero::<DefaultEpsilon>(value);

        // Assert
        assert_eq!(result, false);
    }

    #[test]
    fn test_approx_eq_boundary_at_exact_epsilon() {
        // Arrange
        let a = 1.0;
        let b = 1.0 + DefaultEpsilon::EPSILON;

        // Act
        let result = approx_eq::<DefaultEpsilon>(a, b);

        // Assert
        assert_eq!(result, false);
    }
}
