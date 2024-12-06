use std::fs;

#[derive(Clone)]
struct Position {
    symbol: char,
    counter: usize,
    in_loop: bool,
}

impl Position {
    fn add(&mut self, i: usize) {
        self.counter += i;
        self.in_loop = true;
    }
    fn check(&mut self) -> bool {
        if self.counter >= 2 {
            return true;
        } else {
            return false;
        }
    }
}

// x-y positions for moving depending on guards face
fn move_instruction(guard_pos: char, pos: (i32, i32)) -> (i32, i32) {
    if guard_pos == '>' {
        return (pos.0, pos.1+1);
    } else if guard_pos == '<' {
        return (pos.0, pos.1-1);
    } else if guard_pos == '^' {
        return (pos.0-1, pos.1);
    } else {
        return (pos.0+1, pos.1);
    }
}

// Check if the position is movable
fn possible_to_move(map: Vec<Vec<Position>>, guard_pos: char, pos: (i32, i32), map_len: &usize) -> bool {
    let instruction: (i32, i32) = move_instruction(guard_pos, pos);
    let i_x = instruction.0 as usize;
    let i_y = instruction.1 as usize;
    if instruction.0 < 0 || i_x >= *map_len || instruction.1 < 0 || i_y >= *map_len {
        return false;
    } else {
        if map[i_x][i_y].symbol != '#' && map[i_x][i_y].symbol != 'O' {
            return true;
        } else {
            return false;
        }
    }
}


fn move_on_map(map: &mut Vec<Vec<Position>>, pos: &(i32, i32), loop_counter: &mut usize, part: usize) -> (Vec<Vec<Position>>, bool) {
    
    // Get move instruction depending on guards face.
    let x: usize = pos.0 as usize;
    let y: usize = pos.1 as usize;
    let position_char = map[x][y].symbol;
    let mut instruction: (i32, i32) = move_instruction(map[x][y].symbol, *pos);
    let i_x = instruction.0 as usize;
    let i_y = instruction.1 as usize;
    if instruction.0 < 0 || i_x >= map.len().try_into().unwrap() || instruction.1 < 0 || i_y >= map.len().try_into().unwrap() {
        map[x][y].symbol = 'X';
        return (map.to_vec(), false);

    } else if map[i_x][i_y].symbol == '#' || map[i_x][i_y].symbol == 'O' {

        if map[x][y].symbol == '+' && map[x][y].check() {
            println!("Check the loop");
        }
        
        // if the obstacle is the one we put there, add 1 to loop counter
        if position_char == '>' {
            // if there is an obstacle to the right:
            // check if guard is able to move down,
            // set guard character down one step.
            if possible_to_move(map.to_vec(), 'v', *pos, &map.len()) {
                instruction = move_instruction('v', *pos);
                if part == 2{
                    if map[x][y].in_loop == true {
                        println!("In a loop");
                        return (map.to_vec(), true);
                    }
                    map[x][y].symbol = '+';
                    map[x][y].add(1);
                } else {
                    map[x][y].symbol = 'X';
                }
                map[instruction.0 as usize][instruction.1 as usize].symbol = 'v';
                return move_on_map(map, &instruction, loop_counter, part);
            } else {
                map[x][y].symbol = 'v';
            }
        } else if position_char == '<' {
            if possible_to_move(map.to_vec(), '^', *pos, &map.len()) {
                instruction = move_instruction('^', *pos);
                if part == 2{
                    if map[x][y].in_loop {
                        println!("In a loop");
                        return (map.to_vec(), true);
                    }
                    map[x][y].symbol = '+';
                    map[x][y].add(1);
                } else {
                    map[x][y].symbol = 'X';
                }
                map[instruction.0 as usize][instruction.1 as usize].symbol = '^';
                return move_on_map(map, &instruction, loop_counter, part);
            } else {
                map[x][y].symbol = '^';
            }
        } else if position_char == '^' {
            if possible_to_move(map.to_vec(), '>', *pos, &map.len()) {
                instruction = move_instruction('>', *pos);
                if part == 2{
                    if map[x][y].in_loop {
                        println!("In a loop");
                        return (map.to_vec(), true);
                    }
                    map[x][y].symbol = '+';
                    map[x][y].add(1);
                } else {
                    map[x][y].symbol = 'X';
                }

                map[instruction.0 as usize][instruction.1 as usize].symbol = '>';
                return move_on_map(map, &instruction, loop_counter, part);
            } else {
                map[x][y].symbol = '>';
            }
        } else {
            if possible_to_move(map.to_vec(), '<', *pos, &map.len()) {
                instruction = move_instruction('<', *pos);
                if part == 2{
                    if map[x][y].in_loop {
                        println!("In a loop");
                        return (map.to_vec(), true);
                    }
                    map[x][y].symbol = '+';
                    map[x][y].add(1);
                } else {
                    map[x][y].symbol = 'X';
                }

                map[instruction.0 as usize][instruction.1 as usize].symbol = '<';
                return move_on_map(map, &instruction, loop_counter, part);
            } else {
                map[x][y].symbol = '<';
            }
        }
        return move_on_map(map, &pos, loop_counter, part);
    }
    else {
        if part == 2 {
            if position_char == '>' {
                map[x][y].symbol = '-';
            } else if position_char == '<' {
                map[x][y].symbol = '-';
            } else if position_char == '^' {
                map[x][y].symbol = '|';
            } else {
                map[x][y].symbol = '|';
            }
        } else {
            map[x][y].symbol = 'X';

        }
        // Change the current pos to X
        // Add 1 to internal counter
        // Move guard to direction
        map[i_x][i_y].symbol = position_char;
        return move_on_map(map, &instruction, loop_counter, part);
    }
}

fn main() {
    // Read the puzzle input
    let content = fs::read_to_string("puzzle.txt")
        .expect("Did someone remove the file?");
    
    let mut map: Vec<Vec<Position>> = Vec::new();
    let mut guard_start_pos: (i32, i32) = (0,0);
    for (x, line) in content.lines().enumerate() {
        let mut row: Vec<Position> = Vec::new();
        for (y, c) in line.chars().enumerate() {
            row.push(Position {
                symbol: c,
                counter: 0,
                in_loop: false
            });
            if "^<>v".contains(c) {
                guard_start_pos = (
                    x.try_into().unwrap(),
                    y.try_into().unwrap()
                );
            }
        }
        map.push(row);
    }
    // Part One
    let part_one_map = move_on_map(&mut map.clone(), &guard_start_pos, &mut 0, 1).0;
    let mut distinct_positions = 0;
    for line in &part_one_map {
        for pos in line {
            if pos.symbol == 'X' {
                distinct_positions += 1;
            }
        }
    }
    println!("PART ONE: {distinct_positions}");
    // Part Two
    let mut looped = 0;
    for x in 0..map.len() {
        for y in 0..map.len() {
            if part_one_map[x][y].symbol == 'X' {
                println!("[{x}][{y}]");
                let mut obstacle_map = map.clone();
                obstacle_map[x][y].symbol = 'O';
                // println!("Cloned map");
                if move_on_map(&mut obstacle_map, &guard_start_pos,&mut 0, 2).1 {
                    println!("Found one loop");
                    looped += 1;
                }
            }
        }
    }
    println!("PART TWO: {looped}");

}
