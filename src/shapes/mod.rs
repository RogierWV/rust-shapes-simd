pub mod circle;
pub mod square;

pub trait Shape {
	fn area(&self) -> f64;
	fn location(&self) -> (f64,f64);
	//fn mult(&mut self, Shape);
}