pub mod matrix;
pub mod matrix_access;
pub mod matrix_creation;
pub mod matrix_linalgebra;
pub mod matrix_manipulation;
pub mod matrix_math;
pub mod matrix_ops;
pub mod matrix_statistics;
pub mod utils;

#[derive(Clone)]
pub struct Matrix {
    pub(crate) mem: Vec<f32>,
    pub(crate) shape: (usize, usize),
    pub(crate) stride: (usize, usize),
}
