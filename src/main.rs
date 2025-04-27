use stoat_prime::Expr;

fn main() {
    let e = Expr::Num(2.0).mul(Expr::Var.exp(7)).add(Expr::Var.exp(9.0));

    println!("{}", e.derive());
}
