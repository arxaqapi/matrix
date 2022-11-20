use crate::Matrix;

impl Matrix {
    pub fn fill_mat(&mut self, value: f32) {
        for e in self.mem.iter_mut() {
            *e = value;
        }
    }
    pub fn reset(&mut self) {
        for e in self.mem.iter_mut() {
            *e = 0.;
        }
    }
    pub fn transpose(&self) -> Matrix {
        let mut new = Matrix::zeros((self.width(), self.height()));
        assert_eq!(self.shape.0, new.shape.1);
        assert_eq!(self.shape.1, new.shape.0);
        for i in 0..self.height() {
            for j in 0..self.width() {
                new[(j, i)] = self[(i, j)];
            }
        }

        new
    }
    pub fn t(&self) -> Matrix {
        self.transpose()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transpose() {
        let mat = Matrix::range((2, 3));

        let transposed = mat.transpose();
        assert_eq!(mat.height(), transposed.width());
        assert_eq!(mat.width(), transposed.height());
        assert_eq!(mat[(0, 2)], transposed[(2, 0)]);
        assert_eq!(mat[(1, 2)], transposed[(2, 1)]);
    }
}
