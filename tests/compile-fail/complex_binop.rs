#![feature(plugin)]
#![plugin(eq_op)]

#[deny(eq_op)]
fn main() {
	((1 + 1) & (1 + 1) == (1 + 1) & (1 + 1));
	//~^ ERROR
					//~^^ ERROR
						//~^^^ ERROR
	(1 + 2) * 3 - 5 < 3 * (2 + 1) - 5; //~ERROR
}
