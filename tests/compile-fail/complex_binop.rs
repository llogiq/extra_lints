#![feature(plugin)]
#![plugin(extra_lints)]

#[deny(eq_op)]
fn main() {
	((1 + 1) & (1 + 1) == (1 + 1) & (1 + 1));
	//~^ ERROR
					//~^^ ERROR
						//~^^^ ERROR
	(1 * 2) + (3 * 4) == 1 * 2 + 3 * 4; //~ERROR
}
