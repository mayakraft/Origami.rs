// export these under the module axioms::
pub mod axioms;

// // export these under the top level
// mod axioms;
// pub use axioms::axiom1;
// pub use axioms::axiom2;
// pub use axioms::axiom3;
// pub use axioms::axiom4;
// pub use axioms::axiom5;
// pub use axioms::axiom6;
// pub use axioms::axiom7;

// export these under the top level
mod math;
pub use math::Vector;
pub use math::Line;
pub use math::Segment;
pub use math::Rect;

// additional static constructors

// prefer to use this constructor when making a rect, or at least,
// it's very important that the "sides" are Lines with normals
// that point outwards. needed for the "contains" method.
pub fn make_square () -> Rect {
	Rect { sides: [
		Line { u: Vector { x: 0.0 , y: 1.0 }, d: 0.0 },
		Line { u: Vector { x: 1.0 , y: 0.0 }, d: 1.0 },
		Line { u: Vector { x: 0.0 , y: -1.0 }, d: -1.0 },
		Line { u: Vector { x: -1.0 , y: 0.0 }, d: 0.0 }
	]}
}
