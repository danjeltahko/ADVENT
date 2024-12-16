use std::{fs, process::exit};

fn direction(pos: (usize, usize), movement: char) -> (usize, usize) {
    if movement == '^' {
        return (pos.0 - 1, pos.1);
    } else if movement == '>' {
        return (pos.0, pos.1 + 1);
    } else if movement == 'v' {
        return (pos.0 + 1, pos.1);
    } else if movement == '<' {
        return (pos.0, pos.1 - 1);
    } else {
        println!("Wrong direction??");
        exit(1);
    }
}

fn move_robot(map: &mut Vec<Vec<char>>,
    pos: (usize, usize),
    movement: char) -> bool {
    
    let (x, y) = direction(pos, movement);
    if map[x][y] == '#' {
        return false;
    } else if map[x][y] == '.' {
        map[x][y] = 'O';
        return true;
    } else {
        if move_robot(map, (x,y), movement) {
            map[x][y] = 'O';
            return true;
        } else {
            return false;
        }
    }
}

fn main() {
    // read input file to string
    let binding = fs::read_to_string("puzzle.txt")
        .expect("Did someone remove the file?");
    // split the map and move to separate lists 
    let content: Vec<&str> = binding.split("\n\n").collect();
    // extract map to vector with chars 
    let mut map: Vec<Vec<char>> = content[0]
        .split("\n")
        .collect::<Vec<&str>>()
        .into_iter()
        .map(|x| x.chars().collect())
        .collect();
    // transform move vector with strings to string
    let moves: String = content[1]
        .split("\n")
        .collect::<Vec<&str>>()
        .into_iter()
        .flat_map(|x| x.chars())
        .collect();
    // get the starting position
    let mut start_pos: (usize, usize) = (0,0);
    for x in 0..map.len() {
        for y in 0..map[0].len() {
            if map[x][y] == '@' {
                start_pos = (x,y);
                break;
            }
        }
    }
    // move the robot according to movements.
    for movement in moves.chars() {
        if move_robot(&mut map, start_pos, movement) {
            map[start_pos.0][start_pos.1] = '.';
            start_pos = direction(start_pos, movement);
            map[start_pos.0][start_pos.1] = '@';
        } else {
            continue;
        }
    }
    let mut gps = 0;
    for (x, line) in map.into_iter().enumerate() {
        for (y, c) in line.into_iter().enumerate() {
            // print!("{}", c);
            if c == 'O' {
                // println!("{}, {}", x, y);
                gps += (100 * x) + y;
            }
        }
        // println!("");
    }
    println!("PART ONE: {gps:?}"); // 1492518

}
