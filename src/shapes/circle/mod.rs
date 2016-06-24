pub struct Circle {
	pub id: u64,
	pub x: f64,
	pub y: f64,
	pub radius: f64
}

impl Circle {
	pub fn mult(&mut self, other: Circle) {
		self.x = self.x * other.x;
		self.y = self.y * other.y;
		self.radius = self.radius * other.radius;
	}
}

impl super::Shape for Circle {
	fn area(&self) -> f64 {
	    ::std::f64::consts::PI * (self.radius * self.radius)
	}

	fn location(&self) -> (f64,f64) {
		(self.x,self.y)
	}
}

impl Drop for Circle {
	fn drop(&mut self) {
		println!("Dropped circle {:?}", self.id);
	}
}