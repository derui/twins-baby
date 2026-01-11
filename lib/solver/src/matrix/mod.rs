use anyhow::Result;

use rect::Rect;
use size::Size;

pub(crate) mod op;
pub mod rect;
pub mod simple;
pub mod size;
pub mod sparse;

/// A matrix trait to define standard behavior of matrix.
pub trait Matrix<Element>: std::fmt::Debug
where
    Element: std::fmt::Debug,
{
    /// Get diagonal components.
    ///
    /// # Returns
    /// * Return compoments that is number of `row`. If the matrix is not square, return None
    fn diagonal_components(&self) -> Option<Vec<Option<Element>>>;

    /// Get the element of matrix at specified row and column.
    ///
    /// # Arguments
    /// * `row` - The row index of the element to retrieve.
    /// * `col` - The column index of the element to retrieve.
    ///
    /// # Returns
    /// * `Option<Element>` - Some(element) if the element exists at the specified position, None otherwise.
    fn get(&self, row: usize, col: usize) -> Result<Option<Element>, anyhow::Error>;

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
    ) -> Result<Option<Element>, anyhow::Error>;

    /// Get the size of matrix.
    ///
    /// # Description
    /// Returns the size of the matrix as a `Size` struct which first element is number of rows and second element is number of columns.
    fn size(&self) -> Size;

    /// Get sub-matrix from `self`.
    ///
    /// # Arguments
    /// * `rect` : sub-matrix rectangle
    ///
    /// # Returns
    /// * A new sub-matrix of the matrix. The matrix is sharing reference as original, so same lifetime of self.
    fn sub_matrix(&self, rect: Rect) -> Result<&dyn Matrix<Element>>;
}

pub trait MatrixExtract<Element> {
    /// Extract a matrix of f32 from this matrix
    ///
    /// This function to use specialized math function for f32, such as determinant calculation.
    ///
    /// # Arguments
    /// * `extract` - a function to extract `f32` from the element.
    ///
    /// # Returns
    /// * Return a new matrix of f32
    fn extract<T>(&self, extract: T) -> impl Matrix<f32>
    where
        T: Fn(&Element) -> f32;
}
