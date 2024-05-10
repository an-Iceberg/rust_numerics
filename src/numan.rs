pub mod zeros;
pub mod linalg;
pub mod ode;

pub type Matrix = Vec<Vec<f64>>;
pub type Vector = Vec<f64>;

// TODO: docs
// TODO: tests
// TODO: generics, so that any numeric type can be used
// TODO: rename this file to `num`

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

pub fn d_part(f: fn(&Vector) -> f64, x: &Vector, i: usize, h: f64, degree: u8) -> f64
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

pub fn partial(f: fn(&Vector) -> f64, x: &Vector, i: usize, degree: u8) -> f64
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

fn jacobian(f: Vec<fn(&Vector) -> f64>, x: &Vector, ε: f64) -> Matrix
{
  #[allow(non_snake_case)]
  let mut D = vec![vec![0.; x.len()]; f.len()];

  for n in 0..x.len()
  {
    for m in 0..f.len()
    {
      D[m][n] = d_part(f[m], x, n, ε, 1);
    }
  }

  D
}

#[allow(non_snake_case)]
fn Df(f: Vec<fn(&Vector) -> f64>, x: &Vector, ε: f64) -> Matrix
{
  jacobian(f, x, ε)
}

pub fn rect(f: fn(f64) -> f64, a: f64, b: f64, h: f64) -> f64
{
  let n = ((b-a) / h).ceil() as usize;
  #[allow(non_snake_case)]
  let mut Σ = 0.;
  let x = |i| a + (i as f64 * h);
  for i in 0..n
  { Σ += f(x(i) + h/2.); }
  h*Σ
}

#[allow(non_snake_case)]
pub fn Rf(f: fn(f64) -> f64, a: f64, b: f64, h: f64) -> f64
{
  rect(f, a, b, h)
}

pub fn trapez(f: fn(f64) -> f64, a: f64, b: f64, h: f64) -> f64
{
  let n = ((b-a) / h).ceil() as usize;
  #[allow(non_snake_case)]
  let mut Σ = 0.;
  let x = |i| a + (i as f64 * h);
  for i in 0..n
  { Σ += f(x(i)); }
  h*((f(a) + f(b)) / 2. + Σ)
}

#[allow(non_snake_case)]
pub fn Tf(f: fn(f64) -> f64, a: f64, b: f64, h: f64) -> f64
{
  trapez(f, a, b, h)
}

pub fn simpson(f: fn(f64) -> f64, a: f64, b: f64, h: f64) -> f64
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

#[allow(non_snake_case)]
pub fn Sf(f: fn(f64) -> f64, a: f64, b: f64, h: f64) -> f64
{
  simpson(f, a, b, h)
}

/// Calculates the area under the curve (integral) of the function `f` using
/// simpson's rule. `h` is the precision.
pub fn int(f: fn(f64) -> f64, a: f64, b: f64, h: f64) -> f64
{
  simpson(f, a, b, h)
}

pub fn gauss_newton(f: &Vec<fn(&Vector) -> f64>, λ0: &Vector, ε: f64) -> Vector
{
  todo!()
}

pub fn gauss_newton_dampened(f: &Vec<fn(&Vector) -> f64>, λ0: &Vector, ε: f64) -> Vector
{
  todo!()
}

// Todo: romberg with simpson?
