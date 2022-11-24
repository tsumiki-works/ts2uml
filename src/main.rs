use std::env;
use std::fs::File;
use std::io::prelude::*;

use lexer::get_token;

mod lexer;
fn main() {
    let args = env::args().collect::<Vec<String>>();
    if args.len() != 2 {
        println!("ts2uml need only 1 argments");
        return;
    }
    let f = File::open(&args[1]);
    let mut file = 
        match f {
            Ok(file) => file,
            Err(error) => {
                println!("Expect: {:?}", error);
                return;
            },
        };
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Something went wrong reading file");
    get_token(contents);
}
