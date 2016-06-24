//use simd::multiply_squares;
mod simd;

#[repr(C)]
#[derive(Debug,PartialEq)]
pub struct Square {
	pub id: u64,
	pub x0: f64,
	pub x1: f64,
	pub y0: f64,
	pub y1: f64
}

impl Square {
	pub fn mult(&mut self, other:Square) {
		simd::multiply_squares(self, other);
	}
}

impl super::Shape for Square {
	fn area(&self) -> f64 {
	    ((self.x1-self.x0)*(self.y1-self.y0))
	}

	fn location(&self) -> (f64,f64) {
		((self.x1-self.x0),(self.y1-self.y0))
	}
}

impl Drop for Square {
	fn drop(&mut self) {
		println!("Dropped square {}", self.id);
	}
}