use crate::Matrix;

impl Matrix {
    //// Operators
    /// exp: natural exponential, inverso of nat log
    pub fn exp(&self) -> Self {
        Matrix {
            mem: self.mem.iter().map(|e| e.exp()).collect(),
            ..*self
        }
    }
    /// power
    pub fn power(&self, exponent: f32) -> Self {
        Matrix {
            mem: self.mem.iter().map(|e| e.powf(exponent)).collect(),
            ..*self
        }
    }
    /// sqrt: of the matrix
    pub fn sqrt(&self) -> Self {
        Matrix {
            mem: self.mem.iter().map(|e| e.sqrt()).collect(),
            ..*self
        }
    }
    /// ln: natural logarithms of the matrix
    pub fn ln(&self) -> Self {
        Matrix {
            mem: self.mem.iter().map(|e| e.ln()).collect(),
            ..*self
        }
    }
    /// log2
    pub fn log2(&self) -> Self {
        Matrix {
            mem: self.mem.iter().map(|e| e.log2()).collect(),
            ..*self
        }
    }
    /// log10
    pub fn log10(&self) -> Self {
        Matrix {
            mem: self.mem.iter().map(|e| e.log10()).collect(),
            ..*self
        }
    }
    /// logn: log of base: base
    pub fn logn(&self, base: f32) -> Self {
        Matrix {
            mem: self.mem.iter().map(|e| e.log(base)).collect(),
            ..*self
        }
    }

    //// Sum, prods and differences
    //// https://numpy.org/doc/stable/reference/routines.math.html#sums-products-differences

    /// prod: product over the given axis
    pub fn prod(&self, axis: usize) -> Matrix {
        assert_eq!(axis < 2, true);
        match axis {
            0 => {
                let mut new = Matrix::ones((1, self.width()));

                for i in 0..self.height() {
                    for j in 0..self.width() {
                        new[(0, j)] *= self[(i, j)];
                    }
                }
                new
            }
            1 => {
                let mut new = Matrix::ones((self.height(), 1));

                for i in 0..self.height() {
                    for j in 0..self.width() {
                        new[(i, 0)] *= self[(i, j)];
                    }
                }
                new
            }
            _ => {
                panic!("Axis value is not correct")
            }
        }
    }

    /// sum: sum over the given axis
    pub fn sum(&self, axis: usize) -> Matrix {
        assert_eq!(axis < 2, true);
        match axis {
            0 => {
                let mut new = Matrix::zeros((1, self.width()));

                for i in 0..self.height() {
                    for j in 0..self.width() {
                        new[(0, j)] += self[(i, j)];
                    }
                }
                new
            }
            1 => {
                let mut new = Matrix::zeros((self.height(), 1));

                for i in 0..self.height() {
                    for j in 0..self.width() {
                        new[(i, 0)] += self[(i, j)];
                    }
                }
                new
            }
            _ => {
                panic!("Axis value is not correct")
            }
        }
    }
    /// prod_total: total product of all matrix elements
    pub fn prod_total(&self) -> f32 {
        self.mem.iter().fold(1., |b, e| b * e)
    }
    /// sum_total: total sum of all matrix elements
    pub fn sum_total(&self) -> f32 {
        self.mem.iter().sum()
    }

    /// cumprod_total: total matrix cumulative product
    pub fn cumprod_total(&self) -> Self {
        let mut new = Vec::with_capacity(self.len());

        self.mem.iter().fold(1., |acc, x| {
            let temp = acc * *x;
            new.push(temp);
            temp
        });

        Matrix { mem: new, ..*self }
    }

    /// cumsum_total: total matrix cumulative sum
    pub fn cumsum_total(&self) -> Self {
        let mut new = Vec::with_capacity(self.len());

        self.mem.iter().fold(0., |acc, x| {
            let temp = acc + *x;
            new.push(temp);
            temp
        });

        Matrix { mem: new, ..*self }
    }
    // gradient
    // cross

    //// Trigonometric functions
    //// - https://numpy.org/doc/stable/reference/routines.math.html#trigonometric-functions
    // sin
    // cos
    // tan
    // arcsin
    // arccos
    // arctan
    // hypot
    // arctan2
    // degrees
    // radians
    // unwrap
    // deg2rad
    // rad2deg

    //// Hyperbolic functions
    //// https://numpy.org/doc/stable/reference/routines.math.html#hyperbolic-functions
    // sinh
    // cosh
    // tanh
    // arcsinh
    // arccosh
    // arctanh
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prod_axis_0() {
        let mat = Matrix::range((3, 2));

        let new = mat.prod(0);
        assert_eq!(new.shape, (1, 2));

        assert_eq!(new[(0, 0)], 0.);
        assert_eq!(new[(0, 1)], 15.);
    }

    #[test]
    fn test_prod_axis_1() {
        let mat = Matrix::range((3, 2));

        let new = mat.prod(1);
        assert_eq!(new.shape, (3, 1));

        assert_eq!(new[(0, 0)], 0.);
        assert_eq!(new[(1, 0)], 6.);
        assert_eq!(new[(2, 0)], 20.);
    }

    #[test]
    fn test_sum_axis_0() {
        let mat = Matrix::range((3, 2));

        let new = mat.sum(0);
        assert_eq!(new.shape, (1, 2));

        assert_eq!(new[(0, 0)], 6.);
        assert_eq!(new[(0, 1)], 9.);
    }

    #[test]
    fn test_sum_axis_1() {
        let mat = Matrix::range((3, 2));

        let new = mat.sum(1);
        assert_eq!(new.shape, (3, 1));

        assert_eq!(new[(0, 0)], 1.);
        assert_eq!(new[(1, 0)], 5.);
        assert_eq!(new[(2, 0)], 9.);
    }
}
