use std::fmt;
// use std::iter::FromIterator;
use super::vector::Vector;

#[derive(Copy, Clone)]
pub struct Segment {
	pub a: Vector,
	pub b: Vector
}

impl Segment {
	// given we already know these two segments are collinear
	// check if they also overlap
	pub fn quick_overlap (&self, b: Segment) -> bool {
		// seg a
		let vec_a = self.b.subtract(self.a);
		let mag_a = vec_a.magnitude();
		let norm_a = vec_a.normalize();
		// seg b
		let vec_b = b.b.subtract(b.a);
		let mag_b = vec_b.magnitude();
		let norm_b = vec_b.normalize();
		// project the other other segment's points onto its vector
		let aa = norm_b.dot(self.a.subtract(b.a));
		let ab = norm_b.dot(self.b.subtract(b.a));
		let ba = norm_a.dot(b.a.subtract(self.a));
		let bb = norm_a.dot(b.b.subtract(self.a));
		let t1 = aa >= 0.0 && aa <= mag_b;
		let t2 = ab >= 0.0 && ab <= mag_b;
		let t3 = ba >= 0.0 && ba <= mag_a;
		let t4 = bb >= 0.0 && bb <= mag_a;
		return t1 || t2 || t3 || t4;
	}
}

impl fmt::Debug for Segment {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.debug_struct("Segment")
			.field("x1", &self.a.x)
			.field("y1", &self.a.y)
			.field("x2", &self.b.x)
			.field("y2", &self.b.y)
			.finish()
	}
}

