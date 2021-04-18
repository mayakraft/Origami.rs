// use std::fmt;
// use std::iter::FromIterator;
use super::vector::Vector;
use super::line::Line;
use super::segment::Segment;
use super::common::EPSILON;

#[derive(Copy, Clone)]
pub struct Rect {
	pub sides: [Line; 4]
}

const NULL_SEGMENT: Segment = Segment {
	a: Vector {x:0.0, y:0.0},
	b: Vector {x:0.0, y:0.0}
};

impl Rect {
	// todo: this is currently hard-coded to a unit square
	pub fn contains (&self, p: Vector) -> bool {
		p.x >= 0.0 && p.x <= 1.0 &&
		p.y >= 0.0 && p.y <= 1.0
	}
	// @returns a tuple: true/false if clip is possible and the segment.
	pub fn clip (&self, l: Line) -> (bool, Segment) {
		// test intersection with every side, exclude pts outside polygon
		let results: Vec<Vector> = self.sides.iter()
			.map(|line| line.intersect(l))
			.map(|(success, seg)| (success, self.contains(seg), seg))
			.filter(|el| el.0 && el.1)
			.map(|el| el.2)
			.collect();
		if results.len() < 2 { return (false, NULL_SEGMENT); }
		// sort intersection points along line
		let origin = l.u.scale(l.d);
		let vector = l.u.rotate90();
		let ts: Vec<f64> = results.iter()
			.map(|pt| pt.subtract(origin).dot(vector))
			.collect();
		// get the min and max, construct a segment between them
		let min = *ts.iter().fold(&ts[0], |a, b| if b < a {b} else {a});
		let max = *ts.iter().fold(&ts[0], |a, b| if b > a {b} else {a});
		// if the two points are the same the segment is degenerate
		if max - min < EPSILON { return (false, NULL_SEGMENT); }
		return (true, Segment {
			a: origin.add(vector.scale(min)),
			b: origin.add(vector.scale(max))
		});
	}
}
