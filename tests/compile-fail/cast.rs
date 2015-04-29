#![feature(plugin)]
#![plugin(extra_lints)]

#[deny(eq_op)]
fn main() {
	(10 as f32) > (10 as f32): //~ERROR
}
