use rustc::lint::*;
use syntax::ast::*;
use syntax::ast_util as ast_util;
use syntax::ptr::P;
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
    
    fn check_expr(&mut self, cx: &Context, e: &Expr) {
        if let ExprBinary(ref op, ref left, ref right) = e.node {
            if is_cmp_or_bit(op) && is_exp_equal(left, right) {
                cx.span_lint(EQ_OP, e.span, &format!("equal expressions as operands to {}", ast_util::binop_to_string(op.node)));
            }
        }
    }
}

fn is_exp_equal(left : &Expr, right : &Expr) -> bool {
	match (&left.node, &right.node) {
		(&ExprBinary(ref lop, ref ll, ref lr), &ExprBinary(ref rop, ref rl, ref rr)) => 
			lop.node == rop.node && is_exp_equal(ll, rl) && is_exp_equal(lr, rr),
		(&ExprBox(ref lpl, ref lboxedpl), &ExprBox(ref rpl, ref rboxedpl)) => 
			both(lpl, rpl, |l, r| is_exp_equal(l, r)) && is_exp_equal(lboxedpl, rboxedpl),
		(&ExprCall(ref lcallee, ref largs), &ExprCall(ref rcallee, ref rargs)) => 
			is_exp_equal(lcallee, rcallee) && is_exp_vec_equal(largs, rargs),
		(&ExprCast(ref lcast, ref lty), &ExprCast(ref rcast, ref rty)) => 
			is_ty_equal(lty, rty) && is_exp_equal(lcast, rcast),
		(&ExprField(ref lfexp, ref lfident), &ExprField(ref rfexp, ref rfident)) => 
			lfident.node == rfident.node && is_exp_equal(lfexp, rfexp),
		(&ExprLit(ref llit), &ExprLit(ref rlit)) => llit.node == rlit.node,
		(&ExprMethodCall(ref lident, ref lcty, ref lmargs), &ExprMethodCall(ref rident, ref rcty, ref rmargs)) => 
			lident.node == rident.node && is_ty_vec_equal(lcty, rcty) && is_exp_vec_equal(lmargs, rmargs),
		(&ExprParen(ref lparen), &ExprParen(ref rparen)) => is_exp_equal(lparen, rparen),
		(&ExprParen(ref lparen), _) => is_exp_equal(lparen, right),
		(_, &ExprParen(ref rparen)) => is_exp_equal(left, rparen),
		(&ExprPath(ref lqself, ref lsubpath), &ExprPath(ref rqself, ref rsubpath)) => 
			both(lqself, rqself, |l, r| is_qself_equal(l, r)) && is_path_equal(lsubpath, rsubpath),		
		(&ExprTup(ref ltup), &ExprTup(ref rtup)) => is_exp_vec_equal(ltup, rtup),
		(&ExprUnary(lunop, ref lparam), &ExprUnary(runop, ref rparam)) => lunop == runop && is_exp_equal(lparam, rparam), 
		(&ExprVec(ref lvec), &ExprVec(ref rvec)) => is_exp_vec_equal(lvec, rvec),
		_ => false
	}
}

fn is_exp_vec_equal(left : &Vec<P<Expr>>, right : &Vec<P<Expr>>) -> bool {
	over(left, right, |l, r| is_exp_equal(l, r))
}

fn is_path_equal(left : &Path, right : &Path) -> bool {
	left.global == right.global && left.segments == right.segments
}

fn is_qself_equal(left : &QSelf, right : &QSelf) -> bool {
	left.ty.node == right.ty.node && left.position == right.position
}

