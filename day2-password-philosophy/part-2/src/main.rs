use regex::Regex;
use std::{
    env, fs,
    io::{prelude::*, BufReader},
    path,
};

fn lines_from_file(filename: impl AsRef<path::Path>) -> Vec<String> {
    let file = fs::File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn main() {
    let re = Regex::new(r"^(\d+)-(\d+) (\w+): (\w+)$").unwrap();
    let mut num_valid = 0;

    println!("Loading input...");
    let filename = "data/input";
    let lines = &lines_from_file(filename);
    println!("Looping over content...");
    for a in lines {
        println!("Parsing line {}", a);
        for cap in re.captures_iter(a) {
            let pos1 = cap[1].parse::<usize>().unwrap();
            let pos2 = cap[2].parse::<usize>().unwrap();
            let chr = &cap[3];
            let pass = &cap[4];
            println!("pos1: {} pos2: {} Char: {} Pass: {}", pos1, pos2, chr, pass);

            // Assuming passwords are non-unicode, 1 byte per char.
            let pos1_char = &pass[pos1 - 1..pos1];
            let pos2_char = &pass[pos2 - 1..pos2];
            if pos1_char == chr || pos2_char == chr {
                if pos1_char == pos2_char {
                    continue;
                }
                println!("Valid: {}", a);
                println!("pos1_char: {} pos2_char: {}", pos1_char, pos2_char);
                num_valid = num_valid + 1;
            }
        }
    }
    println!("Valid passwords: {}", num_valid)
}
