extern crate json_parser;
use json_parser::Value;
use std::env::args_os;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn main() {
    let path = args_os().skip(1).next().expect("usage: json_parser <filename>");
    let mut file = BufReader::new(File::open(path).expect("could not open file"));

    let mut s = String::new();
    file.read_to_string(&mut s).unwrap();

    Value::from_str(&s).unwrap();
}
