use std::fmt;
use std::ops::*;

use crate::utils::random_in_unit_sphere;

type Num = f32;
pub type Point3d = Vec3; // 3D point
pub type Color = Vec3; // RGB color

#[derive(Clone, Debug, Copy)]
pub struct Vec3 {
  pub e: [Num; 3],
}

#[allow(dead_code)]
impl Vec3 {
  pub const fn new(a: Num, b: Num, c: Num) -> Vec3 {
    Vec3 { e: [a, b, c] }
  }

  pub fn uni(v: f32) -> Vec3 {
    Vec3::new(v, v, v)
  }

  pub fn zero() -> Vec3 {
    Vec3::uni(0.0)
  }

  pub fn one() -> Vec3 {
    Vec3::uni(1.0)
  }

  pub fn rand_unit() -> Vec3 {
    random_in_unit_sphere().unit_vector()
  }

  pub fn right() -> Vec3 {
    // +x
    Vec3::new(1.0, 0.0, 0.0)
  }

  pub fn up() -> Vec3 {
    // +y
    Vec3::new(0.0, 1.0, 0.0)
  }

  pub fn forward() -> Vec3 {
    // -z
    Vec3::new(0.0, 0.0, -1.0)
  }

  pub fn x(&self) -> Num {
    self.e[0]
  }
  pub fn y(&self) -> Num {
    self.e[1]
  }
  pub fn z(&self) -> Num {
    self.e[2]
  }

  pub fn length(&self) -> Num {
    self.length_squared().sqrt()
  }

  pub fn length_squared(&self) -> Num {
    self.dot(*self)
  }

  // Return true if the vector is close to zero in all dimensions.
  pub fn near_zero(&self) -> bool {
    let eps = 1e-8;
    let is_smol = |x: f32| -> bool { x.abs() < eps };
    is_smol(self.x()) && is_smol(self.y()) && is_smol(self.z())
  }

  pub fn dot(&self, v: Vec3) -> Num {
    self.x() * v.x() + self.y() * v.y() + self.z() * v.z()
  }

  pub fn cross(&self, v: Vec3) -> Vec3 {
    let u = self;
    Vec3::new(
      u.e[1] * v.e[2] - u.e[2] * v.e[1],
      u.e[2] * v.e[0] - u.e[0] * v.e[2],
      u.e[0] * v.e[1] - u.e[1] * v.e[0],
    )
  }

  pub fn unit_vector(&self) -> Vec3 {
    let l: Num = self.length();
    self.div(l)
  }
}

impl Default for Vec3 {
  fn default() -> Self {
    Vec3::zero()
  }
}

impl Not for Vec3 {
  type Output = Self;
  fn not(self) -> Vec3 {
    Vec3::new(-self.x(), -self.y(), -self.z())
  }
}

impl Add for Vec3 {
  type Output = Self;
  fn add(self, other: Self) -> Self {
    Self::new(
      self.x() + other.x(),
      self.y() + other.y(),
      self.z() + other.z(),
    )
  }
}

impl Sub for Vec3 {
  type Output = Self;
  fn sub(self, other: Self) -> Self {
    self + !other
  }
}

impl Mul<Num> for Vec3 {
  type Output = Self;
  fn mul(self, rhs: Num) -> Self {
    Self::new(self.x() * rhs, self.y() * rhs, self.z() * rhs)
  }
}

impl Mul<Vec3> for Vec3 {
  type Output = Self;
  fn mul(self, rhs: Vec3) -> Self {
    Self::new(self.x() * rhs.x(), self.y() * rhs.y(), self.z() * rhs.z())
  }
}

impl Div<Num> for Vec3 {
  type Output = Self;
  fn div(self, rhs: Num) -> Self {
    self * (1.0 / rhs)
  }
}

impl Index<usize> for Vec3 {
  type Output = Num;

  fn index(&self, i: usize) -> &Self::Output {
    &self.e[i]
  }
}

impl IndexMut<usize> for Vec3 {
  fn index_mut(&mut self, i: usize) -> &mut Self::Output {
    &mut self.e[i]
  }
}

impl fmt::Display for Vec3 {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "Vec3({}, {}, {})", self.x(), self.y(), self.z())
  }
}

#[cfg(test)]
mod tests {
  use crate::vec3::Num;
  use crate::vec3::Vec3;
  use assert_approx_eq::assert_approx_eq;

  fn assert_eq_v(v: Vec3, a: Num, b: Num, c: Num) {
    assert_approx_eq!(v.x(), a, 0.01);
    assert_approx_eq!(v.y(), b, 0.01);
    assert_approx_eq!(v.z(), c, 0.01);
  }

  #[test]
  fn setters() {
    let mut v = Vec3::default();
    v[0] = 1.0;
    v[1] = 2.0;
    v[2] = 3.0;
    assert_eq_v(v, 1.0, 2.0, 3.0);
  }

  #[test]
  fn negate() {
    let v = Vec3::new(1.0, 2.0, 3.0);
    assert_eq_v(!v, -1.0, -2.0, -3.0);
  }

  #[test]
  fn add() {
    let v = Vec3::new(1.0, 2.0, 3.0);
    let b = Vec3::new(4.0, 5.0, 6.0);
    assert_eq_v(v + b, 5.0, 7.0, 9.0);
  }

  #[test]
  fn sub() {
    let b = Vec3::new(4.0, 1.0, 10.0);
    let v = Vec3::new(1.0, 2.0, 3.0);
    assert_eq_v(b - v, 3.0, -1.0, 7.0);
  }

  #[test]
  fn mul() {
    let v = Vec3::new(1.0, 2.0, 3.0);
    assert_eq_v(v * 2.0, 2.0, 4.0, 6.0);
  }

  #[test]
  fn div() {
    let v = Vec3::new(1.0, 2.0, 3.0);
    assert_eq_v(v / 2.0, 0.5, 1.0, 1.5);
  }

  #[test]
  fn length() {
    let v = Vec3::new(1.0, 2.0, 3.0);
    let r: f32 = 1.0 + 4.0 + 9.0;
    assert_approx_eq!(v.length(), r.sqrt());
  }

  #[test]
  fn length_squared() {
    let v = Vec3::new(1.0, 2.0, 3.0);
    assert_approx_eq!(v.length_squared(), 1.0 + 4.0 + 9.0);
  }

  #[test]
  fn dot() {
    let v = Vec3::new(1.0, 2.0, 3.0);
    let b = Vec3::new(4.0, 5.0, 6.0);
    assert_approx_eq!(v.dot(b), 4.0 + 10.0 + 18.0);
  }

  #[test]
  fn cross() {
    let a = Vec3::new(3.0, -3.0, 1.0);
    let b = Vec3::new(4.0, 9.0, 2.0);
    assert_eq_v(a.cross(b), -15.0, -2.0, 39.0);
  }

  #[test]
  fn unit_vector() {
    let v = Vec3::new(1.0, 2.0, 3.0);
    let unit = v.unit_vector();
    assert_approx_eq!(unit.length(), 1.0);
    assert_eq_v(v, 1.0, 2.0, 3.0);
  }
}
