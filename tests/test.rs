
#![feature(plugin)]
#![plugin(eq_op)]

#[deny(eq_op)]
#[test]
fn test_simple() {
	if 1 == 1 { print!("No"); }
}
