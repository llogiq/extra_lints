#![feature(plugin)]
#![plugin(eq_cmp)]
#[test]
fn test_simple() {
	if 1 == 1 { print!("No"); }
}
