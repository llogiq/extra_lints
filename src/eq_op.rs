
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
use syntax::codemap as code;

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
        if let ast::ExprBinary(ref op, ref left, ref right) = e.node {
            if is_cmp_or_bit(op) && is_exp_equal(left, right) {
                cx.span_lint(EQ_OP, e.span, &format!("equal expressions as operands to {}", ast_util::binop_to_string(op.node)));
            }
        }
    }
}

fn is_exp_equal(left : &ast::Expr, right : &ast::Expr) -> bool {
	match (&left.node, &right.node) {
		(&ast::ExprBinary(lop, ref ll, ref lr), &ast::ExprBinary(rop, ref rl, ref rr)) => lop.node == rop.node && is_exp_equal(ll, rl) && is_exp_equal(lr, rr),
		(&ast::ExprBox(Option::None, ref lboxed), &ast::ExprBox(Option::None, ref rboxed)) => is_exp_equal(lboxed, rboxed),
		(&ast::ExprBox(Option::Some(ref lpl), ref lboxedpl), &ast::ExprBox(Option::Some(ref rpl), ref rboxedpl)) => is_exp_equal(lpl, rpl) && is_exp_equal(lboxedpl, rboxedpl),
		(&ast::ExprCall(ref lcallee, ref largs), &ast::ExprCall(ref rcallee, ref rargs)) => is_exp_equal(lcallee, rcallee) && is_exp_vec_equal(largs, rargs),
		(&ast::ExprCast(ref lcast, ref lty), &ast::ExprCast(ref rcast, ref rty)) => is_ty_equal(lty, rty) && is_exp_equal(lcast, rcast),
		(&ast::ExprField(ref lfexp, ref lfident), &ast::ExprField(ref rfexp, ref rfident)) => lfident.node == rfident.node && is_exp_equal(lfexp, rfexp),
		(&ast::ExprLit(ref llit), &ast::ExprLit(ref rlit)) => llit.node == rlit.node,
		(&ast::ExprMethodCall(ref lident, ref lcty, ref lmargs), &ast::ExprMethodCall(ref rident, ref rcty, ref rmargs)) => 
			lident.node == rident.node && is_ty_vec_equal(lcty, rcty) && is_exp_vec_equal(lmargs, rmargs),
		(&ast::ExprParen(ref lparen), &ast::ExprParen(ref rparen)) => is_exp_equal(lparen, rparen),
		(&ast::ExprPath(Option::None, ref lpath), &ast::ExprPath(Option::None, ref rpath)) => is_path_equal(lpath, rpath),
		(&ast::ExprPath(Option::Some(ref lqself), ref lsubpath), &ast::ExprPath(Option::Some(ref rqself), ref rsubpath)) => 
			is_qself_equal(lqself, rqself) && is_path_equal(lsubpath, rsubpath),		
		(&ast::ExprTup(ref ltup), &ast::ExprTup(ref rtup)) => is_exp_vec_equal(ltup, rtup),
		(&ast::ExprUnary(lunop, ref lparam), &ast::ExprUnary(runop, ref rparam)) => lunop == runop && is_exp_equal(lparam, rparam), 
		(&ast::ExprVec(ref lvec), &ast::ExprVec(ref rvec)) => is_exp_vec_equal(lvec, rvec),
		_ => false
	}
}

fn is_path_equal(left : &ast::Path, right : &ast::Path) -> bool {
	left.global == right.global && left.segments == right.segments
}

fn is_qself_equal(left : &ast::QSelf, right : &ast::QSelf) -> bool {
	left.ty.node == right.ty.node && left.position == right.position
}

