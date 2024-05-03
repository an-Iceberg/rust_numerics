// TODO: docs
// TODO: tests
// TODO: generics, so that any numeric type can be used
// TODO: rename this file to `num`

use std::ops::{Div, Mul};

// ToDo: create one fn `deriv` which then calls different functions based on parameters
// such as dergee and which method (standard, 2 point, 5 point)

// https://en.wikipedia.org/wiki/Numerical_differentiation#Higher_derivatives
pub fn d(f: fn(f64) -> f64,x: f64, h: f64, degree: u8) -> f64
{
  match degree
  {
    1 => (f(x+h) - f(x-h)) / (2.*h),
    2 => (f(x+h) - 2.*f(x) + f(x-h)) / (h.powi(2)),
    n =>
    {
      #[allow(non_snake_case)]
      let mut Σ = 0.;
      for k in 0..=n
      {
        Σ += (-1_f64).powf((k + n).into()) * nCr(n.into(), k.into()) * f(x + (k as f64*h));
      }

      (1./h.powf(n.into())) * Σ
    }
  }
  // Note: 5 point method: (f(x - 2.*h) - 8.*f(x - h) + 8.*f(x + h) - f(x + 2.*h)) / (12. * h)
}

pub fn deriv(f: fn(f64) -> f64, x: f64, degree: u8) -> f64
{
  if degree == 0
  { return f(x); }

  let (mut h, mut δold) = (1., 10.);
  let mut δnew;
  let mut f2 = d(f, x, h, degree);
  let mut f1 = f2 + 10.;

  loop
  // for _ in 0..=20
  {
    f2 = d(f, x, h, degree);
    δnew = (f1 - f2).abs();
    // Value is gaining imprecision again, return previous result
    if δold < δnew || δnew == 0. { return f1; }
    δold = δnew;
    f1 = f2;
    h /= 10.;
  }
}

pub fn d_part(f: fn(&Vec<f64>) -> f64, x: &Vec<f64>, i: usize, h: f64, degree: u8) -> f64
{
  match degree
  {
    0 => f(x),
    1 =>
    {
      let (mut x_plus_h, mut x_minus_h) = (x.clone(), x.clone());
      x_plus_h[i] += h; x_minus_h[i] -= h;
      (f(&x_plus_h) - f(&x_minus_h)) / (2. * h)
    },
    2 =>
    {
      let (mut x_plus_h, mut x_minus_h) = (x.clone(), x.clone());
      x_plus_h[i] += h; x_minus_h[i] -= h;
      (f(&x_plus_h) - (2.*f(x)) + f(&x_minus_h)) / h.powi(2)
    }
    _ => todo!()
  }
}

pub fn partial(f: fn(&Vec<f64>) -> f64, x: &Vec<f64>, i: usize, degree: u8) -> f64
{
  if i >= x.len()
  {
    panic!("{}", format!("Index i = {} out of range for x of length {}", i, x.len()));
  }

  // TODO: implement this

  let h = 1.;
  d_part(f, x, i, h, degree)
}

#[allow(non_snake_case)]
fn nCr(n: u64, r: u64) -> f64
{
  // Note: https://www.geeksforgeeks.org/program-calculate-value-ncr/
  fact(n) / (fact(r) * fact(n-r))
}

fn fact(n: u64) -> f64
{
  // Note: https://users.rust-lang.org/t/whats-the-idiomatic-or-preferred-way-of-calculating-factorials/77684/3
  // https://docs.rs/factorial/latest/factorial/
  let mut x = 1.;
  for i in 1..=n { x *= i as f64; }
  x
}

/// Calculates the area under the curve (integral) of the function `f` using
/// simpson's rule. `h` is the precision.
pub fn int(f: fn(f64) -> f64, a: f64, b: f64, h: f64) -> f64
{
  let n = ((b - a) / h).ceil() as u64;
  #[allow(non_snake_case)]
  let (mut Σ1, mut Σ2) = (0., 0.);

  let x = |i| a + (i as f64 * h);

  for i in 1..n
  { Σ1 += f(x(i)); }

  for (i, j) in (0..=n).zip((0..=n).skip(1))
  { Σ2 += f((x(i) + x(j)) / 2.); }

  (h/3.)*((f(a)/2.) + Σ1 + (2.*Σ2) + (f(b)/2.))
}

// Todo: romberg with simpson?

// Note: https://en.wikipedia.org/wiki/Root-finding_algorithms
// Note: https://en.wikipedia.org/wiki/Householder%27s_method
// Note: https://en.wikipedia.org/wiki/Gauss%E2%80%93Seidel_method

// Todo: tolerance should take ε into account
/// [Formula](https://en.wikipedia.org/wiki/Nested_intervals)
pub fn bisection(f: fn(f64) -> f64, a: f64, b: f64, tolerance: f64) -> Result<f64, String>
{
  let (mut a, mut b) = (a, b);

  // If both have the same sign we can't know which half to keep searching in
  if (f(a) > 0. && f(b) > 0.) || (f(a) < 0. && f(b) < 0.)
  { return Err("Both bounds have the same sign. Can't determine, which half to search.".to_string()); }

  // a shall be < 0 and b shall be > 0
  if f(a) > 0. && f(b) < 0.
  {
    std::mem::swap(&mut a, &mut b);
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

  Some(x3)
}

/// [Formula](https://en.wikipedia.org/wiki/Newton%27s_method#)
pub fn newton_raphson(f: fn(f64) -> f64, x0: f64, tolerance: f64) -> Option<f64>
{
  // ToDo: fail states
  let mut x1 = x0;
  let mut x2 = x1 + 2. * tolerance;

  while (x1 - x2).abs() > tolerance
  {
    x1 = x2;
    x2 = x1 - f(x1).div(deriv(f, x1, 1));
  }

  Some(x2)
}

pub mod linalg
{
  pub type Matrix = Vec<Vec<f64>>;
  pub type Vector = Vec<f64>;

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
}
