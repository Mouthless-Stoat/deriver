use std::f64::consts::E;

use crate::prelude::*;

impl Expr {
    pub fn derive(self) -> Self {
        match self {
            Bin(Mul, a, f) | Bin(Mul, f, a) if a.is_num() => a.mul(f.derive()),

            Bin(Add, f, g) => f.derive().add(g.derive()),
            Bin(Sub, f, g) => f.derive().sub(g.derive()),

            Bin(Div, f, g) => f
                .clone()
                .derive()
                .mul(*g.clone())
                .sub(g.clone().derive().mul(*f))
                .div(g.exp(2.0)),
            Bin(Mul, f, g) => f.clone().derive().mul(*g.clone()).add(g.derive().mul(*f)),

            Bin(Exp, f, g) => match (*f, *g) {
                // x^a -> ax^(a-1)
                (x @ Var, a @ Num(n)) => match n - 1.0 {
                    0.0 => a,
                    1.0 => a.mul(x),
                    p => a.mul(x.exp(p)),
                },

                // f(x)^n -> af(x)^(a-1)*f'(x)
                (f, a @ Num(n)) => a.mul(f.clone().exp(n - 1.0)).mul(f.derive()),

                // e^x -> e^x
                (e @ Num(E), x @ Var) => e.exp(x),

                // e^f(x) -> e^f(x) * f'(x)
                (e @ Num(E), f) => e.exp(f.clone()).mul(f.derive()),

                // a^x -> a^x * ln a
                (a @ Num(_), x @ Var) => a.clone().exp(x).mul(a.ln()),

                // a^f(x) -> a^f(x) * ln a * f'(x)
                (a @ Num(_), f) => a.clone().exp(f.clone()).mul(a.ln()).mul(f.derive()),

                // f(x)^g(x) -> f(x)^g(x) * (g'(x)*ln f(x) + f'(x)*g(x)/f(x))
                (f, g) => f.clone().exp(g.clone()).mul(
                    g.clone()
                        .derive()
                        .mul(f.clone().ln())
                        .mul(f.clone().derive().mul(g).div(f)),
                ),
            },

            Bin(Log, f, g) => match (*f, *g) {
                // ln x -> 1/x
                (Num(E), x @ Var) => Num(1.0).div(x),

                // ln f(x) -> f'(x)/f(x)
                (Num(E), f) => f.clone().derive().div(f),

                // log_a x -> 1/(x ln a)
                (a @ Num(_), x @ Var) => Num(1.0).div(x.mul(a.ln())),

                // log_a f(x) -> f'(x)/(f(x) * ln a)
                (a @ Num(_), f) => f.clone().derive().div(f.mul(a.ln())),

                // log_x a -> (ln a)/(x (ln x)^2)
                (x @ Var, a @ Num(_)) => a.ln().div(x.clone().mul(x.ln().exp(2.0))),

                // log_f(x) a -> (ln a)/(f(x) (ln f(x))^2) * ln f(x)
                (f, a @ Num(_)) => a
                    .ln()
                    .div(f.clone().mul(f.clone().exp(2.0).ln()))
                    .mul(f.derive()),

                (f, g) => g
                    .clone()
                    .derive()
                    .mul(f.clone().ln())
                    .div(g.clone())
                    .sub(f.clone().derive().mul(g.ln()).div(f.clone()))
                    .div(f.ln().exp(2.0)),
            },
            Trig(func, f) => match func {
                Sin => f.clone().trig(Cos),
                Tan => f.clone().trig(Sec).exp(2.0),
                Sec => f.clone().trig(Sec).mul(f.clone().trig(Tan)),

                Cos => f.clone().trig(Sin).neg(),
                Cot => f.clone().trig(Csc).exp(2.0).neg(),
                Csc => f.clone().trig(Csc).mul(f.clone().trig(Cot)).neg(),
            }
            .mul(f.derive()),

            Var => Num(1.0),
            Num(_) => Num(0.0),
        }
    }
}
