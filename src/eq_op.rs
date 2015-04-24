
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
use syntax::ptr as ptr;

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
		(&ast::ExprBinary(lop, ref ll, ref lr), &ast::ExprBinary(rop, ref rl, ref rr)) => lop.node == rop.node && is_node_equal(ll, rl) && is_node_equal(lr, rr),
		(&ast::ExprBox(Option::None, ref lboxed), &ast::ExprBox(Option::None, ref rboxed)) => is_node_equal(lboxed, rboxed),
		(&ast::ExprBox(Option::Some(ref lpl), ref lboxedpl), &ast::ExprBox(Option::Some(ref rpl), ref rboxedpl)) => is_node_equal(lpl, rpl) && is_node_equal(lboxedpl, rboxedpl),
		(&ast::ExprCall(ref lcallee, ref largs), &ast::ExprCall(ref rcallee, ref rargs)) => is_node_equal(lcallee, rcallee) && is_node_vec_equal(largs, rargs),
		(&ast::ExprCast(ref lcast, ref lty), &ast::ExprCast(ref rcast, ref rty)) => lty.node == rty.node && is_node_equal(lcast, rcast), // does not seem to work!?
		(&ast::ExprLit(ref llit), &ast::ExprLit(ref rlit)) => llit.node == rlit.node,
		(&ast::ExprMethodCall(ref lident, ref lcty, ref lmargs), &ast::ExprMethodCall(ref rident, ref rcty, ref rmargs)) => lident.node == rident.node && is_ty_vec_equal(lcty, rcty) && is_node_vec_equal(lmargs, rmargs),
		(&ast::ExprParen(ref lparen), &ast::ExprParen(ref rparen)) => is_node_equal(lparen, rparen),
		(&ast::ExprTup(ref ltup), &ast::ExprTup(ref rtup)) => is_node_vec_equal(ltup, rtup),
		(&ast::ExprUnary(lunop, ref lparam), &ast::ExprUnary(runop, ref rparam)) => lunop == runop && is_node_equal(lparam, rparam), 
		(&ast::ExprVec(ref lvec), &ast::ExprVec(ref rvec)) => is_node_vec_equal(lvec, rvec),
		_ => false
	}
}

fn is_ty_vec_equal(left : &Vec<ptr::P<ast::Ty>>, right : &Vec<ptr::P<ast::Ty>>) -> bool {
	left.len() == right.len() && left.iter().zip(right.iter()).all(|(l, r)| l.node == r.node)
}

fn is_node_vec_equal(left : &Vec<ptr::P<ast::Expr>>, right : &Vec<ptr::P<ast::Expr>>) -> bool {
	(left.len() == right.len()) && left.iter().zip(right.iter()).all(|(l, r)| is_node_equal(l, r))
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
