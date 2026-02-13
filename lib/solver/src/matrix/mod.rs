use color_eyre::eyre::Result;

use size::Size;

pub(crate) mod op;
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
    /// * `Option<&Element>` - Some(element) if the element exists at the specified position, None otherwise.
    fn get(&self, row: usize, col: usize) -> Result<Option<&Element>>;

    /// Get the row from matrix. result is copy of element, so does not reflect change for it.
    fn get_row(&self, row: usize) -> Result<Vec<Option<Element>>>;

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
    ) -> Result<Option<Element>>;

    /// Set the elements to the row.
    fn set_row(&mut self, row: usize, elements: &[Option<Element>]) -> Result<()>;

    /// Get the size of matrix.
    ///
    /// # Description
    /// Returns the size of the matrix as a `Size` struct which first element is number of rows and second element is number of columns.
    fn size(&self) -> Size;
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
