use std::{env, process};

fn main() {
    let mut args = env::args();

    args.next();

    let arg = args.next().unwrap_or_else(|| {
        eprintln!("Problem parsing arguments!");
        process::exit(1);
    });

    let num: u128 = arg.parse().expect("Not a valid number!");

    fn factorial(n: u128) -> u128 {
        if n == 0 {
            1
        } else {
            match n.checked_mul(factorial(n - 1)) {
                Some(n) => n,
                None => {
                    eprintln!("The result it too big!");
                    process::exit(1);
                }
            }
        }
    }

    println!("{:?}", factorial(num));
}
