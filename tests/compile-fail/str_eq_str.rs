#![feature(plugin)]
#![feature(box_syntax)]
#![plugin(eq_op)]

#[deny(eq_op)]
fn main() {
	"1" == "1"; //~ERROR
}
