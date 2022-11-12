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
}