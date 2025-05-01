use std::f64::consts::E;

use super::{
    Bin::*,
    Expr::{self, *},
    Trig,
};

#[allow(clippy::should_implement_trait)]
impl Expr {
    /// Add this expr to another.
    pub fn add(self, f: impl Into<Expr>) -> Self {
        Bin(Add, Box::new(self), Box::new(f.into()))
    }

    /// Subtract this expr to another.
    pub fn sub(self, f: impl Into<Expr>) -> Self {
        Bin(Sub, Box::new(self), Box::new(f.into()))
    }

    /// Divide this expr by another.
    pub fn div(self, f: impl Into<Expr>) -> Self {
        Bin(Div, Box::new(self), Box::new(f.into()))
    }

    /// Multiply this expr by another.
    pub fn mul(self, f: impl Into<Expr>) -> Self {
        Bin(Mul, Box::new(self), Box::new(f.into()))
    }

    /// raise this expr to another power.
    /// If the expr is `x` and the other is `a`, the result will be `x^a`
    pub fn exp(self, f: impl Into<Expr>) -> Self {
        Bin(Exp, Box::new(self), Box::new(f.into()))
    }

    /// The the nth root of this expr. This does not produce a div expr but instead pre-compute the
    /// value
    pub fn root_n(self, n: f64) -> Self {
        Bin(Exp, Box::new(self), Box::new((1.0 / n).into()))
    }

    /// Take the root of this expr.
    pub fn root(self, f: impl Into<Expr>) -> Self {
        Bin(Exp, Box::new(self), Box::new(Num(1.0).div(f.into())))
    }

    pub fn sprt(self) -> Self {
        Bin(Exp, Box::new(self), Box::new((1.0 / 2.0).into()))
    }

    /// log base expr of another value.
    /// If the expr is `x` and the other is `a`, the result will be `log_x (a)`
    pub fn log(self, f: impl Into<Expr>) -> Self {
        Bin(Log, Box::new(f.into()), Box::new(self))
    }

    pub fn ln(self) -> Self {
        Bin(Log, Box::new(E.into()), Box::new(self))
    }

    /// Apply a trig function to this value.
    pub fn trig(self, func: Trig) -> Self {
        Trig(func, Box::new(self))
    }

    pub fn neg(self) -> Self {
        self.mul(-1.0)
    }
}
