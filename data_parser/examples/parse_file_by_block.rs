use std::env;
use std::fs;

use nom::error::VerboseError;

use data_parser::validate;

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = args[1].clone();

    println!("reading file {}", filename);

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let blocks = contents.split("\n\n").collect::<Vec<_>>();
    let total = blocks.len();

    let ok = blocks
        .iter()
        .filter(|block| {
            let block = format!("{}\n", block);
            let parsed = validate::<VerboseError<&str>>(&block);
            parsed.is_ok()
        })
        .count();

    let first_failed = blocks
        .iter()
        .map(|oblock| {
            let block = format!("{}\n", oblock);
            let parsed = validate::<VerboseError<&str>>(&block);
            (oblock, parsed.is_err())
        })
        .filter(|(_, is_err)| *is_err)
        .next();

    println!("{} blocks found, {} ok", total, ok);

    println!("first failed: {:?}", first_failed);
}
