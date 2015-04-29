#![feature(plugin)]
#![plugin(extra_lints)]

#[deny(eq_op)]
fn main() {
	(-(2) < -(2));  //~ERROR
	1 != (1); //~ERROR
	(1 + 2) == 1 + 2; //~ERROR
}
