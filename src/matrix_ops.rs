use std::ops::{Add, Div, Index, IndexMut, Mul, Neg, Sub};
use crate::Matrix;

impl Add for Matrix {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
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

impl Sub for Matrix {
    type Output = Self;

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

impl Mul for Matrix {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        assert_eq!(self.shape, rhs.shape, "The shapes do not match");
        Matrix {
            mem: self
                .mem
                .iter()
                .zip(rhs.mem.iter())
                .map(|(e, rhse)| e * rhse)
                .collect(),
            shape: self.shape,
            stride: self.stride,
        }
    }
}

impl Div for Matrix {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        assert_eq!(self.shape, rhs.shape, "The shapes do not match");
        Matrix {
            mem: self
                .mem
                .iter()
                .zip(rhs.mem.iter())
                .map(|(e, rhse)| e * rhse)
                .collect(),
            shape: self.shape,
            stride: self.stride,
        }
    }
}

impl Neg for Matrix {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Matrix {
            mem: self.mem.iter().map(|e| -e).collect(),
            shape: self.shape,
            stride: self.stride,
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