use crate::num::linalg::Matrix;

mod num;
#[cfg(test)]
mod numerics_tests;

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
  println!("{:?}", num::newton_zero(f, 100., 1e-10));
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
}
