use crate::{Expr, Trig};

#[test]
fn simple() {
    assert_eq!(
        "1 + 2 * 3".parse::<Expr>().unwrap(),
        Expr::Num(1.0).add(Expr::Num(2.0).mul(3.0))
    )
}

#[test]
fn order_operation() {
    assert_eq!(
        "1 * 2 + 3 * 4".parse::<Expr>().unwrap(),
        Expr::Num(1.0).mul(2.0).add(Expr::Num(3.0).mul(4.0))
    )
}

#[test]
fn func() {
    assert_eq!(
        "sin 90".parse::<Expr>().unwrap(),
        Expr::Num(90.0).trig(Trig::Sin)
    )
}

#[test]
fn func_exp() {
    assert_eq!(
        "sin x^2".parse::<Expr>().unwrap(),
        Expr::Var.exp(2.0).trig(Trig::Sin)
    )
}

#[test]
fn func_paren() {
    assert_eq!(
        "sin (90 * x)".parse::<Expr>().unwrap(),
        Expr::Num(90.0).mul(Expr::Var).trig(Trig::Sin)
    )
}

#[test]
fn log_base() {
    assert_eq!("log_5 x".parse::<Expr>().unwrap(), Expr::Var.log(5.0))
}

#[test]
fn log() {
    assert_eq!("log x".parse::<Expr>().unwrap(), Expr::Var.log(10.0))
}

#[test]
fn log_complex_base() {
    assert_eq!(
        "log_x^2 10".parse::<Expr>().unwrap(),
        Expr::Num(10.0).log(Expr::Var.exp(2.0))
    )
}
