extern crate libc;

//#[link(name="simd", kind="static")]
extern "C" {
	fn mult_squares(a: &mut super::Square, b: super::Square);
}

pub fn multiply_squares(a: &mut super::Square, b: super::Square) {
	unsafe { mult_squares(a,b) }
}