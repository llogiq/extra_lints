pub use rustc::plugin::Registry;
pub use rustc::lint::*;
pub use syntax::ast::*;
pub use syntax::ast_util::{is_comparison_binop, binop_to_string};
pub use syntax::ptr::P;
pub use syntax::codemap::Span;

declare_lint! {
    BAD_BIT_MASK,
    Deny,
    "Deny the use of incompatible bit masks in comparisons, e.g. '(a & 1) == 2'"
}

#[derive(Copy,Clone)]
pub struct BitMask;

impl LintPass for BitMask {
    fn get_lints(&self) -> LintArray {
        lint_array!(BAD_BIT_MASK)
    }
    
    fn check_expr(&mut self, cx: &Context, e: &Expr) {
        if let ExprBinary(ref cmp, ref left, ref right) = e.node {
			if !is_comparison_binop(cmp.node) { return; }
			if let Some(cmp_value) = fetch_int_literal(&right.node) {
				check_compare(cx, left, cmp.node, cmp_value, &e.span);
			}
		}
    }
}

fn check_compare(cx: &Context, bit_op: &Expr, cmp_op: BinOp_, cmp_value: u64, span: &Span) {
	match &bit_op.node {
		&ExprParen(ref subexp) => check_compare(cx, subexp, cmp_op, cmp_value, span),
		&ExprBinary(ref op, ref left, ref right) => {
			if op.node != BiBitAnd && op.node != BiBitOr { return; }
			if let Some(mask_value) = fetch_int_literal(&right.node) {
				check_bit_mask(cx, op.node, cmp_op, mask_value, cmp_value, span);
			} else if let Some(mask_value) = fetch_int_literal(&left.node) {
				check_bit_mask(cx, op.node, cmp_op, mask_value, cmp_value, span);
			}
		},
		_ => ()
	}
}

fn check_bit_mask(cx: &Context, bit_op: BinOp_, cmp_op: BinOp_, mask_value: u64, cmp_value: u64, span: &Span) {
	match cmp_op {
		BiEq | BiNe => match bit_op {
			BiBitAnd => if mask_value & cmp_value != mask_value {
				cx.span_lint(BAD_BIT_MASK, *span, &format!("incompatible bit mask: _ & {} can never be equal to {}", mask_value,
					cmp_value));
			},
			BiBitOr => if mask_value | cmp_value != cmp_value {
				cx.span_lint(BAD_BIT_MASK, *span, &format!("incompatible bit mask: _ | {} can never be equal to {}", mask_value,
					cmp_value));
			},
			_ => ()
		},
		BiLt | BiGe => match bit_op {
			BiBitAnd => if mask_value < cmp_value {
				cx.span_lint(BAD_BIT_MASK, *span, &format!("incompatible bit mask: _ & {} will always be lower than {}", mask_value,
					cmp_value));
			},
			BiBitOr => if mask_value >= cmp_value {
				cx.span_lint(BAD_BIT_MASK, *span, &format!("incompatible bit mask: _ | {} will never be lower than {}", mask_value,
					cmp_value));
			},
			_ => ()
		},
		BiLe | BiGt => match bit_op {
			BiBitAnd => if mask_value <= cmp_value {
				cx.span_lint(BAD_BIT_MASK, *span, &format!("incompatible bit mask: _ & {} will never be higher than {}", mask_value,
					cmp_value));
			},
			BiBitOr => if mask_value > cmp_value {
				cx.span_lint(BAD_BIT_MASK, *span, &format!("incompatible bit mask: _ | {} will always be higher than {}", mask_value,
					cmp_value));				
			},
			_ => ()
		},
		_ => ()
	}
}

fn fetch_int_literal(lit : &Expr_) -> Option<u64> {
	if let &ExprLit(ref lit_ptr) = lit {
		if let &LitInt(value, _) = &lit_ptr.node {
			return Option::Some(value); //TODO: Handle sign
		}
	}
	Option::None
}
