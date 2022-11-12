use crate::Matrix;

impl Matrix {
    pub fn col(_col_number: usize) { //  -> Vec<&f32> 

    }

    pub fn row(_row_number: usize) { //  -> Vec<&f32> 
        
    }

    pub fn height(&self) -> usize {
        self.shape.0
    }
    pub fn width(&self) -> usize {
        self.shape.1
    }
}