#[test]
fn ui_tests() {
    // Arrange
    let t = trybuild::TestCases::new();

    // Act & Assert
    t.pass("tests/ui/pass/*.rs");
    t.compile_fail("tests/ui/fail/*.rs");
}
