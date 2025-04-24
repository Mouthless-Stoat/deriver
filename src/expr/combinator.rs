use std::f64::consts::E;

use super::{Expr, Trig};

impl Expr {
    /// Add this expr to another.
    pub fn add(self, f: impl Into<Expr>) -> Self {
        Self::Add(Box::new(self), Box::new(f.into()))
    }

    /// Subtract this expr to another.
    pub fn sub(self, f: impl Into<Expr>) -> Self {
        Self::Sub(Box::new(self), Box::new(f.into()))
    }

    /// Divide this expr by another.
    pub fn div(self, f: impl Into<Expr>) -> Self {
        Self::Div(Box::new(self), Box::new(f.into()))
    }

    /// Multiply this expr by another.
    pub fn mul(self, f: impl Into<Expr>) -> Self {
        Self::Mul(Box::new(self), Box::new(f.into()))
    }

    /// raise this expr to another power.
    /// If the expr is `x` and the other is `a`, the result will be `x^a`
    pub fn exp(self, f: impl Into<Expr>) -> Self {
        Self::Exp(Box::new(self), Box::new(f.into()))
    }

    /// log base expr of another value.
    /// If the expr is `x` and the other is `a`, the result will be `log_x (a)`
    pub fn log(self, f: impl Into<Expr>) -> Self {
        Self::Log(Box::new(self), Box::new(f.into()))
    }

    pub fn ln(f: impl Into<Expr>) -> Self {
        Self::Log(Box::new(E.into()), Box::new(f.into()))
    }

    /// Apply a trig function to this value.
    pub fn trig(self, func: Trig) -> Self {
        Self::Trig(func, Box::new(self))
    }

    /// Will panics if trying to flip a non binary.
    pub fn flip(self) -> Self {
        match self {
            Expr::Add(a, b) => Expr::Add(b, a),
            Expr::Sub(a, b) => Expr::Sub(b, a),
            Expr::Div(a, b) => Expr::Div(b, a),
            Expr::Mul(a, b) => Expr::Mul(b, a),
            Expr::Exp(a, b) => Expr::Exp(b, a),
            Expr::Log(a, b) => Expr::Log(b, a),

            _ => panic!("Tried to flip a non binary value"),
        }
    }
}
