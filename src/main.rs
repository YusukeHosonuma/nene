extern crate clap;

use std::process::Command;
use std::fs;
use std::io::{BufWriter, Write};
use clap::{Arg, App};
use std::path::Path;

fn main() {
    let matches = App::new("nene")
                          .version("0.1.0")
                          .author("tobi462 <tobi462@gmail.com>")
                          .about("Remove ANSI escape codes in file.")
                          .arg(Arg::with_name("path")
                                .help("input filepath")
                                .required(true))
                          .arg(Arg::with_name("output")
                                .help("output filepath")
                                .short("o")
                                .long("out")
                                .takes_value(true))
                          .get_matches();

    let path = Path::new(matches.value_of("path").unwrap());
    let text = read_as_plaintext(&path);

    match matches.value_of("output") {
        None => println!("{}", text),
        Some(path) => write_newfile(&Path::new(path), &text),
    }
}

fn read_as_plaintext(path: &Path) -> String {

    // remove ANSI escape code
    let output = Command::new("sed")
        .arg("-E")
        .arg("s/\\[([0-9]{1,2}(;[0-9]{1,2})*)?m//g") // Note: include esc char \e
        .arg(path)
        .output()
        .unwrap();

    String::from_utf8(output.stdout).unwrap()
}

fn write_newfile(path: &Path, text: &str) {
    let mut w = BufWriter::new(fs::File::create(path.clone()).unwrap());
    w.write(text.as_bytes()).unwrap();
    eprintln!("output: {}", path.display());
}
