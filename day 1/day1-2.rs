use std::io::{BufRead, BufReader};
use std::fs::File;

// Learning Rust, i highly recommend not referencing this code
// Thanks to this guy: https://stackoverflow.com/questions/70307871/how-can-i-parse-the-nth-line-in-a-file-as-an-integer/70307923#70307923
fn main() {
    
    let mut previous_window: u32 = 0;
    let mut current_window: u32 = 0;
    let mut increases: i32 = -1; // because the first one doesn't count
    let mut counter: i32 = -2; // lol
    let reader = BufReader::new(File::open("input").expect("Cannot open file"));

    // This should've just been solved using itertools::Itertools;
    // Why make it easy tho?
    for line in reader.lines() {
        for number in line.unwrap().parse::<u32>() {
            current_window += number;

            if counter < 0 {
                counter += 1;
                continue;
            }

            if current_window > previous_window {
                increases += 1;
            }

            let reader = BufReader::new(File::open("input").expect("Cannot open file"));
            let tail: u32 = reader.lines()
                                  .nth(counter as usize)
                                  .expect("out of bounds")
                                  .expect("could not read line")
                                  .parse::<u32>()
                                  .expect("invalid number");

            previous_window = current_window;
            current_window = current_window - tail;
            
            counter += 1;
        }
    }

    println!("{}", increases);
}
