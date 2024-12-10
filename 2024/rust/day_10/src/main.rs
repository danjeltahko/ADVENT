use std::{fs, usize};

fn hike_path(map: &Vec<Vec<u32>>, mut trailheads: Vec<(usize, usize)>, x: usize, y: usize, part: usize) -> Vec<(usize, usize)> {
    
    if map[x][y] == 9 {
        if part == 1 && !trailheads.contains(&(x,y)) {
            trailheads.push((x,y));
        } else if part == 2 {
            trailheads.push((x,y));
        }
        return trailheads;
    }
    // East
    if y + 1 < map.len() {
        if map[x][y] + 1 == map[x][y+1] {
            trailheads = hike_path(&map, trailheads, x, y+1, part)
        }
    }
    // South
    if x + 1 < map.len() {
        if map[x][y] + 1 == map[x+1][y] {
            trailheads = hike_path(&map, trailheads, x+1, y, part)
        }
    }
    // West 
    if y as i16 - 1 >= 0 {
        if map[x][y] + 1 == map[x][y-1] {
            trailheads = hike_path(&map, trailheads, x, y-1, part)
        }
    }
    // North 
    if x as i16 - 1 >= 0 {
        if map[x][y] + 1 == map[x-1][y] {
            trailheads = hike_path(&map, trailheads, x-1, y, part)
        }
    }
    return trailheads 
}

fn main() {
    let content = fs::read_to_string("puzzle.txt")
        .expect("Did someone remove the file?");
   
    // would love to actually do this as one-liner
    let mut start_positions: Vec<(usize, usize)> = Vec::new();
    let mut map: Vec<Vec<u32>> = Vec::new();
    for (x, line) in content.lines().enumerate() {
        let mut row: Vec<u32> = Vec::new();
        for (y, c) in line.chars().enumerate() {
            let number = c.to_digit(10);
            if (number.unwrap() as usize) == 0 {
                start_positions.push((x, y))
            }
            row.push(c.to_digit(10).unwrap())
        }
        map.push(row);
    }
    let mut score_part_one = 0;
    let mut score_part_two = 0;
    for start in start_positions {
        let mut trailheads: Vec<(usize, usize)> = Vec::new();
        trailheads = hike_path(&map, trailheads, start.0, start.1, 1);
        score_part_one += trailheads.len();
        
        let mut trailheads: Vec<(usize, usize)> = Vec::new();
        trailheads = hike_path(&map, trailheads, start.0, start.1, 2);
        score_part_two += trailheads.len();
    }
    println!("PART ONE: {score_part_one}"); // 531
    println!("PART TWO: {score_part_two}"); // 1210
}
