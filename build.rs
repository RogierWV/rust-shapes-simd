extern crate gcc;

fn main() {
	gcc::Config::new()
		.file("src/shapes/square/simd/c/square_simd_funcs.c")
		.compile("libsimd.a");
}
