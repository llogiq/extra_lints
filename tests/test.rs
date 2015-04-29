#![feature(plugin)]
#![plugin(extra_lints)]

use std::cmp::{min,max};

#[deny(eq_op)]
#[test]
fn test_eq_op() {
	assert!(false != true);
	let x = 2;
	assert!(x - 1 == 2 - 1); // we don't track state
	assert!((1 + 2) + 3 == 1 + (2 + 3)); // we don't match associative exprs yet
	1 + 2 == 2 + 1;
	min(1, 2) == min(2, 1);
}

#[test]
fn test_bit_masks() {
	let x = 5;
	x & 1 == 1;
	x & 1 > 0;
}
