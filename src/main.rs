extern crate alga;
#[macro_use]
extern crate decimal;
extern crate num_traits;
extern crate ncollide2d;

use ncollide2d::shape::Cuboid;
use ncollide2d::math::Vector;
use decimal::d128;
use num_traits::Bounded;


fn main() {
//  let max = d128!();
 // println!("max: {}", max);

/*
  let max = d128!(Infinity).previous();
  let min = d128!(-Infinity).next();
  println!("{}", max);
  println!("{}", min);

  let tmax = d128::max_value();
  let tmin = d128::min_value();
  println!("{}", tmax);
  println!("{}", tmin);
*/

  let half_w = d128!(4.27);
  let half_h = d128!(3.65);

  let vector = Vector::new(half_w, half_h);
  let cuboid = Cuboid::new(vector);

  assert!(half_w == cuboid.half_extents().x);
  assert!(half_h == cuboid.half_extents().y);
}
