#include <stdint.h>

typedef struct _ {
	int64_t id;
	double x0,x1,y0,y1;
} Square;

typedef double v4sd __attribute__ ((vector_size (32)));

void mult_squares(Square a, Square b) {
	v4sd a_vec = {a.x0,a.x1,a.y0,a.y1};
	v4sd b_vec = {b.x0,b.x1,b.y0,b.y1};
	a_vec *= b_vec;
	a = (Square) {.x0=a_vec[0], .x1=a_vec[1], .y0=a_vec[2], .y1=a_vec[3]};
}