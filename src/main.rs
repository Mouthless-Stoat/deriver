use stoat_prime::prelude::*;

fn main() {
    let mut e = Var.exp(5);

    for i in 0..10 {
        e = e.derive();
        println!("{}", e);
    }
}
