// this source includes a cubic solver
// ported from Robert Lang's Reference Finder
// http://langorigami.com
use math::Vector;
use math::Line;
use math::Rect;
// todo, many of these tests assume that the boundary is a convex polygon

const EPSILON: f64 = 1.0e-8;

// for testing axiom 1:
// (maybe) make sure the paper connects continuously between the two points
// (at least) make sure the points are contained in the paper, which will be
//   satisfied by the first test
pub fn axiom1 (a: Vector, b: Vector, boundary: Rect) -> Vec<Line> {
	if !boundary.contains(a) || !boundary.contains(b) { return vec![] }
	let u: Vector = b.subtract(a).rotate90().normalize();
	let d: f64 = a.add(b).dot(u) / 2.0;
	return vec![Line { u: u, d: d }];
}

// for testing axiom 2:
// make sure that the two points are inside the boundary
pub fn axiom2 (a: Vector, b: Vector, boundary: Rect) -> Vec<Line> {
	if !boundary.contains(a) || !boundary.contains(b) { return vec![] }
	let u: Vector = b.subtract(a).normalize();
	let d: f64 = a.add(b).dot(u) / 2.0;
	return vec![Line { u: u, d: d }];
}

// for testing axiom 3:
// 1. chop the input parameter lines into segments inside the boundary
// 2. for each solution (1 or 2), make solution a reflection line
// 3. reflect one input paramter (should be on top of the other)
//    and test for any point to be inside the other segment.
pub fn axiom3 (a: Line, b: Line, boundary: Rect) -> Vec<Line> {
	let seg_a = boundary.clip(a);
	let seg_b = boundary.clip(b);
	if !seg_a.0 || !seg_b.0 { println!("AX3 this should not happen"); return vec![]; }
	// get intersection and a test if they are parallel
	let intersect = a.intersect(b);
	// if lines are parallel only one solution exists
	if !intersect.0 {
		// special case, because this is a square, we don't need to test any further
		return vec![ Line { u: a.u, d: (a.d + b.d * a.u.dot(b.u)) / 2.0 } ];
	}
	// 2 solutions
	let u1 = a.u.add(b.u).normalize();
	let u2 = a.u.subtract(b.u).normalize();
	let d1 = intersect.1.dot(u1);
	let d2 = intersect.1.dot(u2);
	let solutions: Vec<Line> = vec![ Line { u: u1, d: d1 }, Line { u: u2, d: d2 } ];
	// are the solutions inside the page
	let inside_test: Vec<bool> = solutions.iter()
		.map(|line| boundary.clip(*line).0)
		// .map(|seg| true) // testing: ignore this check
		.collect();
	// seg_a will be the only one reflected
	let reflect_test: Vec<bool> = solutions.iter()
		.map(|l| l.reflect_segment(seg_a.1))
		.map(|seg| seg.quick_overlap(seg_b.1))
		// .map(|seg| true) // testing: ignore this check
		.collect();
	return solutions.iter().enumerate()
		.filter(|(i, _line)| inside_test[*i] && reflect_test[*i])
		.map(|(_i, line)| *line)
		.collect();
}
// for testing axiom 4:
// check the intersection point
pub fn axiom4 (a: Vector, b: Line, boundary: Rect) -> Vec<Line> {
	let u = b.u.rotate90();
	let d = a.dot(u);
	let solution = Line {u, d};
	// test the line before we return it
	// shortest distance between the input point and the input line
	let dist = b.d - a.dot(b.u);
	// dist as a vector, from the point to the line
	let vector = u.scale(dist);
	let point = a.add(vector);
	// 1. the point along the paralle line must be visible
	// 2. prevent lines external and collinear to boundary point
	let test1 = boundary.contains(point);
	// todo: I suspect there might be a simpler way to check this
	//   without calling clip.
	let (test2, _segment) = boundary.clip(solution);
	return if test1 && test2 { vec![solution] } else { vec![] }
}

// p1 is the point the line will pass through (does not move)
// p2 is the point that will fold onto the line (moves)
pub fn axiom5 (p1: Vector, p2: Vector, l: Line, boundary: Rect) -> Vec<Line> {
	let p1base = p1.dot(l.u);
	let a = l.d - p1base;
	let c = p1.distance_to(p2);
	if a > c { return vec![] }
	let b = (c * c - a * a).sqrt();
	let a_vec = l.u.scale(a);
	let base_center = p1.add(a_vec);
	let base_vector = l.u.rotate90().scale(b);
	// if b is near 0 we have one solution, otherwise two
	let mirrors: Vec<Vector> = if b < EPSILON { vec![base_center] }
		else { vec![
			base_center.add(base_vector),
			base_center.subtract(base_vector)
		]};
	// for each construction to be valid its mirror point must be in the boundary
	return mirrors.iter()
		.filter(|vec| boundary.contains(**vec))
		.map(|vec| p2.subtract(*vec).normalize())
		.map(|u| Line { u, d: p1.dot(u) })
		.collect::<Vec<Line>>();
}

// cube root preserve sign
fn cubrt (n: f64) -> f64 {
	if n < 0.0 { -(-n).powf(1.0/3.0) } else { n.powf(1.0/3.0) }
}

