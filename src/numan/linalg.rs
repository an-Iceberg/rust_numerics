use crate::numan::{Matrix, Vector};

#[allow(non_snake_case)]
pub fn mat_to_string(M: &Matrix) -> String
{
  let mut string = "".to_string();

  M.iter()
    .for_each(|line|
    {
      let mut line = vec_to_string(line);
      line.push('\n');
      string.push_str(line.as_str());
    });

  string.chars()
    .next_back()
    .unwrap()
    .to_string()
}

pub fn vec_to_string(a: &Vector) -> String
{
  format!("{:?}", a)
}

pub fn zero(n: usize) -> Matrix
{
  vec![vec![0.; n]; n]
}

pub fn id(n: usize) -> Matrix
{
  #[allow(non_snake_case)]
  let mut M = zero(n);

  for i in 0..n { M[i][i] = 1.; }
  // for (i, _) in M.iter_mut().enumerate().take(n) { M[i][i] = 1.; }

  M
}

#[allow(non_snake_case)]
pub fn matmul(A: &Matrix, B: &Matrix) -> Result<Matrix, String>
{
  todo!()
}

#[allow(non_snake_case)]
pub fn lr(M: &Matrix) -> Result<(Matrix, Matrix), String>
{
  todo!()
}

fn householder(v: Vector) -> Matrix
{
  todo!()
}

#[allow(non_snake_case)]
pub fn qr(M: &Matrix) -> Result<(Matrix, Matrix), String>
{
  todo!()
}

#[allow(non_snake_case)]
pub fn gauss_seidel(A: &Matrix, b: &Vector) -> Result<Vector, String>
{
  todo!()
}

#[allow(non_snake_case)]
pub fn print_mat(A: &Matrix)
{
  A.iter()
    .for_each(|vec| println!("{:?}", vec));
}

