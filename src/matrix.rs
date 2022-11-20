use crate::utils::{argmax, argmin};
use crate::Matrix;

impl Matrix {
    /// total length of the matrix (n° of columns * n° rows)
    pub fn len(&self) -> usize {
        self.shape.0 * self.shape.1
    }
    pub fn get_shape(&self) -> (usize, usize) {
        self.shape
    }

    pub fn argmax(&self) -> (usize, f32) {
        argmax(&self.mem)
    }
    pub fn argmin(&self) -> (usize, f32) {
        argmin(&self.mem)
    }
    pub fn max(&self) -> f32 {
        argmax(&self.mem).1
    }
    pub fn maximum(&self) -> f32 {
        argmax(&self.mem).1
    }
    pub fn min(&self) -> f32 {
        argmin(&self.mem).1
    }
    pub fn minimum(&self) -> f32 {
        argmin(&self.mem).1
    }
}

/// Test suite
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max() {
        let mut mat = Matrix::fill((2, 3), 2.);
        let value = 87.;
        mat[(1, 2)] = value;

        assert_eq!(mat.max(), value);
        assert_eq!(mat.maximum(), value);
    }

    #[test]
    fn test_min() {
        let mut mat = Matrix::fill((2, 3), 182.);
        let value = 87.;
        mat[(1, 2)] = value;

        assert_eq!(mat.min(), value);
        assert_eq!(mat.minimum(), value);
    }
}
