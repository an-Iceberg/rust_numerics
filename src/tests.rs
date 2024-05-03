use crate::num::deriv;

fn approx_eq(a: f64, b: f64, δ: f64) -> bool
{ a + δ >= b || a - δ <= b }

#[allow(non_upper_case_globals)]
const ε: f64 = 10e-8;

#[test]
fn derivative()
{
  let f1: fn(f64) -> f64 = |x| x.powi(2);
  let f2: fn(f64) -> f64 = |x| x.powi(3) + (3.*x.powi(2));
  let f3: fn(f64) -> f64;
  let f4: fn(f64) -> f64;
  let f5: fn(f64) -> f64;
  let f6: fn(f64) -> f64;
  let f7: fn(f64) -> f64;
  let f8: fn(f64) -> f64;

  let test_suite = [
    (f1, 0., 0, 0.),
    (f1, 2., 0, 4.),
    (f1, 3., 0, 9.),

    // f'
    (f1, 3., 1, 0.),
    (f1, 2., 1, 4.),
    (f1, 3., 1, 6.),
    (f1, 4.5, 1, 9.),
    (f1, 5., 1, 10.),
    (f1, -2., 1, -4.),
    (f1, -2.5, 1, -5.),
    (f1, -5., 1, -10.),
    // f''
    (f1, 1., 2, 2.),
    (f1, 10., 2, 2.),
    (f1, 26., 2, 2.),
    (f1, -6., 2, 2.),
    (f1, -476., 2, 2.),

    (f2, 0., 1, 0.),
    (f2, -1.5, 1, -2.25),
    (f2, -1., 1, -3.),
    (f2, 1., 1, 9.),
    (f2, -3., 1, 9.),
    (f2, 2., 1, 24.),
  ];

  test_suite.iter()
    .for_each(|(f, x, degree, should)|
    {
      assert!(approx_eq(deriv(*f, *x, *degree), *should, ε));
    });
}
