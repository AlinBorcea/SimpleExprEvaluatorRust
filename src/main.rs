use std::env;
use std::process;

use expr_eval::Config;
use expr_eval::two_stacks;

fn main() {
    let args: Vec<String> = env::args().collect();

    let cfg = Config::new(&args).unwrap_or_else(|err| {
        println!("{}", err);
        process::exit(1);
    });

    if let Err(e) = two_stacks::run(&cfg) {
        println!("{}", e);
        process::exit(2);
    } 

    println!("Success!");
}
