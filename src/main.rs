use std::f64::consts::E;

use crate::num::{deriv, linalg::Matrix};

mod num;
#[cfg(test)]
mod tests;

fn main()
{
  fn f(x: f64) -> f64 { x.powi(2) + x - 1. }
  fn f1(x: Vec<f64>) -> f64 { x[0].powi(2) - x[1].powi(2) }

  println!("f1([3,2]) = {}", f1(vec![3., 2.]));

  println!("f(3) should:11 is:{}", f(3.));
  println!("f(4) should:19 is:{}", f(4.));
  println!("f'(3) should:7 is:{} ε:{}", num::deriv(f, 3., 1), (num::deriv(f, 3., 1) - 7.).abs());
  println!("f'(4) should:9 is:{} ε:{}", num::deriv(f, 4., 1), (num::deriv(f, 4., 1) - 9.).abs());
  println!();
  println!("{:?}", num::bisection(f, -10., 0., 1e-20));
  println!();
  println!("{:?}", num::newton_raphson(f, 100., 1e-10));
  println!();
  println!("{:?}", num::secant_zero(f, 100., 98., 1e-10));

  #[allow(non_snake_case)]
  let A: Matrix = vec![
    vec![1., 2., 3.],
    vec![4., 5., 6.],
    vec![7., 8., 9.],
  ];

  println!("\nA[2][0] ⇒  should:7 is:{}\n", A[2][0]);
  // ↑ this is good, it means we can take the indices 1 to 1 from the math formulas

  num::linalg::print_mat(&A);
  // println!("{}", 1e-5);
  // println!("{}", 10e-5);
  println!("f'(-0.25) should:0.5 is:{} ε:{}", num::deriv(f, -0.25, 1), (num::deriv(f, -0.25, 1) - 0.5).abs());
  println!("{}", num::deriv(|x| x.powi(3) - x.powi(2), 1., 1));

  let h = 0.01;
  println!("∫[1,5] e^x dx for h = {} should:145.69487727 is:{}", h, num::int(|x| E.powf(x), 1., 5., h));

  println!("{}", num::linalg::mat_to_string(&A));

  let f: fn(f64) -> f64 = |x| x.powi(3) + (3.*x.powi(2));

  println!("b: {}", deriv(f, 0., 1));
  println!("{}", deriv(|x| x.powi(2), 26., 2));
}
