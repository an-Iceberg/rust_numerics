// Note: https://en.wikipedia.org/wiki/Root-finding_algorithms
// Note: https://en.wikipedia.org/wiki/Householder%27s_method
// Note: https://en.wikipedia.org/wiki/Gauss%E2%80%93Seidel_method

use std::ops::{Div, Mul};

use super::{deriv, Vector};

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
pub fn secant(f: fn(f64) -> f64, x0: f64, x1: f64, tolerance: f64) -> Option<f64>
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

pub fn newton_raphson_multidim(f: &Vec<fn(&Vector) -> f64>, x0: &Vector, ε: f64) -> Vector
{
  todo!()
}
