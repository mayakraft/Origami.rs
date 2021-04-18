extern crate rabbit_ear as ear;

mod tests {

	use ear::axioms;
	use ear::Vector;
	use ear::Line;
	use ear::Segment;
	use ear::Rect;
	use ear::make_square;

	const EPSILON: f64 = f64::EPSILON * 10.0;

	macro_rules! assert_delta {
		($x:expr, $y:expr, $d:expr) => {
			assert_eq!(true, $x - $y < $d && $y - $x < $d);
		}
	}

	#[test]
	fn vector_tests () {
		let sqrt2 = (2.0_f64).sqrt();
		let v: Vector = Vector { x: 1.2, y: -0.8 };
		let u: Vector = Vector { x: 2.0, y: 2.0 };
		let l = Line { u: Vector { x: 1.0, y: 0.0 }, d: 1.0 };
		let m = Line { u: Vector { x: -sqrt2, y: sqrt2 }, d: 1.0 };
		// let m = Line { u: Vector { x: 0.0, y: 1.0 }, d: 1.0 };
		let mag1: f64 = u.magnitude();
		let mag2: f64 = u.normalize().magnitude();
		let mag3: f64 = u.magnitude_squared();
		let norm: Vector = u.normalize();
		let rot90: Vector = u.normalize().rotate90();
		let rot270: Vector = u.normalize().rotate270();
		let flip: Vector = v.flip();
		let dot: f64 = u.dot(v);
		let determ: f64 = v.determinant(u);
		let degenerate: bool = u.degenerate();
		let parallel: bool = u.parallel(v);
		let (success, intersect) = l.intersect(m);
		let equivalent: bool = u.equivalent(v);
		assert_delta!(mag1, 2.8284271247461903, EPSILON);
		assert_delta!(mag2, 1.0, EPSILON);
		assert_delta!(mag3, 8.0, EPSILON);
		assert_delta!(norm.x, (2.0_f64).sqrt() / 2.0, EPSILON);
		assert_delta!(norm.y, (2.0_f64).sqrt() / 2.0, EPSILON);
		assert_delta!(rot90.x, -(2.0_f64).sqrt() / 2.0, EPSILON);
		assert_delta!(rot90.y, (2.0_f64).sqrt() / 2.0, EPSILON);
		assert_delta!(rot270.x, (2.0_f64).sqrt() / 2.0, EPSILON);
		assert_delta!(rot270.y, -(2.0_f64).sqrt() / 2.0, EPSILON);
		assert_delta!(flip.x, -1.2, EPSILON);
		assert_delta!(flip.y, 0.8, EPSILON);
		assert_delta!(dot, 0.8, EPSILON);
		assert_delta!(determ, 4.0, EPSILON);
		assert_eq!(degenerate, false);
		assert_eq!(parallel, false);
		assert_eq!(success, true);
		assert_eq!(equivalent, false);
		assert_delta!(intersect.x, 1.0, EPSILON);
		assert_delta!(intersect.y, 1.0_f64 + 2.0_f64.sqrt() / 2.0, EPSILON);
	}

	#[test]
	fn line_tests () {
		let a = Line {
			u: Vector { x: 0.7071067811865475, y: 0.7071067811865475},
			d: 0.7071067811865475
		};
		let b = Line {
			u: Vector { x: 1.0, y: 0.0},
			d: 0.5
		};
		let equivalent_a: bool = a.equivalent(b);
		let equivalent_b: bool = b.equivalent(a);
		let reflect1 = a.reflect_vector(Vector { x: 0.2, y: -0.2 });
		let _reflect2 = a.reflect_segment(Segment { a: Vector{x:0.0, y:0.0}, b: Vector {x:1.0, y:1.0} });
		assert_eq!(equivalent_a, false);
		assert_eq!(equivalent_b, false);
		assert_delta!(reflect1.x, 1.2, EPSILON);
		assert_delta!(reflect1.y, 0.8, EPSILON);

		// make sure these should be duplicate
		// test if they are duplicate
		// duplicate test Line { x: -1.0, y: 0.0, d: -0.5 } Line { x: 1.0, y: 0.0, d: 0.5 }
	}

