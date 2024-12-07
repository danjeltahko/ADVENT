use std::fs;
use regex::Regex;

fn main() {
    // Read the puzzle input
    let memory = fs::read_to_string("puzzle.txt")
        .expect("Did someone remove the file?");
    
    let mut result_one = 0;
    let mut result_two = 0;
    
    // First time figuring out regex by myself :D
    let re_one = Regex::new(r"mul(\([0-9]+),([0-9]+)\)").unwrap();
    let seq: Vec<&str> = re_one.find_iter(&memory).map(|m| m.as_str()).collect();

    let re = Regex::new(r"([0-9]+)").unwrap();
    for key in &seq {
        let numbers: Vec<i32> = re.find_iter(&key).map(|m| m.as_str().parse().unwrap()).collect();
        result_one += numbers[0] * numbers[1];
        
    }
    
    let re_two = Regex::new(r"mul(\([0-9]+),([0-9]+)\)|don't|do").unwrap();
    let seq: Vec<&str> = re_two.find_iter(&memory).map(|m| m.as_str()).collect();
    
    let re = Regex::new(r"([0-9]+)").unwrap();
    let mut include = true;
    for key in &seq {
        if *key == "do" {
            include = true
        } else if *key == "don't" {
            include = false
        } else {
            if include {
                let numbers: Vec<i32> = re.find_iter(&key).map(|m| m.as_str().parse().unwrap()).collect();
                result_two += numbers[0] * numbers[1];
            } else {
                continue;
            }
        }
        
    }
    println!("PART ONE: {result_one}");
    println!("PART TWO: {result_two}");
}
