type Matrix = ((i32, i32), (i32, i32));

fn transpose(mut matrix: Matrix) -> Matrix {
    let tmp = matrix.1 .0;
    matrix.1 .0 = matrix.0 .1;
    matrix.0 .1 = tmp;

    matrix
}

fn main() {
    let matrix1: Matrix = ((1, 2), (3, 4));

    let matrix2: Matrix = transpose(matrix1);
    println!("{:?}", matrix2);
}
