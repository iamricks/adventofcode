use std::io::{BufRead, BufReader};
use std::fs::File;

fn main() {
    
    let mut previous_number: u32 = 0;
    let mut increases: u32 = 0;
    let reader = BufReader::new(File::open("input").expect("Cannot open file"));

    // Skip the first line because we cannot increase from nothing
    for line in reader.lines().skip(1) {
        for number in line.unwrap().parse::<u32>() {
            if number > previous_number {
                increases += 1
            }

            previous_number = number;
        }
    }

    println!("{}", increases);
}
