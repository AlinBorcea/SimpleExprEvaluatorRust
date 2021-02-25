use std::env;
use std::process;

use expr_eval::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let cfg = Config::new(&args).unwrap_or_else(|err| {
        println!("Error! {}", err);
        process::exit(1);
    });

    println!("Expr -> {}", cfg.expression());
    println!("AlgType -> {}", cfg.alg_type());

}