fn is_ty_equal(left : &Ty, right : &Ty) -> bool {
	match (&left.node, &right.node) {
	(&TyVec(ref lvec), &TyVec(ref rvec)) => is_ty_equal(lvec, rvec),
	(&TyFixedLengthVec(ref lfvty, ref lfvexp), &TyFixedLengthVec(ref rfvty, ref rfvexp)) => 
		is_ty_equal(lfvty, rfvty) && is_exp_equal(lfvexp, rfvexp),
	(&TyPtr(ref lmut), &TyPtr(ref rmut)) => is_mut_ty_equal(lmut, rmut),
	(&TyRptr(ref ltime, ref lrmut), &TyRptr(ref rtime, ref rrmut)) => 
		both(ltime, rtime, is_lifetime_equal) && is_mut_ty_equal(lrmut, rrmut),
	(&TyBareFn(ref lbare), &TyBareFn(ref rbare)) => is_bare_fn_ty_equal(lbare, rbare),
    (&TyTup(ref ltup), &TyTup(ref rtup)) => is_ty_vec_equal(ltup, rtup),
	(&TyPath(Option::None, ref lpath), &TyPath(Option::None, ref rpath)) => is_path_equal(lpath, rpath),
	(&TyPath(Option::Some(ref lqself), ref lsubpath), &TyPath(Option::Some(ref rqself), ref rsubpath)) =>
		is_qself_equal(lqself, rqself) && is_path_equal(lsubpath, rsubpath),
    (&TyObjectSum(ref lsumty, ref lobounds), &TyObjectSum(ref rsumty, ref robounds)) => 
		is_ty_equal(lsumty, rsumty) && is_param_bounds_equal(lobounds, robounds),
	(&TyPolyTraitRef(ref ltbounds), &TyPolyTraitRef(ref rtbounds)) => is_param_bounds_equal(ltbounds, rtbounds),
    (&TyParen(ref lty), &TyParen(ref rty)) => is_ty_equal(lty, rty),
    (&TyTypeof(ref lof), &TyTypeof(ref rof)) => is_exp_equal(lof, rof),
	(&TyInfer, &TyInfer) => true,
	_ => false
	}
}

fn is_param_bound_equal(left : &TyParamBound, right : &TyParamBound) -> bool {
	match(left, right) {
	(&TraitTyParamBound(ref lpoly, ref lmod), &TraitTyParamBound(ref rpoly, ref rmod)) => 
		lmod == rmod && is_poly_traitref_equal(lpoly, rpoly),
    (&RegionTyParamBound(ref ltime), &RegionTyParamBound(ref rtime)) => is_lifetime_equal(ltime, rtime),
    _ => false
	}
}

fn is_poly_traitref_equal(left : &PolyTraitRef, right : &PolyTraitRef) -> bool {
	is_lifetimedef_vec_equal(&left.bound_lifetimes, &right.bound_lifetimes) && 
		is_path_equal(&left.trait_ref.path, &right.trait_ref.path)
}

fn is_param_bounds_equal(left : &TyParamBounds, right : &TyParamBounds) -> bool {
	over(left, right, is_param_bound_equal)
}

fn is_mut_ty_equal(left : &MutTy, right : &MutTy) -> bool {
	left.mutbl == right.mutbl && is_ty_equal(&left.ty, &right.ty)
}

fn is_bare_fn_ty_equal(left : &BareFnTy, right : &BareFnTy) -> bool {
	left.unsafety == right.unsafety && left.abi == right.abi && 
		is_lifetimedef_vec_equal(&left.lifetimes, &right.lifetimes) && is_fndecl_equal(&left.decl, &right.decl)
} 

fn is_fndecl_equal(left : &P<FnDecl>, right : &P<FnDecl>) -> bool {
	left.variadic == right.variadic && is_arg_vec_equal(&left.inputs, &right.inputs) && 
		is_fnret_ty_equal(&left.output, &right.output)
}

fn is_fnret_ty_equal(left : &FunctionRetTy, right : &FunctionRetTy) -> bool {
	match (left, right) {
	(&NoReturn(_), &NoReturn(_)) | (&DefaultReturn(_), &DefaultReturn(_)) => true,
	(&Return(ref lty), &Return(ref rty)) => is_ty_equal(lty, rty),
	_ => false	
	}
}

fn is_arg_equal(left : &Arg, right : &Arg) -> bool {
	is_ty_equal(&left.ty, &right.ty) && is_pat_equal(&left.pat, &right.pat)
}

fn is_arg_vec_equal(left : &Vec<Arg>, right : &Vec<Arg>) -> bool {
	over(left, right, is_arg_equal)
}

