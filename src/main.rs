use std::env;
use std::process;

use expr_eval::Config;
use expr_eval::two_stacks;

fn main() {
    let mut args: Vec<String> = env::args().collect();
    let cfg = Config::new(&mut args).unwrap_or_else(|err| {
        println!("Error! {}", err);
        process::exit(1);
    });

    if let Err(e) = two_stacks::run(cfg) {
        println!("Error! {}", e);
        process::exit(1);
    }

}
