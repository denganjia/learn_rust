use std::fmt::{self, Display, Formatter};

#[allow(dead_code)]
fn reverse(pair: (i32, bool)) -> (bool, i32) {
    let (integer, boolean) = pair;
    (boolean, integer)
}
#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

impl Display for Matrix {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "\n({},{})\n({},{})", self.0, self.1, self.2, self.3)
    }
}
fn transpose (matrix:Matrix)->Matrix{
    Matrix(matrix.0, matrix.2, matrix.1, matrix.3)
}
pub fn tuples() {
    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("{}", matrix);
    println!("{}",transpose(matrix))
}
