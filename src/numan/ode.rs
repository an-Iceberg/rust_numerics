use super::Vector;

pub fn euler(f: fn(f64, f64) -> f64, a: f64, b: f64, h: f64, y0: f64) -> (Vector, Vector)
{
  let n = ((b - a) / h).ceil() as usize;
  let mut x = vec![a];
  let mut y = vec![y0];
  for i in 0..n
  {
    x.push(x[i] + h);
    y.push(y[i] + h*f(x[i], y[i]));
  }
  return (x,y);
}

pub fn midpoint(f: fn(f64, f64) -> f64, a: f64, b: f64, h: f64, y0: f64) -> (Vector, Vector)
{
  let n = ((b - a) / h).ceil() as usize;
  let mut x = vec![a];
  let mut y = vec![y0];
  for i in 0..n
  {
    x.push(x[i] + h);
    let xh2 = x[i] + h/2.;
    let yh2 = y[i] + (h/2.)*f(x[i], y[i]);
    y.push(y[i] + h*f(xh2, yh2));
  }
  return (x, y);
}

pub fn mod_euler(f: fn(f64, f64) -> f64, a: f64, b: f64, h: f64, y0: f64) -> (Vector, Vector)
{
  let n = ((b - a) / h).ceil() as usize;
  let mut x = vec![a];
  let mut y = vec![y0];
  for i in 0..n
  {
    x.push(x[i] + h);
    let k1 = f(x[i], y[i]);
    let k2 = f(x[i + 1], y[i] + h*f(x[i], y[i]));
    y.push(y[i] + h*((k1 + k2) / 2.));
  }
  return (x, y);
}

pub fn runge_kutta_4(f: fn(f64, f64) -> f64, a: f64, b: f64, h: f64, y0: f64) -> (Vector, Vector)
{
  let n = ((b - a) / h).ceil() as usize;
  let mut x = vec![a];
  let mut y = vec![y0];
  for i in 0..n
  {
    x.push(x[i] + h);
    let k1 = f(x[i], y[i]);
    let k2 = f(x[i] + h/2., y[i] + (h/2.)*k1);
    let k3 = f(x[i] + h/2., y[i] + (h/2.)*k2);
    let k4 = f(x[i] + h, y[i] + h*k3);
    y.push(y[i] + h*(1./6.)*(k1 + 2.*k2 + 2.*k3 + k4));
  }
  return (x, y);
}

#[allow(non_snake_case)]
pub fn RK4(f: fn(f64, f64) -> f64, a: f64, b: f64, h: f64, y0: f64) -> (Vector, Vector)
{
  runge_kutta_4(f, a, b, h, y0)
}
