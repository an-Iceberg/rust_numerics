mod num;
#[cfg(test)]
mod numerics_tests;

fn main()
{
  fn f(x: f64) -> f64 { return x.powi(2) + x - 1.; }

  println!("f(2) = {}", f(2.));
  println!("f'(2) = {:.3}", num::deriv1_2(f, 2.));
  println!();
  println!("f(2) = {}", f(2.));
  println!("f'(2) = {:.3}", num::deriv1_5(f, 2.));
  println!();
  println!("f(-1) = {}", f(-1.));
  println!("f'(-1) = {:.3}", num::deriv1_2(f, -1.));
  println!();
  println!("{:.3?}", num::bisection(f, -10., 0., 1e-20));
  println!();
  println!("{:.3?}", num::newton_zero(f, 100., 1e-5));
  println!();
  println!("{:.3?}", num::secant_zero(f, 100., 98., 1e-5));
}
