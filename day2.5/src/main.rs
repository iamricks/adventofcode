use std::io::{BufRead, BufReader};
use std::fs::File;
use regex::Regex;
use lazy_static::lazy_static;

fn extract_number(text: &str) -> &str {
    lazy_static! {
        static ref RE: Regex = Regex::new("[0-9]+").unwrap();
    }

    return RE.find(text).unwrap().as_str();
}

fn extract_text(text: &str) -> &str {
    lazy_static! {
        static ref RE: Regex = Regex::new("[a-zA-Z]+").unwrap();
    }
    
    return RE.find(text).unwrap().as_str();
}

fn main() {
    let mut horizontal_position: u32 = 0;
    let mut depth: u32 = 0;
    let mut aim: u32 = 0;
    let reader = BufReader::new(File::open("input").expect("Cannot open file"));

    // Skip the first line because we cannot increase from nothing
    for line in reader.lines() {
        let line_str: String = line.unwrap();
        let direction = extract_text(&line_str);
        let distance_str = extract_number(&line_str);
        let distance = distance_str.parse::<u32>().unwrap();

        match direction {
            "up" => {
                aim -= distance;
            },
            "down" => {
                aim += distance;
            },
            "forward" => {
                horizontal_position += distance;
                depth += aim * distance;
            },
            _ => println!("something else!"),
        }
    }

    println!("Depth: {}", depth);
    println!("Horizontal Position: {}", horizontal_position);
    println!("Answer {}", horizontal_position * depth);
}
