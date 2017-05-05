extern crate shell_parse;

use shell_parse::parse_statements;

use std::fs::File;
use std::io::Read;
use std::env::args;

fn main() {
    let mut file = File::open(
        &args().nth(1).expect("You must supply a path")
    ).unwrap();
    let mut script = String::new();
    file.read_to_string(&mut script).unwrap();

    println!(
        "{:#?}",
        parse_statements(
            &script as &str
        ).collect::<Vec<_>>()
    );
}
