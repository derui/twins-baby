use std::error::Error;

use size::Size;

pub mod simple;
pub mod size;
pub mod sparse;

/// A matrix trait to define standard behavior of matrix.
pub trait Matrix<Element> {
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
    /// * `Option<Element>` - Some(element) if the element exists at the specified position, None otherwise.
    fn get(&self, row: usize, col: usize) -> Result<Option<Element>, Box<dyn Error>>;

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
        &mut self,
        row: usize,
        col: usize,
        element: Element,
    ) -> Result<Option<Element>, Box<dyn Error>>;

    /// Extract a matrix of f32 from this matrix
    ///
    /// This function to use specialized math function for f32, such as determinant calculation.
    ///
    /// # Arguments
    /// * `extract` - a function to extract `f32` from the element.
    ///
    /// # Returns
    /// * Return a new matrix of f32
    fn extract<T>(&self, extract: T) -> impl Matrix<f32> + FloatingMatrix
    where
        T: Fn(&Element) -> f32;

    /// Get diagonal components.
    ///
    /// # Returns
    /// * Return compoments that is number of `row`. If the matrix is not square, return None
    fn diagonal_components(&self) -> Option<Vec<Option<Element>>>;
}

pub trait FloatingMatrix {
    /// Calculate a determinant of this matrix
    ///
    /// # Returns
    /// * Return the determinant if the matrix can define determinant, or None if the matrix can not calculate it.
    fn determinant(&self) -> Option<f32>;
}
