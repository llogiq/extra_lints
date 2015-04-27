#![feature(plugin)]
#![plugin(eq_op)]

#[deny(eq_op)]
fn main() {
	!([1] != [1]); //~ERROR
	!((1, 2) != (1, 2)); //~ERROR
	[1].len() == [1].len(); //~ERROR
	vec![1, 2, 3] == vec![1, 2, 3]; //no error yet, as we don't match macros yet
}
