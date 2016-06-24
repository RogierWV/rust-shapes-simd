mod shapes;

use shapes::Shape;
use shapes::circle::Circle;
use shapes::square::Square;

fn main() {
	let mut c = Circle{ id: 0u64, x: 10f64, y: 100f64, radius: std::f64::consts::PI };
	println!("c.area() = {}", c.area());
	let mut s = Square{ id: 1u64, x0:10f64, x1: 20f64, y0: 0f64, y1: 10f64};
	println!("s.area() = {}", s.area());
	let c1 = Circle{ id: 2u64, x: 10f64, y: 100f64, radius: std::f64::consts::PI };
	c.mult(c1);
	println!("c.mult(c1) == {{ {} {} {} {} }}", c.id, c.x, c.y, c.radius);
	let s1 = Square{ id: 3u64, x0:10f64, x1: 20f64, y0: 0f64, y1: 10f64};
	s.mult(s1);
	println!("s.mult(s1) == {:?}", s);
}
