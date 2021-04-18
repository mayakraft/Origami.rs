use std::fmt;
// use std::iter::FromIterator;
use super::vector::Vector;
use super::segment::Segment;
use super::common::EPSILON;

#[derive(Copy, Clone)]
pub struct Line {
	pub u: Vector,
	pub d: f64
}

impl Line {
	// @returns a tuple: true/false if intersection is possible and the point.
	pub fn intersect (&self, l: Line) -> (bool, Vector) {
		let det = self.u.determinant(l.u);
		if det.abs() < EPSILON {
			return (false, Vector { x: 0.0, y: 0.0 });
		}
		let x = self.d * l.u.y - l.d * self.u.y;
		let y = l.d * self.u.x - self.d * l.u.x;
		return (true, Vector { x: x / det, y: y / det });
	}
	pub fn equivalent (&self, l: Line) -> bool {
		// check if lines are parallel
		(self.u.dot(l.u.rotate90()).abs() < EPSILON) &&
		// instead of simply comparing the .d values,
		// scale the incoming by the dot prod of both .u normals
		// this allows (1,0) and (-1,0) to be treated the same
		((self.d - l.d * self.u.dot(l.u)).abs() < EPSILON)
	}
	// use this line as a mirror plane, reflect the point to the other side
	pub fn reflect_vector (&self, p: Vector) -> Vector {
		let v1 = self.u.scale(self.d);
		let rot90 = self.u.rotate90();
		let v2 = rot90.scale(p.dot(rot90));
		let projection = v1.add(v2);
		return projection.add(projection.subtract(p));
	}
	// use this line as a mirror plane, reflect a segment to the other side
	pub fn reflect_segment (&self, s: Segment) -> Segment {
		Segment {
			a: self.reflect_vector(s.a),
			b: self.reflect_vector(s.b)
		}
	}
}

impl fmt::Debug for Line {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.debug_struct("Line")
			.field("x", &self.u.x)
			.field("y", &self.u.y)
			.field("d", &self.d)
			.finish()
	}
}
