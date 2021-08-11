use std::env;
use minigrep::{Config, run};
use std::process;


fn main() {
    let args = env::args();

    let config = Config::new(args).unwrap_or_else(|err| {
        println!("{}", err);
        process::exit(1);
    });

    println!("Searching for  '{}'  in  {}", config.query, config.filename);

    if let Err(e) = run(config){
        println!("The program occurred error: {}", e);
        process::exit(1);
    }

}

