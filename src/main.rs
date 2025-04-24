mod expr;
use expr::*;

fn main() {
    let e = Expr::Num(2.0)
        .mul(Expr::Var.exp(2.0))
        .div(Expr::Var.add(2.0));

    println!("{e}");
    println!("{}", e.derive());
}

impl From<f64> for Expr {
    fn from(value: f64) -> Self {
        Expr::Num(value)
    }
}
