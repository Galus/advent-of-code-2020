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
            let min = &cap[1];
            let max = &cap[2];
            let chr = &cap[3];
            let pass = &cap[4];
            println!("Min: {} Max: {} Char: {} Pass: {}", min, max, chr, pass);
            let match_count = pass.matches(chr).count();
            if match_count >= min.parse::<usize>().unwrap()
                && match_count <= max.parse::<usize>().unwrap()
            {
                println!("Valid: {}", a);
                num_valid = num_valid + 1;
            }
        }
    }
    println!("Valid passwords: {}", num_valid)
}
