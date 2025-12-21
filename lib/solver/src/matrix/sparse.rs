use crate::matrix::Matrix;

/// implement sparse matrix

/// Sparse matrix model. This implementation is based on simple CSR model.
#[derive(Debug, Clone)]
struct SparseMatrix<M> {
    /// Values of row-ordered non-zero value. Zero is mean None in this type.
    values: Vec<Option<M>>,
    /// Column index of values
    col_indices: Vec<usize>,
    /// Pointer of index is to start column at the row
    row_ptr: Vec<usize>,
}

impl<M: Clone> SparseMatrix<M> {
    /// Create a sparse matrix from other matrix
    pub fn from_matrix(mat: &impl Matrix<M>) -> Self {
        let size = mat.size();
        let mut values: Vec<Option<M>> = vec![];
        let mut col_indices: Vec<usize> = vec![];
        let mut row_ptr: Vec<usize> = vec![];

        for r in 0..(size.rows()) {
            let mut ptr_recorded = false;

            for c in 0..(size.columns()) {
                if let Ok(Some(v)) = mat.get(r, c) {
                    values.push(Some(v));
                    col_indices.push(c);

                    if !ptr_recorded {
                        ptr_recorded = true;
                        row_ptr.push(c)
                    }
                }
            }
        }

        row_ptr.push(values.iter().filter_map(Option::Some).count());

        SparseMatrix {
            values: values.iter().cloned().filter_map(|v| Some(v)).collect(),
            col_indices: vec![],
            row_ptr: vec![],
        }
    }
}
