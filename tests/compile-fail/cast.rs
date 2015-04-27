#![feature(plugin)]
#![plugin(eq_op)]

#[deny(eq_op)]
fn main() {
	(10 as f32) > (10 as f32): //~ERROR
}
