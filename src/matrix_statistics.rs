use crate::Matrix;

impl Matrix {
    pub fn mean_total(&self) -> f32 {
        self.sum_total() / (self.len() as f32)
    }
    pub fn median_total(&self) -> f32 {
        let mut ordered_copy = self.clone();
        ordered_copy.mem.sort_by(|a, b| a.total_cmp(b));

        let mid_ind = ordered_copy.len() / 2;
        match ordered_copy.len() % 2 {
            0 => (ordered_copy.mem[mid_ind - 1] + ordered_copy.mem[mid_ind]) / 2.,
            1 => ordered_copy.mem[mid_ind],
            _ => panic!("Impossibly strange error occured: a%2 != {{0, 1}}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_median_odd() {
        let new = Matrix::from(&[1., 2., 5., 9., 735.]);
        assert_eq!(new.median_total(), 5.);
    }
    #[test]
    fn test_median_even() {
        let new = Matrix::from(&[1., 2., 5., 9., 735., 12.]);
        assert_eq!(new.median_total(), 7.);
    }
}