fn is_ty_equal(left : &ast::Ty, right : &ast::Ty) -> bool {
	match (&left.node, &right.node) {
	(&ast::TyVec(ref lvec), &ast::TyVec(ref rvec)) => is_ty_equal(lvec, rvec),
	(&ast::TyFixedLengthVec(ref lfvty, ref lfvexp), &ast::TyFixedLengthVec(ref rfvty, ref rfvexp)) => is_ty_equal(lfvty, rfvty) && is_exp_equal(lfvexp, rfvexp),
	(&ast::TyPtr(ref lmut), &ast::TyPtr(ref rmut)) => is_mut_ty_equal(lmut, rmut),
	(&ast::TyRptr(Option::None, ref lrmut), &ast::TyRptr(Option::None, ref rrmut)) => is_mut_ty_equal(lrmut, rrmut),
	(&ast::TyBareFn(ref lbare), &ast::TyBareFn(ref rbare)) => is_bare_fn_ty_equal(lbare, rbare),
    (&ast::TyTup(ref ltup), &ast::TyTup(ref rtup)) => is_ty_vec_equal(ltup, rtup),
	(&ast::TyPath(Option::None, ref lpath), &ast::TyPath(Option::None, ref rpath)) => is_path_equal(lpath, rpath),
	(&ast::TyPath(Option::Some(ref lqself), ref lsubpath), &ast::TyPath(Option::Some(ref rqself), ref rsubpath)) =>
		is_qself_equal(lqself, rqself) && is_path_equal(lsubpath, rsubpath),
    (&ast::TyObjectSum(ref lsumty, ref lobounds), &ast::TyObjectSum(ref rsumty, ref robounds)) => 
		is_ty_equal(lsumty, rsumty) && is_param_bounds_equal(lobounds, robounds),
	(&ast::TyPolyTraitRef(ref ltbounds), &ast::TyPolyTraitRef(ref rtbounds)) => is_param_bounds_equal(ltbounds, rtbounds),
    (&ast::TyParen(ref lty), &ast::TyParen(ref rty)) => is_ty_equal(lty, rty),
    (&ast::TyTypeof(ref lof), &ast::TyTypeof(ref rof)) => is_exp_equal(lof, rof),
	(&ast::TyInfer, &ast::TyInfer) => true,
	_ => false
	}
}

fn is_param_bound_equal(left : &ast::TyParamBound, right : &ast::TyParamBound) -> bool {
	match(left, right) {
	(&ast::TraitTyParamBound(ref lpoly, ref lmod), &ast::TraitTyParamBound(ref rpoly, ref rmod)) => lmod == rmod && is_poly_traitref_equal(lpoly, rpoly),
    (&ast::RegionTyParamBound(ref ltime), &ast::RegionTyParamBound(ref rtime)) => is_lifetime_equal(ltime, rtime),
    _ => false
	}
}

fn is_poly_traitref_equal(left : &ast::PolyTraitRef, right : &ast::PolyTraitRef) -> bool {
	is_lifetimedef_vec_equal(&left.bound_lifetimes, &right.bound_lifetimes) && is_path_equal(&left.trait_ref.path, &right.trait_ref.path)
}

fn is_param_bounds_equal(left : &ast::TyParamBounds, right : &ast::TyParamBounds) -> bool {
	left.len() == right.len() && left.iter().zip(right.iter()).all(|(l, r)| is_param_bound_equal(l, r))
}

fn is_mut_ty_equal(left : &ast::MutTy, right : &ast::MutTy) -> bool {
	left.mutbl == right.mutbl && is_ty_equal(&left.ty, &right.ty)
}

fn is_bare_fn_ty_equal(left : &ast::BareFnTy, right : &ast::BareFnTy) -> bool {
	left.unsafety == right.unsafety && left.abi == right.abi && is_lifetimedef_vec_equal(&left.lifetimes, &right.lifetimes) && is_fndecl_equal(&left.decl, &right.decl)
} 

fn is_fndecl_equal(left : &ptr::P<ast::FnDecl>, right : &ptr::P<ast::FnDecl>) -> bool {
	left.variadic == right.variadic && is_arg_vec_equal(&left.inputs, &right.inputs) && is_fnret_ty_equal(&left.output, &right.output)
}

fn is_fnret_ty_equal(left : &ast::FunctionRetTy, right : &ast::FunctionRetTy) -> bool {
	match (left, right) {
	(&ast::NoReturn(_), &ast::NoReturn(_)) | (&ast::DefaultReturn(_), &ast::DefaultReturn(_)) => true,
	(&ast::Return(ref lty), &ast::Return(ref rty)) => is_ty_equal(lty, rty),
	_ => false	
	}
}

fn is_arg_equal(left : &ast::Arg, right : &ast::Arg) -> bool {
	is_ty_equal(&left.ty, &right.ty) && is_pat_equal(&left.pat, &right.pat)
}

fn is_arg_vec_equal(left : &Vec<ast::Arg>, right : &Vec<ast::Arg>) -> bool {
	left.len() == right.len() && left.iter().zip(right.iter()).all(|(l, r)| is_arg_equal(l, r))
}

