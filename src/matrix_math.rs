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
    /// logn
    pub fn logn(&self, base: f32) -> Self {
        Matrix {
            mem: self.mem.iter().map(|e| e.log(base)).collect(),
            ..*self
        }
    }

    //// Sum, prods and differences
    //// https://numpy.org/doc/stable/reference/routines.math.html#sums-products-differences
    // prod
    // sum
    // cumprod
    // cumsum
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