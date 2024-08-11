use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::io::prelude::*;
use clap::{Arg, ArgMatches, Command};


fn get_input_args() -> ArgMatches{
    return Command::new("grep-lite")
        .version("0.1.0")
        .about("Searches for patterns")
        .arg(Arg::new("pattern")
            .help("The pattern to Search for ")
            .required(true)
            .index(1)
        )
        .arg(Arg::new("input")
            .help("File To Search")
            .required(true)
            .index(2)
        )
        .get_matches();
}


fn process_lines<T: BufRead + Sized>(reader: T, re: Regex) {
    for (i ,line ) in reader.lines().enumerate(){
        let l: String = line.unwrap();
        match re.find(&l) {
            Some(_) => println!("line {}, => {}",i, l),
            None => (),
        }
    }
}

fn main() {
    let args:ArgMatches = get_input_args();

    let pattern: &String = args.get_one::<String>("pattern").unwrap();
    let file_input: &String = args.get_one::<String>("input").unwrap();

    let re: Regex = Regex::new(pattern).unwrap();

    let f = File::open(file_input).unwrap();

    let reader = BufReader::new(f);

    process_lines(reader, re);
    // cargo run hello /Users/forest_choi/RustroverProjects/grep-lite/src/hello.txt
}