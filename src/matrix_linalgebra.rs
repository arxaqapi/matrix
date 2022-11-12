use crate::Matrix;

/// https://numpy.org/doc/stable/reference/routines.linalg.html
impl Matrix {

    /// Matrix miltiplication between two matrices
    /// - https://www.wikiwand.com/en/Matrix%20multiplication
    pub fn matmul(x: &Self, y: &Self) -> Self {
        assert_eq!(x.shape.1, y.shape.0);

        let m = x.shape.0;
        let n = x.shape.1;
        let p = y.shape.1;
        
        let mut result = Matrix::zeros((m, p));
        for i in 0..m {
            for j in 0..p {
                for k in 0..n {
                    result[(i, j)] += x[(i, k)] * y[(k, j)];
                }
            }
        }
        result
    }

    ///
    /// - https://www.wikiwand.com/en/Dot_product
    pub fn dot(x: &Self, _y: &Self) -> Self {
        Matrix::zeros_like(x)
    }

    pub fn trace(m: &Self) -> f32 {
        assert_eq!(m.height(), m.width());
        0.
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matmul() {
        let mut a = Matrix::ones((4, 3));
        let mut b = Matrix::fill((3, 3), 2.);
        a[(0, 1)] = 0.;
        a[(1, 0)] = 2.;
        a[(2, 0)] = 0.;
        a[(3, 2)] = 2.;

        b[(0, 0)] = 1.;
        b[(0, 2)] = 1.;
        b[(1, 1)] = 3.;
        b[(1, 2)] = 1.;
        b[(2, 0)] = 4.;
        b[(2, 1)] = 2.;
        b[(2, 2)] = 2.;
        
        assert_eq!(a.shape.1, b.shape.0);
        let res = Matrix::matmul(&a, &b);

        // check values
        assert_eq!(res[(0, 0)], 5.);
        assert_eq!(res[(0, 1)], 4.);
        assert_eq!(res[(0, 2)], 3.);

        assert_eq!(res[(1, 0)], 8.);
        assert_eq!(res[(1, 1)], 9.);
        assert_eq!(res[(1, 2)], 5.);

        assert_eq!(res[(2, 0)], 6.);
        assert_eq!(res[(2, 1)], 5.);
        assert_eq!(res[(2, 2)], 3.);

        assert_eq!(res[(3, 0)], 11.);
        assert_eq!(res[(3, 1)], 9.);
        assert_eq!(res[(3, 2)], 6.);
    }
    
    #[test]
    fn test_matmul_medium() {
        let m1 = Matrix::ones((2, 7));
        let m2 = Matrix::fill((7, 23), 3.);

        Matrix::matmul(&m1, &m2);
        assert_eq!(m1.shape.1, m2.shape.0);
    }
}