use std::fs;

// Lowercase item types a through z have priorities 1 through 26.
// Uppercase item types A through Z have priorities 27 through 52.

fn main() {

    let contents = fs::read_to_string("puzzle.txt")
        .expect("Something went wrong reading the file");

    let mut prio = Vec::new();
    let mut groups = Vec::new();

    // Get each unique item in the group
    //
    let mut counter = 0;
    let mut group = Vec::new();
    for line in contents.lines() {

        let first_half = &line[0..line.len() / 2];
        let secon_half = &line[line.len() / 2..line.len()];
        
        for i in first_half.chars() {
            if secon_half.contains(i) {
                prio.push(i);
                break
            }
        }
        group.push(line);
        counter += 1;
        if counter == 3 {
            groups.push(group);
            group = Vec::new();
            counter = 0;
        }
    }

    let first_score = get_score(prio);
    prio = Vec::new();
    println!("First Score: {}", first_score);
    
    for group in groups {
        for item in group[0].chars() {
            if group[1].contains(item) && group[2].contains(item) {
                prio.push(item);
                break
            }
        }
    }

    let second_score = get_score(prio);
    println!("Second Score: {}", second_score);
    
}

fn get_score(items: Vec<char>) -> u16 {
    let mut score = 0;
    for i in items {
        if i.is_uppercase() {
            // println!("{} - {}",i, i as u16 - 38);
            score += i as u16 - 38;
        } else {
            // println!("{} - {}",i, i as u16 - 96);
            score += i as u16 - 96;
        }
    }
    score
}
