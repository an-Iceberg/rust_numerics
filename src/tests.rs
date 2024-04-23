use crate::num::deriv;

fn approx_eq(a: f64, b: f64, δ: f64) -> bool
{
  if a + δ <= b || a - δ >= b
  { return false; }
  true
}

#[test]
fn derivative()
{
  let f: fn(f64) -> f64 = |x| x.powi(2);

  assert_eq!(deriv(f, 0., 0), 0.);
  assert_eq!(deriv(f, 2., 0), 4.);
  assert_eq!(deriv(f, 3., 0), 9.);

  assert!(approx_eq(deriv(f, 0., 1), 0., 10e-5));
  assert!(approx_eq(deriv(f, 2., 1), 4., 10e-5));
  assert!(approx_eq(deriv(f, 3., 1), 6., 10e-5));
  assert!(approx_eq(deriv(f, 4.5, 1), 9., 10e-5));
  assert!(approx_eq(deriv(f, 5., 1), 10., 10e-5));

  assert!(approx_eq(deriv(f, -2., 1), -4., 10e-5));
  assert!(approx_eq(deriv(f, -2.5, 1), -5., 10e-5));
  assert!(approx_eq(deriv(f, -5., 1), -10., 10e-5));

  assert!(approx_eq(deriv(f, 1., 2), 2., 10e-5));
  assert!(approx_eq(deriv(f, 10., 2), 2., 10e-5));
  assert!(approx_eq(deriv(f, 26., 2), 2., 10e-5));
  assert!(approx_eq(deriv(f, -6., 2), 2., 10e-5));
  assert!(approx_eq(deriv(f, -476., 2), 2., 10e-5));

  let f: fn(f64) -> f64 = |x| x.powi(3) + (3.*x.powi(2));

  assert!(approx_eq(deriv(f, 0., 1), 0., 10e-5));
  assert!(approx_eq(deriv(f, -1.5, 1), -2.25, 10e-5));
  assert!(approx_eq(deriv(f, -1., 1), -3., 10e-5));
  assert!(approx_eq(deriv(f, 1., 1), 9., 10e-5));
  assert!(approx_eq(deriv(f, -3., 1), 9., 10e-5));
  assert!(approx_eq(deriv(f, 2., 1), 24., 10e-5));
}
