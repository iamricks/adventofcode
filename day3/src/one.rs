use std::io::{BufRead, BufReader};
use std::fs::File;

pub fn run() {

    let mut tally = vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let mut gamma_rate = String::from("");
    let mut epsilon_rate = String::from("");
    let reader = BufReader::new(File::open("input").expect("Cannot open file"));

    // Skip the first line because we cannot increase from nothing
    for line in reader.lines() {
        let line_str: String = line.unwrap();

        for (i, c) in line_str.chars().enumerate() { 
            match c as i32 - 0x30 {
                1 => {
                    tally[i] += 1;
                },
                0 => {
                    tally[i] -= 1;
                },
                _ => println!("Not a bit"),
            }
        }
    }

    for num in &tally {
        if num > &0 {
            gamma_rate.push_str("1");
            epsilon_rate.push_str("0");
        } else {
            gamma_rate.push_str("0");
            epsilon_rate.push_str("1");
        }
    }

    println!("Tally: {:?}", tally);
    println!("Gamma Rate: {}", gamma_rate);
    println!("Epsilon Rate: {}", epsilon_rate);

    let g_rate = isize::from_str_radix(&gamma_rate, 2).unwrap();
    let e_rate = isize::from_str_radix(&epsilon_rate, 2).unwrap();

    println!("Answer: {}", g_rate * e_rate);
}