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
    println!("Loading expense-report...");
    let filename = "src/data/expense-report.txt";
    let lines = &lines_from_file(filename);
    // let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    println!("Doing expensive loops to figure out what two numbers add to 2020");
    for a in lines {
        for b in lines {
            for c in lines {
                let inta = a.parse::<i32>().unwrap();
                let intb = b.parse::<i32>().unwrap();
                let intc = c.parse::<i32>().unwrap();
                if inta + intb + intc == 2020 {
                    println!("{}, {}, and {} add to 2020", inta, intb, intc);
                    println!("product: {}", inta * intb * intc);
                    return;
                }
            }
        }
    }
}
