use std::{collections::HashMap, fs, usize};

fn rule(stones: Vec<usize>) -> Vec<usize> {
    let mut new_stones = Vec::new();
    for stone in stones {
        let stone_string = stone.to_string();
        if stone == 0 {
            new_stones.push(1);
        } else if stone_string.len() % 2 == 0 {
            let r = &stone_string[0..stone_string.len() / 2];
            let l = &stone_string[stone_string.len() / 2..stone_string.len()];
            new_stones.push(r.parse().unwrap());
            new_stones.push(l.parse().unwrap());


        } else {
            new_stones.push(stone * 2024)
        }
    }
    return new_stones;
}

fn rule_two(stones: HashMap<usize, usize>) -> HashMap<usize, usize> {
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
    let content = fs::read_to_string("puzzle.txt")
        .expect("Did someone remove the file?");

    let mut stones_one: Vec<usize> = content.clone().trim().split(" ")
        .map(|x| x.parse().unwrap()).collect();
    
    for _ in 0..25 {
        stones_one = rule(stones_one);
    }
    println!("PART ONE: {}", stones_one.len()); // 200446
    

    let mut stones_two: Vec<usize> = content.clone().trim().split(" ")
        .map(|x| x.parse().unwrap()).collect();

    let mut stones: HashMap<usize, usize> = HashMap::new();
    for stone in stones_two {
        stones.entry(stone).and_modify(| x | *x += 1).or_insert(1);
    }
    for _ in 0..75 {
        stones = rule_two(stones);
    }

    let part_two: usize = stones.values().sum();

    println!("PART TWO: {part_two:?}");
}