	#[test]
	fn axiom_tests () {
		let unit_square: Rect = make_square();

		let t: Vector = Vector { x: 1.01, y: 0.0 };
		let o: Vector = Vector { x: 0.0, y: 0.0 };
		let p: Vector = Vector { x: 0.5, y: 0.75 };
		// let u: Vector = Vector { x: 2.0, y: 2.0 };
		let v: Vector = Vector { x: 1.2, y: -0.8 };
		let w: Vector = Vector { x: 6.0, y: 13.0 }.normalize();
		let l: Line = Line { u: Vector { x: 1.0, y: 0.0 }, d: 1.0 };
		let m: Line = Line { u: Vector { x: 0.0, y: 1.0 }, d: 1.0 };
		let n: Line = Line { u: Vector { x: 0.0, y: 1.0 }, d: 0.5 };
		let r: Line = Line { u: w, d: 3.0 };
		let s: Line = Line { u: Vector { x: 1.0, y: 0.0 }, d: 2.0 };
		let ax1 = axioms::axiom1(o, p, unit_square);
		let ax2 = axioms::axiom2(o, p, unit_square);
		let _ax3a = axioms::axiom3(l, m, unit_square);
		let ax3b = axioms::axiom3(m, n, unit_square);
		let _ax4 = axioms::axiom4(v, r, unit_square);
		let _ax5 = axioms::axiom5(t, o, s, unit_square);
		let _ax6 = axioms::axiom6(t, o, s, r, unit_square);
		let _ax7 = axioms::axiom7(o, r, l, unit_square);
		// assert_eq!(ax3a.len(), 1);
		// assert_delta!(ax3a[0].u.x, 0.7071067811865475, EPSILON);
		// assert_delta!(ax3a[0].u.y, -0.7071067811865475, EPSILON);
		// assert_delta!(ax3a[0].d, 0.0, EPSILON);
		assert_eq!(ax1.len(), 1);
		// println!("1: {:?}", ax1[0]);
		// println!("2: {:?}", ax2[0]);
		// assert_delta!(ax1[0].u.x, 0.9615239476408233, EPSILON);
		// assert_delta!(ax1[0].u.y, -0.2747211278973781, EPSILON);
		// assert_delta!(ax1[0].d, 1.3736056394868903, EPSILON);
		assert_eq!(ax2.len(), 1);
		// assert_delta!(ax2[0].u.x, -0.2747211278973781, EPSILON);
		// assert_delta!(ax2[0].u.y, -0.9615239476408233, EPSILON);
		// assert_delta!(ax2[0].d, -1.016468173220299, EPSILON);
		assert_eq!(ax3b.len(), 1);
		assert_delta!(ax3b[0].u.x, 0.0, EPSILON);
		assert_delta!(ax3b[0].u.y, 1.0, EPSILON);
		assert_delta!(ax3b[0].d, 0.75, EPSILON);
		// println!("axiom 4 {:?}", ax4);
		// println!("axiom 5 {:?}", ax5);
		// println!("axiom 6 {:?}", ax6);
		// println!("axiom 7 {:?}", ax7);
	}

	#[test]
	fn axiom1 () {
		let unit_square: Rect = make_square();
		let _res0 = axioms::axiom1(
			Vector { x: 2.0/3.0, y: 1.0/3.0 },
			Vector { x: 1.0/3.0, y: 2.0/3.0 },
			unit_square);
		let _res1 = axioms::axiom1(
			Vector { x: 2.0/3.0, y: 1.0/3.0 },
			Vector { x: 1.0/3.0, y: 2.0/3.0 },
			unit_square);
		// let expected = {
		// 	vector: [-Math.SQRT1_2, Math.SQRT1_2],
		// 	origin: [2/3, 1/3],
		// };
		// expect(ear.math.equivalent(res0.vector, expected.vector)).toBe(true);
		// expect(ear.math.equivalent(res0.origin, expected.origin)).toBe(true);
		// expect(ear.math.equivalent(res1.vector, expected.vector)).toBe(true);
		// expect(ear.math.equivalent(res1.origin, expected.origin)).toBe(true);
	}

