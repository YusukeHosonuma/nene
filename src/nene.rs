extern crate clap;

use self::clap::{App as Clap, Arg};
use std::fs;
use std::io::{BufWriter, Write};
use std::path::PathBuf;
use std::process::Command;

pub struct Config {
    pub input_path: PathBuf,
    pub output_path: Option<PathBuf>,
}

impl Config {
    pub fn new() -> Config {
        let matches = Clap::new("nene")
            .version("0.1.0")
            .author("tobi462 <tobi462@gmail.com>")
            .about("Remove ANSI escape codes in file.")
            .arg(Arg::with_name("path").help("input filepath").required(true))
            .arg(
                Arg::with_name("output")
                    .help("output filepath")
                    .short("o")
                    .long("out")
                    .takes_value(true),
            )
            .get_matches();

        let input_path = PathBuf::from(matches.value_of("path").unwrap());
        let output_path = matches.value_of("output").map(|path| PathBuf::from(path));

        Config {
            input_path,
            output_path,
        }
    }
}

pub struct App {
    config: Config,
}

impl App {
    pub fn new(config: Config) -> App {
        App { config }
    }

    pub fn run(&self) {
        let text = self.read_as_plaintext();
        self.output_result(&text);
    }

    fn read_as_plaintext(&self) -> String {
        let path = &self.config.input_path;

        // remove ANSI escape code
        let output = Command::new("sed")
            .arg("-E")
            .arg("s/\\[([0-9]{1,2}(;[0-9]{1,2})*)?m//g") // Note: include esc char \e
            .arg(path)
            .output()
            .unwrap();

        String::from_utf8(output.stdout).unwrap()
    }

    fn output_result(&self, text: &str) {
        match self.config.output_path {
            None => self.output_to_stdout(text),
            Some(ref path) => self.output_to_file(path, text),
        }
    }

    fn output_to_stdout(&self, text: &str) {
        println!("{}", text);
    }

    fn output_to_file(&self, path: &PathBuf, text: &str) {
        let mut w = BufWriter::new(fs::File::create(path.clone()).unwrap());
        w.write(text.as_bytes()).unwrap();
        eprintln!("output: {}", path.display());
    }
}
