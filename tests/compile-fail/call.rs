#![feature(plugin)]
#![plugin(eq_op)]


fn id(x : bool) -> bool {
	x
}

#[deny(eq_op)]
fn main() {
	id(true) && id(true); //~ERROR
}
