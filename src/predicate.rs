use std::collections::BTreeSet;

use proc_macro_error2::emit_error;
use syn::{spanned::Spanned, BinOp, Expr, Ident, Lit, LitBool, UnOp};

pub fn validate_predicate(pred: &Expr, all_classes: &BTreeSet<Ident>) {
  match pred {
    Expr::Group(e) => validate_predicate(&e.expr, all_classes),
    Expr::Paren(e) => validate_predicate(&e.expr, all_classes),
    Expr::Unary(e) if matches!(e.op, UnOp::Not(_)) => validate_predicate(&e.expr, all_classes),
    Expr::Binary(e) if matches!(e.op, BinOp::And(_) | BinOp::Or(_)) => {
      validate_predicate(&e.left, all_classes);
      validate_predicate(&e.right, all_classes);
    }
    Expr::Path(e) if e.path.get_ident().is_some() => {
      let i = e.path.get_ident().unwrap();
      if !all_classes.contains(i) {
        emit_error!(i.span(), "undefined class name")
      }
    }
    Expr::Lit(e) if matches!(e.lit, Lit::Bool(_)) => {}
    e => {
      dbg!(e);
      emit_error!(e.span(), "invalid predicate syntax")
    }
  }
}

pub fn eval_predicate(pred: &Expr, classes: &BTreeSet<Ident>) -> bool {
  match pred {
    Expr::Group(e) => eval_predicate(&e.expr, classes),
    Expr::Paren(e) => eval_predicate(&e.expr, classes),
    Expr::Unary(e) if matches!(e.op, UnOp::Not(_)) => !eval_predicate(&e.expr, classes),
    Expr::Binary(e) if matches!(e.op, BinOp::And(_)) => {
      eval_predicate(&e.left, classes) && eval_predicate(&e.right, classes)
    }
    Expr::Binary(e) if matches!(e.op, BinOp::Or(_)) => {
      eval_predicate(&e.left, classes) || eval_predicate(&e.right, classes)
    }
    Expr::Path(e) if e.path.get_ident().is_some() => {
      let i = e.path.get_ident().unwrap();
      classes.contains(i)
    }
    Expr::Lit(e) if matches!(e.lit, Lit::Bool(_)) => {
      matches!(e.lit, Lit::Bool(LitBool { value: true, .. }))
    }
    _ => false,
  }
}
