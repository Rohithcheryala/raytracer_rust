mod matrix;

use matrix::Matrix2D;
fn main() {
    let mut m = Matrix2D::<i32>::new(4, 4);
    // m.inner = vec![vec![1, 2, 3], vec![3, 4, 5], vec![3, 2, 1]];
    m.inner = vec![
        vec![-5, 2, 6, -8],
        vec![1, -5, 1, 8],
        vec![7, 7, -6, -7],
        vec![1, -3, 7, 4],
    ];
    println!("{}", m.inverse());
}
