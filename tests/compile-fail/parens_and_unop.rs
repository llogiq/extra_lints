#![feature(plugin)]
#![plugin(eq_op)]

#[deny(eq_op)]
fn main() {
	(-(2) < -(2));  //~ERROR
	1 != (1); //~ERROR
	(1 + 2) == 2 + 1; //~ERROR
}