// Robert Lang's cubic solver from Reference Finder
fn polynomial (degree: u8, a: f64, b: f64, c: f64, d: f64) -> Vec<f64> {
	// linear
	if degree == 1 { return vec![-d / c]; }
	else if degree == 2 {
		// quadratic
		let discriminant = c.powf(2.0) - (4.0 * b * d);
		// no solution
		if discriminant < -EPSILON { return vec![]; }
		// one solution
		let q1 = -c / (2.0 * b);
		if discriminant < EPSILON {
			return vec![q1];
		}
		// two solutions
		let q2 = discriminant.sqrt() / (2.0 * b);
		return vec![q1 + q2, q1 - q2];
	} else if degree == 3 {
		// cubic
		// Cardano's formula. convert to depressed cubic
		let a2 = b / a;
		let a1 = c / a;
		let a0 = d / a;
		let q = (3.0 * a1 - a2.powf(2.0)) / 9.0;
		let r = (9.0 * a2 * a1 - 27.0 * a0 - 2.0 * a2.powf(3.0)) / 54.0;
		let d0 = q.powf(3.0) + r.powf(2.0);
		let u = -a2 / 3.0;
		// one solution
		if d0 > 0.0 {
			let sqrt_d0 = d0.sqrt();
			let s = cubrt(r + sqrt_d0);
			let t = cubrt(r - sqrt_d0);
			return vec![u + s + t];
		}
		// two solutions
		if d0.abs() < EPSILON {
			let s = r.powf(1.0/3.0);
			// let S = cubrt(R);
			// instead of checking if S is NaN, check if R was negative
			// if (isNaN(S)) { break; }
			if r < 0.0 { return vec![]; }
			return vec![u + 2.0 * s, u - s];
		}
		// three solutions
		let sqrt_d0 = (-d0).sqrt();
		let phi = sqrt_d0.atan2(r) / 3.0;
		let r_s = (r.powf(2.0) - d0).powf(1.0/6.0);
		let s_r = r_s * phi.cos();
		let s_i = r_s * phi.sin();
		return vec![
			u + 2.0 * s_r,
			u - s_r - 3.0_f64.sqrt() * s_i,
			u - s_r + 3.0_f64.sqrt() * s_i
		];
	}
	return vec![];
}

pub fn axiom6 (
	p1: Vector,
	p2: Vector,
	l1: Line,
	l2: Line,
	boundary: Rect
) -> Vec<Line> {
	// at least pointA must not be on lineA
	// for some reason this epsilon is much higher than 1e-6
	if (1.0 - (l1.u.dot(p1) / l1.d)).abs() < 0.02 { return vec![]; }
	// line vec is the first line's vector, along the line, not the normal
	let line_vec = l1.u.rotate90();
	let vec1 = p1.add(l1.u.scale(l1.d)).subtract(p2.scale(2.0));
	let vec2 = l1.u.scale(l1.d).subtract(p1);
	let c1 = p2.dot(l2.u) - l2.d;
	let c2 = 2.0 * vec2.dot(line_vec);
	let c3 = vec2.dot(vec2);
	let c4 = vec1.add(vec2).dot(line_vec);
	let c5 = vec1.dot(vec2);
	let c6 = line_vec.dot(l2.u);
	let c7 = vec2.dot(l2.u);
	let a = c6;
	let b = c1 + c4 * c6 + c7;
	let c = c1 * c2 + c5 * c6 + c4 * c7;
	let d = c1 * c3 + c5 * c7;
	// construct the solution from the root, the solution being the parameter
	// point reflected across the fold line, lying on the parameter line
	let mut polynomial_degree: u8 = 0;
	if c.abs() > EPSILON { polynomial_degree = 1; }
	if b.abs() > EPSILON { polynomial_degree = 2; }
	if a.abs() > EPSILON { polynomial_degree = 3; }
	let roots = polynomial(polynomial_degree, a, b, c, d);
	let mirrors1: Vec<Vector> = roots.iter()
		.map(|n| l1.u.scale(l1.d).add(line_vec.scale(*n)))
		.collect();
	// this tuple temporarily stores (the point, the line's u vector)
	let solutions: Vec<Line> = mirrors1.iter()
		.map(|p| (p, p.subtract(p1).normalize()))
		.map(|el| Line { u: el.1, d: el.1.dot(el.0.midpoint(p1)) })
		.collect();
	let mirrors2: Vec<Vector> = solutions.iter()
		.map(|l| p2.add(l.u.scale(2.0 * (l.d - p2.dot(l.u)))))
		.collect();
	let mut lines: Vec<Line> = vec![];
	for i in 0..solutions.len() {
		if boundary.contains(mirrors1[i])
		&& boundary.contains(mirrors2[i]) {
			lines.push(solutions[i]);
		}
	}
	// this style: need to implement FromIterator for Vec<Line>
	// return solutions.iter().enumerate()
	// 	.filter(|(i, el)| boundary.contains(mirrors1[*i])
	// 		&& boundary.contains(mirrors2[*i]))
	// 	.map(|(_, el)| el)
	// 	.collect::<Vec<Line>>();
	return lines;
}

// l1 is the perpendicular to our solution
// l2 is the line we bring the point onto
pub fn axiom7 (p: Vector, l1: Line, l2: Line, boundary: Rect) -> Vec<Line> {
	let u = l1.u.rotate90();
	let u_u = u.dot(l2.u);
	// if u_u is close to 0, the two input lines are parallel, no solution
	if u_u.abs() < EPSILON { return vec![] }
	let a = p.dot(u);
	let b = p.dot(l2.u);
	let d = (l2.d + 2.0 * a * u_u - b) / (2.0 * u_u);
	// test if construction is valid inside the boundary
	let solution = Line {u, d};
	let intersect = solution.intersect(l1);
	let reflection = solution.reflect_vector(p);
	// the reflected point should be inside the boundary
	// todo: simplify this next line using variables above
	let test1 = boundary.contains(reflection);
	// if this intersection isn't inside, the line can't be folded onto itself
	let test2 = intersect.0 && boundary.contains(intersect.1);
	// mirror should not be the intersection point itself
	let test3 = !reflection.equivalent(intersect.1);
	return if test1 && test2 && test3 { vec![solution] } else { vec![] };
}
