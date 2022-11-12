use crate::Matrix;

/// https://numpy.org/doc/stable/reference/routines.linalg.html
impl Matrix {

    /// Matrix miltiplication between two matrices
    /// - https://www.wikiwand.com/en/Matrix%20multiplication
    pub fn matmul(x: &Self, y: &Self) -> Self {
        assert_eq!(x.shape.1, y.shape.0);

        Matrix::zeros((10, 10))
    }

    ///
    /// - https://www.wikiwand.com/en/Dot_product
    pub fn dot(x: &Self, y: &Self) -> Self {
        Matrix::zeros_like(x)
    }

    pub fn trace(m: &Self) -> f32 {
        0.
    }
}