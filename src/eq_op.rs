
#![staged_api]
#![crate_type = "dylib"]
#![crate_type = "rlib"]
#![feature(plugin_registrar)]
#![feature(box_syntax)]
#![feature(rustc_private)]
#![feature(staged_api)]
#![unstable(feature = "rustc_private")]

extern crate syntax;
#[macro_use]
extern crate rustc;

use rustc::plugin::Registry;
use rustc::lint::*;
use syntax::ast as ast;
use syntax::ast_util as ast_util;

declare_lint! {
    EQ_OP,
    Warn,
    "warn about comparing equal expressions (e.g. x == x)"
}

#[derive(Copy,Clone)]
pub struct EqOp;

impl LintPass for EqOp {
    fn get_lints(&self) -> LintArray {
        lint_array!(EQ_OP)
    }
    
    fn check_expr(&mut self, cx: &Context, e: &ast::Expr) {
        // ExprBinary(BinOp, P<Expr>, P<Expr>)
        if let ast::ExprBinary(ref op, ref left, ref right) = e.node {
            if is_cmp_or_bit(op) && is_node_equal(left, right) {
                cx.span_lint(EQ_OP, e.span, &format!("equal expressions as operands to {}", ast_util::binop_to_string(op.node)));
            }
        }
    }
}

fn is_node_equal(left : &ast::Expr, right : &ast::Expr) -> bool {
	match (&left.node, &right.node) {
		(&ast::ExprLit(ref lit_left), &ast::ExprLit(ref lit_right)) => lit_left.node == lit_right.node,
		_ => false
	}
}

fn is_cmp_or_bit(op : &ast::BinOp) -> bool {
    match op.node {
        ast::BiEq | ast::BiLt | ast::BiLe | ast::BiGt | ast::BiGe | ast::BiNe | ast::BiAnd | 
			ast::BiOr | ast::BiBitXor | ast::BiBitAnd | ast::BiBitOr => true,
        _ => false
    }
}

#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
	reg.register_lint_pass(box EqOp as LintPassObject);
}
