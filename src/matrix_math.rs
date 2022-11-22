use crate::Matrix;

impl Matrix {
    //// Helper functions

    /// clip: fix the upper and lower bound of the values in the matrix
    pub fn clip(&self, lower: f32, upper: f32) -> Self {
        Matrix {
            mem: self
                .mem
                .iter()
                .map(|&e| {
                    if e < lower {
                        return lower;
                    } else if e > upper {
                        return upper;
                    } else {
                        return e;
                    }
                })
                .collect(),
            ..*self
        }
    }
    /// Returns the sign of the matrix
    /// TODO: use total_cm or other comparision method for float equality
    pub fn sign(&self) -> Self {
        Matrix {
            mem: self.mem.iter().map(|&e|
                // e.par
                if e == 0. {
                    return 0.;
                } else if e < 0. {
                    return -1.;
                } else if e > 0.{
                    return 1.;
                } else {
                    panic!("float comparison error")
                }
            ).collect(),
            ..*self
        }
    }
    /// absolute: calculate the absolute value element wise of the input matrix
    pub fn absolute(&self) -> Self {
        Matrix {
            mem: self.mem.iter().map(|e| e.abs()).collect(),
            ..*self
        }
    }
    /// shorthand for absolute
    pub fn abs(&self) -> Self {
        self.absolute()
    }

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
    /// square: same as power but with fixed exponent = 2
    pub fn square(&self) -> Self {
        self.power(2.)
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
    /// sin
    pub fn sin(&self) -> Self {
        Matrix {
            mem: self.mem.iter().map(|e| e.sin()).collect(),
            ..*self
        }
    }
    /// cos
    pub fn cos(&self) -> Self {
        Matrix {
            mem: self.mem.iter().map(|e| e.cos()).collect(),
            ..*self
        }
    }
    /// tan
    pub fn tan(&self) -> Self {
        Matrix {
            mem: self.mem.iter().map(|e| e.tan()).collect(),
            ..*self
        }
    }
    /// arcsin
    pub fn arcsin(&self) -> Self {
        Matrix {
            mem: self.mem.iter().map(|e| e.asin()).collect(),
            ..*self
        }
    }
    /// arccos
    pub fn arccos(&self) -> Self {
        Matrix {
            mem: self.mem.iter().map(|e| e.acos()).collect(),
            ..*self
        }
    }
    /// arctan
    pub fn arctan(&self) -> Self {
        Matrix {
            mem: self.mem.iter().map(|e| e.atan()).collect(),
            ..*self
        }
    }
    /// hypot
    pub fn hypot(&self, matrix_2: &Self) -> Self {
        Matrix {
            mem: self
                .mem
                .iter()
                .zip(matrix_2.mem.iter())
                .map(|(&a, &b)| a.hypot(b))
                .collect(),
            ..*self
        }
    }
    // arctan2

    /// degrees: Convert angles from radians to degrees.
    ///
    /// [&self:] matrix of radians
    pub fn degrees(&self) -> Self {
        Matrix {
            mem: self.mem.iter().map(|e| e.to_degrees()).collect(),
            ..*self
        }
    }
    /// radians: Convert angles from degress to radians.
    ///
    /// [&self:] matrix of degrees
    pub fn radians(&self) -> Self {
        Matrix {
            mem: self.mem.iter().map(|e| e.to_radians()).collect(),
            ..*self
        }
    }
    // unwrap
    // deg2rad
    // rad2deg

    //// Hyperbolic functions
    //// https://numpy.org/doc/stable/reference/routines.math.html#hyperbolic-functions

    /// sinh
    pub fn sinh(&self) -> Self {
        Matrix {
            mem: self.mem.iter().map(|e| e.sinh()).collect(),
            ..*self
        }
    }
    /// cosh
    pub fn cosh(&self) -> Self {
        Matrix {
            mem: self.mem.iter().map(|e| e.cosh()).collect(),
            ..*self
        }
    }
    /// tanh
    pub fn tanh(&self) -> Self {
        Matrix {
            mem: self.mem.iter().map(|e| e.tanh()).collect(),
            ..*self
        }
    }
    /// arcsinh
    pub fn arcsinh(&self) -> Self {
        Matrix {
            mem: self.mem.iter().map(|e| e.asinh()).collect(),
            ..*self
        }
    }
    /// arccosh
    pub fn arccosh(&self) -> Self {
        Matrix {
            mem: self.mem.iter().map(|e| e.acosh()).collect(),
            ..*self
        }
    }
    /// arctanh
    pub fn arctanh(&self) -> Self {
        Matrix {
            mem: self.mem.iter().map(|e| e.atanh()).collect(),
            ..*self
        }
    }
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
