
#![feature(plugin)]
#![feature(box_syntax)]
#![plugin(eq_op)]

fn id(x : bool) -> bool {
	x
}

#[test]
fn test() {
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
}
