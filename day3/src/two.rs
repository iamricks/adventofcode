use std::io::{BufRead, BufReader};
use std::fs::File;

pub fn calculate_oxygen() -> String {
    return calc(false);
}

pub fn calculate_carbon() -> String {
    return calc(true);
}

pub fn calc(flip: bool) -> String {

    let reader = BufReader::new(File::open("input").expect("Cannot open file"));
    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>().unwrap();
    let line_count = lines.len();
    let mut indicies: Vec<usize> = (0..line_count).collect();
    let num_count: Vec<usize> = (0..12).collect();

    for position in num_count {
        let mut one_count: u32 = 0;
        let mut zero_count: u32 = 0;
        let mut zero_indicies: Vec<usize> = vec![];
        let mut one_indicies: Vec<usize> = vec![];

        for i in &indicies {

            if indicies.len() == 1 {
                return lines[indicies[0]].clone();
            }

            let val = &lines[*i];
            let bit = val.chars().nth(position).unwrap();
            
            match bit as i32 - 0x30 {
                1 => {
                    one_count += 1;
                    one_indicies.push(*i);
                },
                0 => {
                    zero_count += 1;
                    zero_indicies.push(*i);
                },
                _ => println!("Not a bit"),
            }
        }

        if flip {
            if one_count < zero_count {
                indicies = one_indicies;
            } else {
                indicies = zero_indicies;
            }
        } else {
            if one_count >= zero_count {
                indicies = one_indicies;
            } else {
                indicies = zero_indicies;
            }
        }

    }
    println!("indicies {}", lines[indicies[0]]);
    return lines[indicies[0]].clone();
}

pub fn run() {
    let oxygen = calculate_oxygen();
    let carbon = calculate_carbon();
    let o_str: &str = &oxygen[..];
    let c_str: &str = &carbon[..];

    let oxygen_bit = isize::from_str_radix(o_str, 2).unwrap();
    let carbon_bit = isize::from_str_radix(c_str, 2).unwrap();
    
    println!("Oxygen {}", oxygen_bit);
    println!("Carbon {}", carbon_bit);
    println!("Answer {}", oxygen_bit * carbon_bit);
}