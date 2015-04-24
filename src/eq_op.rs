
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
            if left == right && is_cmp_or_bit(op) {
                cx.span_lint(EQ_OP, e.span, &format!("equal expressions as operands to {}", fmt_op(op)));
            }
        }
    }
}

fn fmt_op(op : &ast::BinOp) -> &str {
	match op.node {
		ast::BiEq => "==",
        ast::BiLt => "<",
        ast::BiLe => "<=",
        ast::BiGt => ">",
        ast::BiGe => ">=",
        ast::BiNe => "!=",
        ast::BiAnd => "&&",
		ast::BiOr => "||",
		ast::BiBitXor => "^",
		ast::BiBitAnd => "&",
		ast::BiBitOr => "|",
        _ => "?"
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
