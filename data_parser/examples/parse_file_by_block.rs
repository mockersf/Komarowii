use std::env;
use std::fs;

use data_parser::validate;

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = args[1].clone();

    println!("reading file {}", filename);

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let blocks = contents.split("\n\n").collect::<Vec<_>>();
    let total = blocks.iter().filter(|block| **block != "").count();

    let ok = blocks
        .iter()
        .filter(|block| **block != "")
        .filter(|block| {
            let block = format!("{}\n", block);
            let parsed = validate(&block);
            parsed.is_ok()
        })
        .count();

    let first_failed = blocks
        .iter()
        .map(|oblock| {
            let block = format!("{}\n", oblock);
            let parsed = validate(&block);
            (oblock, parsed.is_err())
        })
        .filter(|(_, is_err)| *is_err)
        .next();

    println!("{} blocks found, {} ok", total, ok);
    if total == ok {
        std::process::exit(0);
    }

    println!(
        "first failed: \n{:?}\n--> because: {}",
        first_failed.map(|(b, _)| format!("{}\n", b)),
        first_failed
            .map(|(block, _)| {
                let block = format!("{}\n", block);
                let res = validate(&block);
                format!("{:#?}", res)
            })
            .unwrap_or_else(|| String::from(""))
    );
    std::process::exit(1);
}