fn is_pat_equal(left : &ast::Pat, right : &ast::Pat) -> bool {
	match(&left.node, &right.node) {
	(&ast::PatWild(lwild), &ast::PatWild(rwild)) => lwild == rwild,
	(&ast::PatIdent(ref lmode, ref lident, Option::None), &ast::PatIdent(ref rmode, ref rident, Option::None)) =>
		lmode == rmode && is_ident_equal(&lident.node, &rident.node),
	(&ast::PatIdent(ref lmode, ref lident, Option::Some(ref lpat)), &ast::PatIdent(ref rmode, ref rident, Option::Some(ref rpat))) =>
		lmode == rmode && is_ident_equal(&lident.node, &rident.node) && is_pat_equal(lpat, rpat),
    (&ast::PatEnum(ref lpath, Option::None), &ast::PatEnum(ref rpath, Option::None)) => is_path_equal(lpath, rpath),
    (&ast::PatEnum(ref lpath, Option::Some(ref lenum)), &ast::PatEnum(ref rpath, Option::Some(ref renum))) => 
		is_path_equal(lpath, rpath) && is_pat_vec_equal(lenum, renum),  
    (&ast::PatStruct(ref lpath, ref lfieldpat, lbool), &ast::PatStruct(ref rpath, ref rfieldpat, rbool)) =>
		lbool == rbool && is_path_equal(lpath, rpath) && is_spanned_fieldpat_vec_equal(lfieldpat, rfieldpat),
    (&ast::PatTup(ref ltup), &ast::PatTup(ref rtup)) => is_pat_vec_equal(ltup, rtup), 
    (&ast::PatBox(ref lboxed), &ast::PatBox(ref rboxed)) => is_pat_equal(lboxed, rboxed),
    (&ast::PatRegion(ref lpat, ref lmut), &ast::PatRegion(ref rpat, ref rmut)) => is_pat_equal(lpat, rpat) && lmut == rmut,
	(&ast::PatLit(ref llit), &ast::PatLit(ref rlit)) => is_exp_equal(llit, rlit),
    (&ast::PatRange(ref lfrom, ref lto), &ast::PatRange(ref rfrom, ref rto)) =>
		is_exp_equal(lfrom, rfrom) && is_exp_equal(lto, rto),
    (&ast::PatVec(ref lfirst, Option::None, ref llast), &ast::PatVec(ref rfirst, Option::None, ref rlast)) =>
		is_pat_vec_equal(lfirst, rfirst) && is_pat_vec_equal(llast, rlast),
    (&ast::PatVec(ref lfirst, Option::Some(ref lpat), ref llast), &ast::PatVec(ref rfirst, Option::Some(ref rpat), ref rlast)) =>
		is_pat_vec_equal(lfirst, rfirst) && is_pat_equal(lpat, rpat) && is_pat_vec_equal(llast, rlast),
	// I don't match macros for now, the code is slow enough as is ;-)
	_ => false
	}
}

fn is_spanned_fieldpat_vec_equal(left : &Vec<code::Spanned<ast::FieldPat>>, right : &Vec<code::Spanned<ast::FieldPat>>) -> bool {
	left.len() == right.len() && left.iter().zip(right.iter()).all(|(l, r)| is_fieldpat_equal(&l.node, &r.node))
}

fn is_fieldpat_equal(left : &ast::FieldPat, right : &ast::FieldPat) -> bool {
	left.is_shorthand == right.is_shorthand && is_ident_equal(&left.ident, &right.ident) && is_pat_equal(&left.pat, &right.pat) 
}

fn is_ident_equal(left : &ast::Ident, right : &ast::Ident) -> bool {
	&left.name == &right.name && left.ctxt == right.ctxt
}

fn is_pat_vec_equal(left : &Vec<ptr::P<ast::Pat>>, right : &Vec<ptr::P<ast::Pat>>) -> bool {
	left.len() == right.len() && left.iter().zip(right.iter()).all(|(l, r)| is_pat_equal(l, r))	
}

fn is_lifetimedef_equal(left : &ast::LifetimeDef, right : &ast::LifetimeDef) -> bool {
	is_lifetime_equal(&left.lifetime, &right.lifetime) && is_lifetime_vec_equal(&left.bounds, &right.bounds)
}

fn is_lifetimedef_vec_equal(left : &Vec<ast::LifetimeDef>, right : &Vec<ast::LifetimeDef>) -> bool {
	left.len() == right.len() && left.iter().zip(right.iter()).all(|(l, r)| is_lifetimedef_equal(l, r))
}

fn is_lifetime_equal(left : &ast::Lifetime, right : &ast::Lifetime) -> bool {
	left.name == right.name
}

fn is_lifetime_vec_equal(left : &Vec<ast::Lifetime>, right : &Vec<ast::Lifetime>) -> bool {
	left.len() == right.len() && left.iter().zip(right.iter()).all(|(l, r)| is_lifetime_equal(l, r))
}

fn is_ty_vec_equal(left : &Vec<ptr::P<ast::Ty>>, right : &Vec<ptr::P<ast::Ty>>) -> bool {
	left.len() == right.len() && left.iter().zip(right.iter()).all(|(l, r)| is_ty_equal(l, r))
}

fn is_exp_vec_equal(left : &Vec<ptr::P<ast::Expr>>, right : &Vec<ptr::P<ast::Expr>>) -> bool {
	left.len() == right.len() && left.iter().zip(right.iter()).all(|(l, r)| is_exp_equal(l, r))
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