	fn make_line (vector: &Vector, origin: &Vector) -> Line {
		// let mag = vector.magnitude();
		let u = vector.normalize().rotate90();
		let d = origin.dot(u); // / mag;
		return if d < 0.0
			{ Line { u: Vector { x: -u.x, y: -u.y }, d: -d } } else
			{ Line { u: Vector { x:  u.x, y:  u.y }, d: d } };
	}

	#[test]
	fn axiom5 () {
		let unit_square: Rect = make_square();

		let line1 = make_line(&Vector { x: 1.0, y: 1.0 }, &Vector { x: 0.0, y: 0.0 });
		let point1 = Vector { x: 0.1, y: 0.0 };
		let point2 = Vector { x: 0.9, y: 0.1 };
		let _res = axioms::axiom5(point1, point2, line1, unit_square);

		// println!("axiom 5 #line input {:?}", line1);
		// println!("axiom 5 #res({}): {:?}", res.len(), res);

		// let lines = [{
		// 	origin: [0.14644660940672627, 0.8535533905932738],
		// 	vector: [0.9855985596534889, -0.16910197872576277],
		// },
		// {
		// 	origin: [0.8535533905932738, 0.14644660940672635],
		// 	vector: [0.16910197872576288, -0.9855985596534887],
		// },
		// {
		// 	origin: [0.4999999999999999, 0.4999999999999999],
		// 	vector: [0.7071067811865475, -0.7071067811865475],
		// }];
		// for (let i = 0; i < lines.length; i += 1) {
		// 	expect(res[i].vector[0]).toBeCloseTo(lines[i].vector[0]);
		// 	expect(res[i].vector[1]).toBeCloseTo(lines[i].vector[1]);
		// 	expect(res[i].origin[0]).toBeCloseTo(lines[i].origin[0]);
		// 	expect(res[i].origin[1]).toBeCloseTo(lines[i].origin[1]);
		// }
	}

	#[test]
	fn axiom6 () {
		let unit_square: Rect = make_square();

		let line1 = make_line(&Vector { x: 0.0, y: 1.0 }, &Vector { x: 1.0, y: 0.0 });
		let line2 = make_line(&Vector { x: 1.0, y: 0.0 }, &Vector { x: 0.0, y: 1.0 });
		let point1 = Vector { x: 0.75, y: 0.0 };
		let point2 = Vector { x: 0.0, y: 0.75 };
		let _res = axioms::axiom6(point1, point2, line1, line2, unit_square);

		// println!("axiom 6 #res({}): {:?}", res.len(), res);

		// let lines = [{
		// 	origin: [0.14644660940672627, 0.8535533905932738],
		// 	vector: [0.9855985596534889, -0.16910197872576277],
		// },
		// {
		// 	origin: [0.8535533905932738, 0.14644660940672635],
		// 	vector: [0.16910197872576288, -0.9855985596534887],
		// },
		// {
		// 	origin: [0.4999999999999999, 0.4999999999999999],
		// 	vector: [0.7071067811865475, -0.7071067811865475],
		// }];
		// for (let i = 0; i < lines.length; i += 1) {
		// 	expect(res[i].vector[0]).toBeCloseTo(lines[i].vector[0]);
		// 	expect(res[i].vector[1]).toBeCloseTo(lines[i].vector[1]);
		// 	expect(res[i].origin[0]).toBeCloseTo(lines[i].origin[0]);
		// 	expect(res[i].origin[1]).toBeCloseTo(lines[i].origin[1]);
		// }

	}

}
