use std::{collections::HashMap, fs, usize};

fn rule(stones: HashMap<usize, usize>) -> HashMap<usize, usize> {
    let mut new_stones: HashMap<usize, usize> = HashMap::new();
    for (stone, v) in stones {
        let stone_string = stone.to_string();
        if stone == 0 {
            new_stones.entry(1).and_modify(| x | *x += v).or_insert(v);
        } else if stone_string.len() % 2 == 0 {
            let r = &stone_string[0..stone_string.len() / 2];
            let l = &stone_string[stone_string.len() / 2..stone_string.len()];
            new_stones.entry(r.parse().unwrap())
                .and_modify(| x | *x += v)
                .or_insert(v);
            
            new_stones.entry(l.parse().unwrap())
                .and_modify(| x | *x += v)
                .or_insert(v);


        } else {
            new_stones.entry(stone * 2024)
                .and_modify(| x | *x += v)
                .or_insert(v);
        }
    }
    return new_stones;
}

fn main() {

    // Read puzzle to vector with each stone
    let stones: Vec<usize> = fs::read_to_string("puzzle.txt")
        .expect("Did someone remove the file?")
        .trim().split(" ")
        .map(|x| x.parse().unwrap())
        .collect();

    // Part one
    let mut part_one: HashMap<usize, usize> = HashMap::new();
    for stone in stones.clone() {
        part_one.entry(stone).and_modify(| x | *x += 1).or_insert(1);
    }
    for _ in 0..25 {
        part_one = rule(part_one);
    }
    let score: usize = part_one.values().sum();
    println!("PART ONE: {score:?}"); // 200446
    
    // Part two
    let mut part_two: HashMap<usize, usize> = HashMap::new();
    for stone in stones {
        part_two.entry(stone).and_modify(| x | *x += 1).or_insert(1);
    }
    for _ in 0..75 {
        part_two = rule(part_two);
    }

    let score: usize = part_two.values().sum();
    println!("PART TWO: {score:?}"); // 238317474993392
}
