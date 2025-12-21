use std::error::Error;

use size::Size;

pub mod size;

pub trait Matrix {
    /// Type of element
    type Element;

    /// Get the size of matrix.
    ///
    /// # Description
    /// Returns the size of the matrix as a `Size` struct which first element is number of rows and second element is number of columns.
    fn size(&self) -> Size;

    /// Get the element of matrix at specified row and column.
    ///
    /// # Arguments
    /// * `row` - The row index of the element to retrieve.
    /// * `col` - The column index of the element to retrieve.
    ///
    /// # Returns
    /// * `Option<Self::Element>` - Some(element) if the element exists at the specified position, None otherwise.
    fn get(row: usize, col: usize) -> Option<Self::Element>;

    /// Set the element to the position with value.
    ///
    /// # Arguments
    /// * `row` - row of element to set
    /// * `col` - column of element to set
    /// * `element` - new element
    ///
    /// # Returns
    /// * When succeed setting the element, return old element if exists. Return error string when it failed.
    fn set(
        row: usize,
        col: usize,
        element: Self::Element,
    ) -> Result<Option<Self::Element>, Box<dyn Error>>;

    /// Calculate a determinant of this matrix
    ///
    /// # Arguments
    /// * `extract` - a function to extract `f32` from the element. If
    ///
    /// # Returns
    /// * Return the determinant if the matrix can define determinant, or None if the matrix can not calculate it.
    fn determinant<T>(&self, extract: T) -> Option<f32>
    where
        T: Fn(&Self::Element) -> f32;
}
