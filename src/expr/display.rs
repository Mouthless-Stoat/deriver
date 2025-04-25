use std::f64::consts::E;
use std::fmt::Display;

use super::{Expr, Trig};

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            // catch all cases
            t @ (Self::Add(a, b)
            | Self::Sub(a, b)
            | Self::Div(a, b)
            | Self::Mul(a, b)
            | Self::Exp(a, b)
            | Self::Log(a, b)) => {
                let mut a_str = a.to_string();
                let mut b_str = b.to_string();

                if **a < *t {
                    a_str = format!("({a})");
                }

                if **b < *t {
                    b_str = format!("({b})");
                }

                match t {
                    Expr::Add(..) => write!(f, "{a} + {b}"),
                    Expr::Sub(..) => write!(f, "{a} - {b}"),
                    Expr::Mul(..) => write!(f, "{a} * {b}"),
                    Expr::Div(..) => write!(f, "{a} / {b}"),
                    Expr::Exp(..) => write!(f, "{a}^{b}"),
                    Expr::Log(e, ..) if matches!(**e, Self::Num(E)) => write!(f, "ln {b}"),
                    Expr::Log(..) => write!(f, "log_{a} {b}"),
                    _ => unreachable!(),
                }
            }

            Self::Trig(t, a) => write!(
                f,
                "{} {}",
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

            Self::Var => write!(f, "x"),
            Self::Num(E) => write!(f, "e"),
            Self::Num(n) => write!(f, "{n}"),
        }
    }
}
