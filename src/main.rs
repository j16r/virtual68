mod ast;
mod grammar;
mod machine;

use std::fs::File;
use std::io::Read;
use std::path::Path;
use machine::{new};

fn main() {
    let args: Vec<_> = ::std::env::args().collect();
    let mut file = File::open(&args[1]).unwrap();

    let mut contents: Vec<u8> = Vec::new();
    let result = file.read_to_end(&mut contents).unwrap();
    let input = String::from_utf8(contents).unwrap();

    let machine = machine::new(&input);
    machine.run();
}
