// TODO: docs
// TODO: tests
// TODO: generics, so that any numeric type can be used

use std::ops::{Div, Mul};

// ToDo: create one fn `deriv` which then calls different functions based on parameters
// such as dergee and which method (standard, 2 point, 5 point)

pub fn deriv(f: fn(f64) -> f64, x: f64, degree: u8) -> f64
{
  // ToDo: determine h based on f64::EPSILON
  // I think it is more efficient to have a sort of look-up table for the derivatives
  // instead of using newton's formula https://en.wikipedia.org/wiki/Numerical_differentiation#Higher_derivatives
  let h = 10e-4;
  match degree
  {
    0 => f(x),
    1 => (f(x+h) - f(x-h)) / (2.*h),
    2 => (f(x+h) - 2.*f(x) + f(x-h)) / (h.powi(2)),
    _ => todo!(),
  }
  // Note: 5 point method: (f(x - 2.*h) - 8.*f(x - h) + 8.*f(x + h) - f(x + 2.*h)) / (12. * h)
}

pub fn partial(f: fn(&Vec<f64>) -> f64, x: &Vec<f64>, i: usize, degree: u8) -> f64
{
  if i >= x.len()
  {
    panic!("{}", format!("Index i = {} out of range for x of length {}", i, x.len()));
  }

  let h = 10e-4;
  match degree
  {
    0 => f(x),
    1 =>
    {
      let mut x_plus_h = x.clone();
      let mut x_minus_h = x.clone();
      x_plus_h[i] += h;
      x_minus_h[i] -= h;
      (f(&x_plus_h) - f(&x_minus_h)) / (2. * h)
    },
    2 =>
    {
      let mut x_plus_h = x.clone();
      let mut x_minus_h = x.clone();
      x_plus_h[i] += h;
      x_minus_h[i] -= h;
      (f(&x_plus_h) - (2.*f(x)) + f(&x_minus_h)) / h.powi(2)
    }
    _ => todo!()
  }
}

fn nCr(n: u64, r: u64) -> f64
{
  // Note: https://www.geeksforgeeks.org/program-calculate-value-ncr/
  todo!()
}

fn fact(n: u64) -> u64
{
  // Note: https://users.rust-lang.org/t/whats-the-idiomatic-or-preferred-way-of-calculating-factorials/77684/3
  todo!()
}

/// Calculates the area under the curve (integral) of the function `f` using
/// simpson's rule. `h` is the precision.
pub fn int(f: fn(f64) -> f64, a: f64, b: f64, h: f64) -> f64
{
  let n = ((b - a) / h).ceil() as u64;
  #[allow(non_snake_case)]
  let mut Σ1 = 0.;
  #[allow(non_snake_case)]
  let mut Σ2 = 0.;

  let x = |i| a + (i as f64 * h);

  for i in 1..n
  { Σ1 += f(x(i)); }

  for (i, j) in (0..n).zip((0..n).skip(1))
  { Σ2 += f((x(i) + x(j)) / 2.); }

  (h/3.)*((f(a)/2.) + Σ1 + Σ2.mul(2.) + (f(b)/2.))
}

pub fn int_rect(f: fn(f64) -> f64, a: f64, b: f64, h: f64) -> f64
{
  todo!()
}

// Note: https://en.wikipedia.org/wiki/Root-finding_algorithms
// Note: https://en.wikipedia.org/wiki/Householder%27s_method
// Note: https://en.wikipedia.org/wiki/Gauss%E2%80%93Seidel_method

// Todo: tolerance should take ε into account
/// [Formula](https://en.wikipedia.org/wiki/Nested_intervals)
pub fn bisection(f: fn(f64) -> f64, a: f64, b: f64, tolerance: f64) -> Result<f64, String>
{
  let mut a = a;
  let mut b = b;

  // If both have the same sign we can't know which half to keep searching in
  if (f(a) > 0. && f(b) > 0.) || (f(a) < 0. && f(b) < 0.)
  { return Err("Both bounds have the same sign. Can't determine, which half to search.".to_string()); }

  // a shall be < 0 and b shall be > 0
  if f(a) > 0. && f(b) < 0.
  {
    let temp = a;
    a = b;
    b = temp;
  }

  loop
  {
    let middle = (a + b) / 2.;

    if f(middle).abs() < tolerance
    { return Ok(middle); }

    if f(middle) > 0.
    { b = middle; }
    else if f(middle) < 0.
    { a = middle; }
    else
    { return Ok(middle); }
  }
}

/// [Formula](https://en.wikipedia.org/wiki/Secant_method)
pub fn secant_zero(f: fn(f64) -> f64, x0: f64, x1: f64, tolerance: f64) -> Option<f64>
{
  // ToDo: fail states
  let mut x1 = x0;
  let mut x2 = x0 + tolerance.mul(2.);
  let mut x3 = x0 + tolerance.mul(4.);

  while (x2 - x3).abs() > tolerance
  {
    x1 = x2;
    x2 = x3;
    x3 = x2 - ((x2 - x1) / (f(x2) - f(x1))).mul(f(x2));
  }

  return Some(x3);
}

/// [Formula](https://en.wikipedia.org/wiki/Newton%27s_method#)
pub fn newton_zero(f: fn(f64) -> f64, x0: f64, tolerance: f64) -> Option<f64>
{
  // ToDo: fail states
  let mut x1 = x0;
  let mut x2 = x1 + 2. * tolerance;

  while (x1 - x2).abs() > tolerance
  {
    x1 = x2;
    x2 = x1 - f(x1).div(deriv(f, x1, 1));
  }

  return Some(x2);
}

pub mod linalg
{
  pub type Matrix = Vec<Vec<f64>>;
  pub type Vector = Vec<f64>;

  #[allow(non_snake_case)]
  pub fn mat_to_string(M: &Matrix) -> String
  {
    todo!()
  }

  pub fn vec_to_string(a: Vector) -> String
  {
    todo!()
  }

  pub fn zero(n: usize) -> Matrix
  {
    return vec![vec![0.; n]; n];
  }

  pub fn id(n: usize) -> Matrix
  {
    #[allow(non_snake_case)]
    let mut M = zero(n);
    for i in 0..n { M[i][i] = 1.; }
    return M;
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

  pub fn print_mat(A: &Matrix)
  {
    A.iter()
      .for_each(|vec| println!("{:?}", vec));
  }
}
