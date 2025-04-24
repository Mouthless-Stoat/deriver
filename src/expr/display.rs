use std::f64::consts::E;
use std::fmt::Display;

use super::{Expr, Trig};

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            // catch all cases
            Self::Add(a, b) => write!(f, "({a} + {b})"),
            Self::Sub(a, b) => write!(f, "({a} - {b})"),
            Self::Div(a, b) => write!(f, "({a} / {b})"),
            Self::Mul(a, b) => write!(f, "({a} * {b})"),
            Self::Exp(a, b) => write!(f, "({a}^{b})"),

            Self::Log(e, a) if matches!(**e, Self::Num(E)) => write!(f, "(ln {a})"),
            Self::Log(a, b) => write!(f, "(log_{a} {b})"),

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
