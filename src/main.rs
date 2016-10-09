#[macro_use] extern crate enum_primitive;
extern crate combine;

mod ast;
mod grammar;
mod linker;
mod machine;
mod opcode;

use std::fs::File;
use std::io::Read;

fn main() {
    let args: Vec<_> = ::std::env::args().collect();
    let mut file = File::open(&args[1]).unwrap();

    let mut contents: Vec<u8> = Vec::new();
    file.read_to_end(&mut contents).unwrap();
    let input = String::from_utf8(contents).unwrap();

    let mut machine = machine::new();
    machine.load(&input);
    machine.run();

    println!("machine state {}", machine);
}

#[cfg(test)]
mod tests {
    use grammar::parse_program;

    #[test]
    fn parse() {
        assert!(parse_program("psh a").is_ok());
        assert!(parse_program("sub a 1").is_ok());
    }
}
