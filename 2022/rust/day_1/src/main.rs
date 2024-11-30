use std::fs;

fn main() {

    let contents = fs::read_to_string("puzzle.txt")
        .expect("Something went wrong reading the file");
    
    let mut elfs: Vec<i32> = Vec::new();
    let mut counter = 0;
    for line in contents.lines() {
        if line.is_empty() {
            elfs.push(counter);
            counter = 0;
        } else {
            let cals: i32 = line.parse().unwrap();
            counter += cals;
        }
    }
    elfs.sort();
    elfs.reverse();
    let top_elfs = elfs[0] + elfs[1] + elfs[2];
    println!("Highest calorie count of {0}", elfs[0]);
    println!("Elf 1 has the lowest calorie count of {0}", top_elfs);
}
