use std::{fs, i32};

fn main() {

    let contents = fs::read_to_string("puzzle.txt")
        .expect("Something went wrong reading the file");

    let mut counted = 0;
    let mut pair_counted = 0;

    for line in contents.lines() {
        
        // Splitting the elfs
        let first = line.split(",").collect::<Vec<&str>>()[0];
        let second = line.split(",").collect::<Vec<&str>>()[1];
        
        // Splitting the first elfs section IDs (1-4)
        let first_first = first.split("-").collect::<Vec<&str>>()[0].parse::<i32>().unwrap();
        let first_second = first.split("-").collect::<Vec<&str>>()[1].parse::<i32>().unwrap();

        // Splitting the second elfs section IDs (7-9)
        let second_first = second.split("-").collect::<Vec<&str>>()[0].parse::<i32>().unwrap();
        let second_second = second.split("-").collect::<Vec<&str>>()[1].parse::<i32>().unwrap();

        let mut first_elf = Vec::new();
        for i in first_first..first_second+1 {
            first_elf.push(i);
        };

        let mut second_elf = Vec::new();
        for i in second_first..second_second+1 {
            second_elf.push(i);
        };
        
        let mut fully_contained = true;
        let mut some_contained = false;
        if first_elf.len() > second_elf.len() {
           for id in second_elf {
               if first_elf.binary_search(&id).is_err() {
                    fully_contained = false
               } else if first_elf.binary_search(&id).is_ok() {
                   some_contained = true
               }
           } 
        } else {
           for id in first_elf {
               if second_elf.binary_search(&id).is_err() {
                    fully_contained = false
               } else if second_elf.binary_search(&id).is_ok() {
                   some_contained = true
               }

           } 
        }

        if fully_contained {
            counted += 1;
        }
        if some_contained {
            pair_counted += 1;
        }
    }

    println!("Found fully contained : {}", counted);
    println!("Found pairs contained : {}", pair_counted);
}
