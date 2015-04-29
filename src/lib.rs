#![feature(rustc_private)]
#![feature(staged_api)]
#![feature(plugin)]
#![feature(plugin_registrar)]
#![unstable(feature = "rustc_private")]
#![staged_api]
#![crate_type = "dylib"]

#[macro_use]
extern crate rustc;
#[macro_use]
extern crate syntax;

use rustc::plugin::Registry;

mod eq_op;
//mod min_max;
mod bit_mask;

#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
	reg.register_lint_pass(Box::new(eq_op::EqOp));
//	reg.register_lint_pass(Box::new(min_max::MinMax));
	reg.register_lint_pass(Box::new(bit_mask::BitMask));
	
//	reg.register_lint_group("extra_lint", [eq_op::EQ_OP, bit_mask::BAD_BIT_MASK]);
}
