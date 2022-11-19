use std::ops::{Add, Div, Index, IndexMut, Mul, Neg, Sub};
use crate::Matrix;

impl Add for &Matrix {
    type Output = Matrix;

    fn add(self, rhs: &Matrix) -> Self::Output {
        assert_eq!(self.shape, rhs.shape, "The shapes do not match");
        Matrix {
            mem: self
                .mem
                .iter()
                .zip(rhs.mem.iter())
                .map(|(e, rhse)| e + rhse)
                .collect(),
            shape: self.shape,
            stride: self.stride,
        }
    }
}

impl Add<f32> for &Matrix {
    type Output = Matrix;

    fn add(self, rhs: f32) -> Self::Output {
        Matrix {
            mem: self.mem.iter().map(| e | e + rhs).collect(),
            ..*self
        }
    }
}

impl Sub for &Matrix {
    type Output = Matrix;

    fn sub(self, rhs: Self) -> Self::Output {
        assert_eq!(self.shape, rhs.shape, "The shapes do not match");
        Matrix {
            mem: self
                .mem
                .iter()
                .zip(rhs.mem.iter())
                .map(|(e, rhse)| e - rhse)
                .collect(),
            shape: self.shape,
            stride: self.stride,
        }
    }
}

impl Mul for &Matrix {
    type Output = Matrix;

    fn mul(self, rhs: Self) -> Self::Output {
        assert_eq!(self.shape, rhs.shape, "The shapes do not match");
        Matrix {
            mem: self
                .mem
                .iter()
                .zip(rhs.mem.iter())
                .map(|(e, rhse)| e * rhse)
                .collect(),
            ..*self
        }
    }
}

impl Mul<f32> for &Matrix {
    type Output = Matrix;

    fn mul(self, rhs: f32) -> Self::Output {
        Matrix {
            mem: self
                .mem
                .iter()
                .map(| e | e * rhs )
                .collect(),
            ..*self
        }
    }
}

impl Div for &Matrix {
    type Output = Matrix;

    fn div(self, rhs: Self) -> Self::Output {
        assert_eq!(self.shape, rhs.shape, "The shapes do not match");
        Matrix {
            mem: self
                .mem
                .iter()
                .zip(rhs.mem.iter())
                .map(|(e, rhse)| e * rhse)
                .collect(),
            ..*self
        }
    }
}

impl Div<f32> for &Matrix {
    type Output = Matrix;

    fn div(self, rhs: f32) -> Self::Output {
        Matrix {
            mem: self
                .mem
                .iter()
                .map(| e | e / rhs )
                .collect(),
                ..*self
        }
    }
}

impl Neg for &Matrix {
    type Output = Matrix;

    fn neg(self) -> Self::Output {
        Matrix {
            mem: self.mem.iter().map(|e| -e).collect(),
            ..*self
        }
    }
}

// Access
// TODO: Compute access using strides and tuple as param
impl Index<(usize, usize)> for Matrix {
    type Output = f32;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let (_rows, cols) = self.shape;
        &self.mem[index.0 * cols + index.1]
    }
}

impl IndexMut<(usize, usize)> for Matrix {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        let (_rows, cols) = self.shape;
        &mut self.mem[index.0 * cols + index.1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let mat1 = Matrix::ones((10, 23)); 
        let mat2 = Matrix::ones((10, 23)); 
        let res = &mat1 + &mat2;
        for i in 0..res.height() {
            for j in 0..res.width() {
                assert_eq!(res[(i, j)], 2.);
            }
        }
        // mat1 has only been borrowed and is therefor still accessible after add
        let _s = mat1.get_shape();
        let _s2 = mat1.get_shape();

    }
    #[test]
    fn test_sub() {
        let mat1 = Matrix::ones((10, 23)); 
        let mat2 = Matrix::ones((10, 23)); 
        let res = &mat1 - &mat2;
        for i in 0..res.height() {
            for j in 0..res.width() {
                assert_eq!(res[(i, j)], 0.);
            }
        }
        // mat1 has only been borrowed and is therefor still accessible after add
        let _s = mat1.get_shape();
        let _s2 = mat1.get_shape();
    }
    #[test]
    fn test_mul() {
        let mat1 = &Matrix::ones((10, 23)) + &Matrix::ones((10, 23)); 
        let mat2 = mat1.clone(); 

        let res = &mat1 * &mat2;
        for i in 0..res.height() {
            for j in 0..res.width() {
                assert_eq!(res[(i, j)], 4.);
            }
        }
        // mat1 has only been borrowed and is therefor still accessible after add
        let _s = mat1.get_shape();
        let _s2 = mat1.get_shape();
    }

}