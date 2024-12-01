use std::fs;

fn main() {

    // Read the puzzle input
    let content = fs::read_to_string("puzzle.txt")
        .expect("Did someone remove the file?");
    
    // Initialize left and right list
    let mut left = Vec::<i32>::new();
    let mut right = Vec::<i32>::new();

    // Iterate through each line and split it
    // to left and right list. - 37033   48086
    for line in content.lines() {
        left.push(line.split("   ").collect::<Vec<_>>()[0].trim().parse().unwrap());
        right.push(line.split("   ").collect::<Vec<_>>()[1].trim().parse().unwrap());
    }

    left.sort();
    right.sort();

    // Get the distance between the two lists
    let mut distance = 0;
    for i in 0..left.len() {
        distance += (left[i] - right[i]).abs();
    }
    // Get the similarity between the two lists
    let mut similarity = 0;
    for i in &left {
        let mut found = 0;
        for j in &right {
            if i == j {
                found += 1;
            }
        }
        similarity += i * found;
    }

    println!("PART ONE: {}", distance);
    println!("PART TWO: {}", similarity);

}

