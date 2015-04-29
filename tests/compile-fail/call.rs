#![feature(plugin)]
#![plugin(extra_lints)]


fn id(x : bool) -> bool {
	x
}

#[deny(eq_op)]
fn main() {
	id(true) && id(true); //~ERROR
}
