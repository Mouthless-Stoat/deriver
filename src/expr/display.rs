use std::f64::consts::E;
use std::fmt::Display;

use crate::{
    Bin::*,
    Expr::{self, *},
    Trig::*,
};

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            // catch all cases
            Bin(t, a, b) => {
                let mut a_str = a.to_string();
                let mut b_str = b.to_string();

                if a.precedence() < self.precedence() {
                    a_str = format!("({a})");
                }

                if b.precedence() < self.precedence() {
                    b_str = format!("({b})");
                }

                match t {
                    Add => write!(f, "{a_str} + {b_str}"),
                    Sub => write!(f, "{a_str} - {b_str}"),
                    Mul => write!(f, "{a_str} * {b_str}"),
                    Div => write!(f, "{a_str} / {b_str}"),
                    Exp => write!(f, "{a_str}^{b_str}"),
                    Log if matches!(**a, Self::Num(E)) => write!(f, "ln {b_str}"),
                    Log => write!(f, "log_{a_str} {b_str}"),
                }
            }

            Trig(t, a) => write!(
                f,
                "{} {}",
                match t {
                    Sin => "sin",
                    Cos => "cos",
                    Tan => "tan",
                    Csc => "csc",
                    Sec => "sec",
                    Cot => "cot",
                },
                a
            ),

            Var => write!(f, "x"),
            Num(E) => write!(f, "e"),
            Num(n) => write!(f, "{n}"),
        }
    }
}
