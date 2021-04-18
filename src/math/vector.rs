use std::fmt;
// use std::iter::FromIterator;
use super::common::EPSILON;

#[derive(Copy, Clone)]
pub struct Vector {
	pub x: f64,
	pub y: f64
}

impl Vector {
	// returns f64
	pub fn magnitude (&self) -> f64 { (self.x * self.x + self.y * self.y).sqrt() }
	pub fn magnitude_squared (&self) -> f64 { self.x * self.x + self.y * self.y }
	pub fn dot (&self, u: Vector) -> f64 { self.x * u.x + self.y * u.y }
	pub fn determinant (&self, u: Vector) -> f64 { self.x * u.y - self.y * u.x }
	pub fn distance_to (&self, u: Vector) -> f64 {
		Vector { x: self.x - u.x, y: self.y - u.y }.magnitude()
	}
	// returns vector
	pub fn normalize (&self) -> Vector {
		let mut m = self.magnitude();
		if m < EPSILON { m = 1.0; }
		return Vector { x: self.x / m, y: self.y / m };
	}
	pub fn scale (&self, t: f64) -> Vector {
		Vector { x: self.x * t, y: self.y * t }
	}
	pub fn add (&self, u: Vector) -> Vector {
		Vector { x: self.x + u.x, y: self.y + u.y }
	}
	pub fn subtract (&self, u: Vector) -> Vector {
		Vector { x: self.x - u.x, y: self.y - u.y }
	}
	pub fn flip (&self) -> Vector { Vector { x: -self.x, y: -self.y } }
	pub fn rotate90 (&self) -> Vector { Vector { x: -self.y, y: self.x } }
	pub fn rotate270 (&self) -> Vector { Vector { x: self.y, y: -self.x } }
	pub fn midpoint (&self, u: Vector) -> Vector {
		Vector { x: (self.x + u.x) / 2.0, y: (self.y + u.y) / 2.0 }
	}
	// returns bool
	pub fn degenerate (&self) -> bool { (self.x.abs() + self.y.abs()) < EPSILON }
	pub fn equivalent (&self, u: Vector) -> bool {
		self.x - u.x < EPSILON && self.x - u.x > -EPSILON &&
		self.y - u.y < EPSILON && self.y - u.y > -EPSILON
	}
	pub fn parallel (&self, u: Vector) -> bool {
		(1.0 - self.normalize().dot(u.normalize()).abs()) < EPSILON
	}
	// fn lerp (&self, u: Vector, t: f64) -> Vector {
	// 	let s = 1.0 - t;
	// 	return Vector { x: self.x * s + u.x * t, y: self.y * s + u.y * t };
	// }
}

impl fmt::Debug for Vector {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.debug_struct("Vector")
			.field("x", &self.x)
			.field("y", &self.y)
			.finish()
	}
}
