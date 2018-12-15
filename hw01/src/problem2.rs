/// Represents a matrix in row-major order
pub type Matrix = Vec<Vec<f32>>;

fn print_matrix(mat: &Matrix) {
  let line = mat.len();
  assert!(line > 0);
  let row = mat[0].len();

  for i in 0..line {
    for j in 0..row {
      print!("{} ", mat[i][j]);
    }
    println!();
  }
}

/// Computes the product of the inputs `mat1` and `mat2`.
pub fn mat_mult(mat1: &Matrix, mat2: &Matrix) -> Matrix {
  println!("mat1:===========================Start");
  print_matrix(mat1);
  println!("mat1:===========================End");
  println!("mat2:===========================Start");
  print_matrix(mat2);
  println!("mat2:===========================End");
  let mut result: Matrix = Vec::new();
  let line_len: usize = mat1.len();
  assert!(line_len > 0);
  let row_len: usize = mat1[0].len();

  assert!(mat2.len() > 0);
  assert!(row_len == mat2.len());
  let row_len = mat2[0].len();

  for line_index in 0..line_len {
    result.push(Vec::new());
    for count in 0..row_len {
      let mut num: f32 = 0.0;
      for row_index in 0..row_len {
        num = num + mat1[count][row_index] * mat2[row_index][line_index];
      }
      result[line_index].push(num);
    }
  }
  println!("Result:===============================");
  print_matrix(&result);
  return result;
}