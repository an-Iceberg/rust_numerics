// TODO: docs

use std::ops::{Div, Mul};

/// Calculates the first derivative (f'(x)) of `f` at `x` using the formula for
/// the definition of the derivative.
pub fn deriv1(f: fn(f64) -> f64, x: f64) -> f64
{
  let h = 0.001;
  return (f(x + h) - f(x)) / h;
}

/// Calculates the first derivative (f'(x)) of `f` at `x` using a more precise
/// formula.
pub fn deriv1_2(f: fn(f64) -> f64, x: f64) -> f64
{
  let h = 0.001;
  return (f(x + h) - f(x - h)) / (2. * h);
}

/// Calculates the first derivative (f'(x)) of `f` at `x` using a 5-point stencil
/// method.
pub fn deriv1_5(f: fn(f64) -> f64, x: f64) -> f64
{
  // Todo: implement ⇒ https://en.wikipedia.org/wiki/Numerical_differentiation#Higher-order_methods
  let h = 0.001;
  return (f(x - 2.*h) - 8.*f(x - h) + 8.*f(x + h) - f(x + 2.*h)) / (12. * h);
}

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
  let mut x1 = x0;
  let mut x2 = x1 + 2. * tolerance;

  while (x1 - x2).abs() > tolerance
  {
    x1 = x2;
    x2 = x1 - f(x1).div(deriv1(f, x1));
  }

  return Some(x2);
}

pub mod linalg
{
  type Matrix = Vec<Vec<f64>>;
  type Vector = Vec<f64>;

  pub fn zero(n: usize) -> Matrix
  {
    todo!()
  }

  pub fn id(n: usize) -> Matrix
  {
    todo!()
  }

  #[allow(non_snake_case)]
  pub fn matmul(A: Matrix, B: Matrix) -> Result<Matrix, String>
  {
    todo!()
  }

  pub fn lr(M: Matrix) -> (Matrix, Matrix)
  {
    todo!()
  }

  pub fn qr(M: Matrix) -> (Matrix, Matrix)
  {
    todo!()
  }

  pub fn gauss_seidel(A: Matrix, b: Vector) -> Vector
  {
    todo!()
  }
}