fn is_pat_equal(left : &Pat, right : &Pat) -> bool {
	match(&left.node, &right.node) {
	(&PatWild(lwild), &PatWild(rwild)) => lwild == rwild,
	(&PatIdent(ref lmode, ref lident, Option::None), &PatIdent(ref rmode, ref rident, Option::None)) =>
		lmode == rmode && is_ident_equal(&lident.node, &rident.node),
	(&PatIdent(ref lmode, ref lident, Option::Some(ref lpat)), 
			&PatIdent(ref rmode, ref rident, Option::Some(ref rpat))) =>
		lmode == rmode && is_ident_equal(&lident.node, &rident.node) && is_pat_equal(lpat, rpat),
    (&PatEnum(ref lpath, Option::None), &PatEnum(ref rpath, Option::None)) => is_path_equal(lpath, rpath),
    (&PatEnum(ref lpath, Option::Some(ref lenum)), &PatEnum(ref rpath, Option::Some(ref renum))) => 
		is_path_equal(lpath, rpath) && is_pat_vec_equal(lenum, renum),  
    (&PatStruct(ref lpath, ref lfieldpat, lbool), &PatStruct(ref rpath, ref rfieldpat, rbool)) =>
		lbool == rbool && is_path_equal(lpath, rpath) && is_spanned_fieldpat_vec_equal(lfieldpat, rfieldpat),
    (&PatTup(ref ltup), &PatTup(ref rtup)) => is_pat_vec_equal(ltup, rtup), 
    (&PatBox(ref lboxed), &PatBox(ref rboxed)) => is_pat_equal(lboxed, rboxed),
    (&PatRegion(ref lpat, ref lmut), &PatRegion(ref rpat, ref rmut)) => is_pat_equal(lpat, rpat) && lmut == rmut,
	(&PatLit(ref llit), &PatLit(ref rlit)) => is_exp_equal(llit, rlit),
    (&PatRange(ref lfrom, ref lto), &PatRange(ref rfrom, ref rto)) =>
		is_exp_equal(lfrom, rfrom) && is_exp_equal(lto, rto),
    (&PatVec(ref lfirst, Option::None, ref llast), &PatVec(ref rfirst, Option::None, ref rlast)) =>
		is_pat_vec_equal(lfirst, rfirst) && is_pat_vec_equal(llast, rlast),
    (&PatVec(ref lfirst, Option::Some(ref lpat), ref llast), &PatVec(ref rfirst, Option::Some(ref rpat), ref rlast)) =>
		is_pat_vec_equal(lfirst, rfirst) && is_pat_equal(lpat, rpat) && is_pat_vec_equal(llast, rlast),
	// I don't match macros for now, the code is slow enough as is ;-)
	_ => false
	}
}

fn is_spanned_fieldpat_vec_equal(left : &Vec<code::Spanned<FieldPat>>, right : &Vec<code::Spanned<FieldPat>>) -> bool {
	over(left, right, |l, r| is_fieldpat_equal(&l.node, &r.node))
}

fn is_fieldpat_equal(left : &FieldPat, right : &FieldPat) -> bool {
	left.is_shorthand == right.is_shorthand && is_ident_equal(&left.ident, &right.ident) && 
		is_pat_equal(&left.pat, &right.pat) 
}

fn is_ident_equal(left : &Ident, right : &Ident) -> bool {
	&left.name == &right.name && left.ctxt == right.ctxt
}

fn is_pat_vec_equal(left : &Vec<P<Pat>>, right : &Vec<P<Pat>>) -> bool {
	over(left, right, |l, r| is_pat_equal(l, r))
}

fn is_lifetimedef_equal(left : &LifetimeDef, right : &LifetimeDef) -> bool {
	is_lifetime_equal(&left.lifetime, &right.lifetime) && over(&left.bounds, &right.bounds, is_lifetime_equal)
}

fn is_lifetimedef_vec_equal(left : &Vec<LifetimeDef>, right : &Vec<LifetimeDef>) -> bool {
	over(left, right, is_lifetimedef_equal)
}

fn is_lifetime_equal(left : &Lifetime, right : &Lifetime) -> bool {
	left.name == right.name
}

fn is_ty_vec_equal(left : &Vec<P<Ty>>, right : &Vec<P<Ty>>) -> bool {
	over(left, right, |l, r| is_ty_equal(l, r))
}

fn over<X, F>(left: &[X], right: &[X], mut eq_fn: F) -> bool where F: FnMut(&X, &X) -> bool {
    left.len() == right.len() && left.iter().zip(right).all(|(x, y)| eq_fn(x, y))
}

fn both<X, F>(l: &Option<X>, r: &Option<X>, mut eq_fn : F) -> bool where F: FnMut(&X, &X) -> bool {
	if l.is_none() { r.is_none() } else { r.is_some() && eq_fn(l.as_ref().unwrap(), &r.as_ref().unwrap()) }
}

fn is_cmp_or_bit(op : &BinOp) -> bool {
    match op.node {
        BiEq | BiLt | BiLe | BiGt | BiGe | BiNe | BiAnd | BiOr | BiBitXor | BiBitAnd | BiBitOr => true,
        _ => false
    }
}
