use std::f64::consts::E;

use super::{Expr, Trig};

impl Expr {
    pub fn derive(self) -> Self {
        match self {
            Self::Mul(a, f) | Self::Mul(f, a) if a.is_num() => a.mul(f.derive()),

            Self::Add(f, g) => f.derive().add(g.derive()),
            Self::Sub(f, g) => f.derive().sub(g.derive()),

            Self::Div(f, g) => f
                .clone()
                .derive()
                .mul(*g.clone())
                .add(f.mul(g.clone().derive()))
                .div(g.exp(2.0)),
            Self::Mul(f, g) => f.clone().derive().mul(*g.clone()).add(g.derive().mul(*f)),

            Self::Exp(f, g) => match (*f, *g) {
                // Variable convention:
                // x for any variable
                // f for any function
                // a for any exponent
                // n for the value of any exponent
                // f for any function

                // x^a -> ax^(a-1)
                (x @ Self::Var, a @ Self::Num(n)) => a.mul(x.exp(n - 1.0)),

                // f(x)^n -> af(x)^(a-1)*f'(x)
                (f, a @ Self::Num(n)) => a.mul(f.clone().exp(n - 1.0)).mul(f.derive()),

                // e^x -> e^x
                (e @ Self::Num(E), x @ Self::Var) => e.exp(x),

                // e^f(x) -> e^f(x) * f'(x)
                (e @ Self::Num(E), f) => e.exp(f.clone()).mul(f.derive()),

                // a^x -> a^x * ln a
                (a @ Self::Num(_), x @ Self::Var) => a.clone().exp(x).mul(Self::ln(a)),

                // a^f(x) -> a^f(x) * ln a * f'(x)
                (a @ Self::Num(_), f) => a.clone().exp(f.clone()).mul(Self::ln(a)).mul(f.derive()),

                _ => panic!("Exponents not in the form of f(x)^n or n^f(x) where n is a number are too complicated can't compute"),
            },

            Self::Log(f, g) => match (*f, *g) {
                // ln x -> 1/x
                (Self::Num(E), x @ Self::Var) => Self::Num(1.0).div(x),

                // ln f(x) -> f'(x)/f(x)
                (Self::Num(E), f) => f.clone().derive().div(f),

                // log_a f(x) -> f'(x)/(f(x) * ln a)
                (a @ Self::Num(_), f) => f.clone().derive().div(f.mul(Self::ln(a))),

                // log_x a -> (ln a)/(x (ln x)^2)
                (x @ Self::Var, a @ Self::Num(_)) => Self::ln(a).div(x.clone().mul(Self::ln(x).exp(2.0))),

                // log_f(x) a -> (ln a)/(f(x) (ln f(x))^2) * ln f(x)
                (f, a @ Self::Num(_)) => Self::ln(a).div(f.clone().mul(Self::ln(f.clone()).exp(2.0))).mul(f.derive()),

                _ => panic!("Logs not in the form of log_n (f(x)) or log_f(x) (n) where n is a number are too complicated can't compute"),
            },

            Self::Trig(func, f) => match func {
                Trig::Sin => f.clone().trig(Trig::Cos),
                Trig::Tan => f.clone().trig(Trig::Sec).exp(2.0),
                Trig::Sec => f.clone().trig(Trig::Sec).mul(f.clone().trig(Trig::Tan)),

                Trig::Cos => f.clone().trig(Trig::Sin).mul(-1.0),
                Trig::Cot => f.clone().trig(Trig::Csc).exp(2.0).mul(-1.0),
                Trig::Csc => f.clone().trig(Trig::Csc).mul(f.clone().trig(Trig::Cot)).mul(-1.0),
            }.mul(f.derive()),

            Self::Var => Self::Num(1.0),
            Self::Num(_) => Self::Num(0.0),
        }
    }
}
