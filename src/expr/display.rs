use std::fmt::Display;

use super::{Expr, Trig};

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Add(a, b) => write!(f, "({a}) + ({b})"),
            Expr::Sub(a, b) => write!(f, "({a}) - ({b})"),
            Expr::Div(a, b) => write!(f, "({a}) / ({b})"),
            Expr::Mul(a, b) => write!(f, "({a}) / ({b})"),
            Expr::Exp(a, b) => write!(f, "({a})^({b})"),
            Expr::Log(a, b) => write!(f, "log_({a}) ({b})"),
            Expr::Trig(t, a) => write!(
                f,
                "{}({})",
                match t {
                    Trig::Sin => "sin",
                    Trig::Cos => "cos",
                    Trig::Tan => "tan",
                    Trig::Csc => "csc",
                    Trig::Sec => "sec",
                    Trig::Cot => "cot",
                },
                a
            ),
            Expr::Var => write!(f, "x"),
            Expr::Num(n) => write!(f, "{n}"),
        }
    }
}
