use crate::Matrix;

impl Matrix {
    pub fn zeros(shape: (usize, usize)) -> Self { // <const R: usize, const C: usize>
        Matrix {
            mem: vec![f32::default(); shape.0 * shape.1],
            shape: shape,
            stride: (1, 1),
        }
    }
    pub fn ones(shape: (usize, usize)) -> Self {
        Matrix {
            mem: vec![1.; shape.0 * shape.1],
            shape: shape,
            stride: (1, 1),
        }
    }
    // fill - 
    pub fn fill(shape: (usize, usize), e: f32) -> Self {
        Matrix {
            mem: vec![e; shape.0 * shape.1],
            shape: shape,
            stride: (1, 1),
        }
    }
    pub fn range(shape: (usize, usize)) -> Self {
        let memsize = shape.0 * shape.1;
        let mut range_mem = Vec::<f32>::with_capacity(shape.0 * shape.1);
        for i in 0..(memsize) {
            range_mem.push(i as f32);
        }
        Matrix {
            mem: range_mem,
            shape: shape,
            stride: (1, 1),
        }
    }
    pub fn from(values: &[f32]) -> Matrix {
        Matrix {
            mem: values.to_vec(),
            shape: (values.len(), 1),
            stride: (0, 0)
        }
    }

    //// Like:

    /// Create a matrix of the same shape as the matrix passed as parameter
    pub fn zeros_like(m: &Self) -> Self {
        Matrix {
            mem: vec![f32::default(); m.shape.0 * m.shape.1], // Vec::with_capacity(R * C),
            ..*m
        }
    }
    /// Create a matrix of the same shape as the matrix passed as parameter
    pub fn ones_like(m: &Self) -> Self {
        Matrix {
            mem: vec![1.; m.shape.0 * m.shape.1], // Vec::with_capacity(R * C),
            ..*m
        }
    }
    /// Create a matrix of the same shape as the matrix passed as parameter
    pub fn fill_like(m: &Self, e: f32) -> Self {
        Matrix {
            mem: vec![e; m.shape.0 * m.shape.1], // Vec::with_capacity(R * C),
            ..*m
        }
    }
    pub fn range_like(like: &Self) -> Self {
        Matrix {
            mem: Vec::<f32>::with_capacity(like.shape.0 * like.shape.1)
                .iter()
                .enumerate()
                .map(| (i, _) | i as f32 )
                .collect(),
            ..*like
        }
    }
    pub fn eye() {

    }
    pub fn identity() {
        Self::eye()
    }

    pub fn random_normal() {}
    pub fn random_uniform() {}
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zeros() {
        let zeroes = Matrix::zeros((5, 10));

        for i in 0..5 {
            for j in 0..10 {
                assert_eq!(zeroes[(i, j)], 0.);
            }
        }
        assert_eq!(zeroes.mem.iter().sum::<f32>(), 0.);
        assert_eq!(zeroes.shape, (5, 10));
    }

    #[test]
    fn test_ones() {
        let ones = Matrix::ones((1, 10));

        
        for i in 0..1 {
            for j in 0..10 {
                assert_eq!(ones[(i, j)], 1.);
            }
        }
        assert_eq!(ones.mem.iter().sum::<f32>(), 10.);
        assert_eq!(ones.shape, (1, 10));
    }

    #[test]
    fn test_range() {
        let range_mat = Matrix::range((1, 10));
        
        let mut b = 0.;
        for i in 0..1 {
            for j in 0..10 {
                assert_eq!(range_mat[(i, j)], b);
                b += 1.;
            }
        }
        assert_eq!(range_mat.mem.iter().sum::<f32>(), 45.);
        assert_eq!(range_mat.shape, (1, 10));
    }

    #[test]
    fn test_range_like() {
        let ones = Matrix::ones((1, 10));

        for i in 0..1 {
            for j in 0..10 {
                assert_eq!(ones[(i, j)], 1.);
            }
        }
        assert_eq!(ones.mem.iter().sum::<f32>(), 10.);
        assert_eq!(ones.shape, (1, 10));
    }

    #[test]
    fn test_fill() {
        let mat = Matrix::fill((2, 3), 2.);
        
        for i in 0..2 {
            for j in 0..3 {
                assert_eq!(mat[(i, j)], 2.);
            }
        }
        assert_eq!(mat.mem.iter().sum::<f32>(), 12.);
        assert_eq!(mat.shape, (2, 3));
    }
    
    #[test]
    fn test_from() {
        let mat = Matrix::from(&[1., 2., 5.]);

        assert_eq!(mat.shape, (3, 1));
        assert_eq!(mat.mem.len(), 3);
        assert_eq!(mat[(0, 0)], 1.);
        assert_eq!(mat[(1, 0)], 2.);
        assert_eq!(mat[(2, 0)], 5.);

    }
}