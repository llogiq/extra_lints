#![feature(plugin, box_syntax)]
#![plugin(eq_op)]

#[allow(unused_allocation)]
#[deny(eq_op)]
fn main() {
	box 1 == box 1 //~ERROR
		|| (1) < (1); //~ERROR
}
