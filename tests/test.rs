#![feature(plugin)]
#![feature(box_syntax)]
#![plugin(eq_op)]

use std::cmp::{min,max};

fn id(x : bool) -> bool {
	x
}

#[test]
fn test_eq_op() {
	assert!(1 == 1);
	assert!("1" == "1");
	assert!(false == false);
	assert!(box 1 == box 1 || (1) < (1));
	assert!((1 + 1) & (1 + 1) == (1 + 1) & (1 + 1));
	assert!(!(-(2) < -(2)));
	assert!(!((10 as f32) > (10 as f32)));
	assert!(!([1] != [1]));
	assert!(!((1, 2) != (1, 2)));
	assert!([1].len() == [1].len());
	assert!(vec![1, 2, 3] == vec![1, 2, 3]);
	assert!(id(true) && id(true));
	assert!(1 + 2 == 2 + 1);
}

#[test]
fn test_max_min() {
	assert!(min(0, max(100, 200)) == 0);
}
