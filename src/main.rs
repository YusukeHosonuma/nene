use std::env;
use std::process::Command;
use std::fs;
use std::io::{BufWriter, Write};

fn main() {
    let filename = env::args().skip(1).next().unwrap();

    let text = read_as_plaintext(&filename);

    let filename = new_filename(&filename);

    write_newfile(&filename, &text);

    eprintln!("output: {}", filename);
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
}

fn new_filename(filename: &str) -> String {
    String::from("plain_") + filename
}
