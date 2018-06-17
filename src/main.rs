extern crate clap;

use std::process::Command;
use std::fs;
use std::io::{BufWriter, Write};
use clap::{Arg, App};

fn main() {
    let matches = App::new("nene")
                          .version("0.1.0")
                          .author("tobi462 <tobi462@gmail.com>")
                          .about("Remove ANSI escape codes in file.")
                          .arg(Arg::with_name("filename")
                                .help("input filename")
                                .required(true))
                          .arg(Arg::with_name("output")
                                .help("output filename")
                                .short("o")
                                .long("out")
                                .takes_value(true))
                          .get_matches();

    // TODO: support path

    let filename = matches.value_of("filename").unwrap();

    let text = read_as_plaintext(&filename);

    match matches.value_of("output") {
        None => println!("{}", text),
        Some(filename) => write_newfile(&filename, &text),
    }
}

fn read_as_plaintext(filename: &str) -> String {

    // remove ANSI escape code
    let output = Command::new("sed")
        .arg("-E")
        .arg("s/\\[([0-9]{1,2}(;[0-9]{1,2})*)?m//g") // Note: include esc char \e
        .arg(filename)
        .output()
        .unwrap();

    String::from_utf8(output.stdout).unwrap()
}

fn write_newfile(filename: &str, text: &str) {
    let mut w = BufWriter::new(fs::File::create(filename.clone()).unwrap());
    w.write(text.as_bytes()).unwrap();
    eprintln!("output: {}", filename);
}
