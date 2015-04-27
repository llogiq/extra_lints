#![feature(plugin)]
#![plugin(eq_op)]

use std::cmp::{min,max};

#[deny(eq_op)]
#[test]
fn test_eq_op() {
	assert!(false != true);
	let x = 2;
	assert!(x - 1 == 2 - 1); // we don't track state
	assert!((1 + 2) + 3 == 1 + (2 + 3)); // we don't match associative exprs yet
}

#[test]
fn test_max_min() {
	assert!(min(0, max(100, 200)) == 0);
}
