#![feature(plugin)]
#![plugin(extra_lints)]

#[deny(eq_op)]
fn main() {
	1 == 1; //~ERROR
}
