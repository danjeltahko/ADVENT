use std::fs;


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

// For Part one i was able to use recursion but i get
// stack overflow when trying to do it with part 2
fn move_on_map(mut obstacle_map: Box<Vec<Vec<char>>>, pos: (i32, i32), d: (usize, usize)) -> (Vec<Vec<char>>, usize) {
    
    let mut x: usize = pos.0 as usize;
    let mut y: usize = pos.1 as usize;
    let mut position_char = obstacle_map[x][y];
    let mut loop_counter = 0;
    let mut debug = false;
    if d == (9, 58) {
        debug = true;
    }
    loop {
        
        // position to move index
        if debug {
            for line in obstacle_map.iter() {
                for c in line {
                    print!("{c}");
                }
                println!("")
            }
            println!("\n--------------------------\n");
        }
        let instruction: (i32, i32) = move_instruction(obstacle_map[x][y], (x.try_into().unwrap(), y.try_into().unwrap()));
        let i_x = instruction.0 as usize;
        let i_y = instruction.1 as usize;
        
        // if position is moves guard to outisde of map, break. 
        if instruction.0 < 0 || i_x >= obstacle_map.len().try_into().unwrap() || instruction.1 < 0 || i_y >= obstacle_map.len().try_into().unwrap() {
            obstacle_map[x][y] = 'X';
            break;

        }
        // if guard is able to move, check if obstacle is front of guard
        else if obstacle_map[i_x][i_y] == '#' || obstacle_map[i_x][i_y] == 'O' {
            // if the obstacle is the one we put there, add 1 to loop counter
            if obstacle_map[i_x][i_y] == 'O' {
                loop_counter += 1;
                // break if we have looped twice.
                if loop_counter >= 2 {
                    return (*obstacle_map, loop_counter);
                }
            }
            // Change the direction of guard.
            if position_char == '>' {
                obstacle_map[x][y] = 'v';
                position_char = obstacle_map[x][y];
            } else if position_char == '<' {
                obstacle_map[x][y] = '^';
                position_char = obstacle_map[x][y];
            } else if position_char == '^' {
                obstacle_map[x][y] = '>';
                position_char = obstacle_map[x][y];
            } else {
                obstacle_map[x][y] = '<';
                position_char = obstacle_map[x][y];
            }
            continue;
        }
        // Move the guard one step according to instructions.
        else {
            // Get the direction char at pos
            // Change the current pos to X
            // position_char = obstacle_map[x][y];
            obstacle_map[x][y] = 'X';
            // Move guard to direction
            obstacle_map[i_x][i_y] = position_char;
            x = i_x;
            y = i_y;
            // Move again..
            // println!("moved guard = {instruction:?} - {loop_counter:?}");
            continue;
        }
    }
    return (*obstacle_map, loop_counter);

}

fn main() {
    // unsafe { backtrace_on_stack_overflow::enable() };
    // Read the puzzle input
    let content = fs::read_to_string("puzzle.txt")
        .expect("Did someone remove the file?");
    
    let mut map: Vec<Vec<char>> = Vec::new();
    let mut guard_start_pos: (i32, i32) = (0,0);
    for (x, line) in content.lines().enumerate() {
        let mut row: Vec<char> = Vec::new();
        for (y, c) in line.chars().enumerate() {
            row.push(c);
            if "^<>v".contains(c) {
                guard_start_pos = (
                    x.try_into().unwrap(),
                    y.try_into().unwrap()
                );
            }
        }
        map.push(row);
    }
    // // Part One
    let part_one_map = move_on_map(Box::new(map.clone()), guard_start_pos, (0,0)).0;
    let mut distinct_positions = 0;
    for line in &part_one_map {
        for pos in line {
            if *pos == 'X' {
                distinct_positions += 1;
            }
        }
    }
    println!("PART ONE: {distinct_positions}");


    // Part Two
    let mut looped = 0;
    // println!("{part_one_map:?}");
    for i in 0..map.len() {
        for j in 0..map.len() {
            if part_one_map[i][j] == 'X' {
                println!("[{i}][{j}]");
                // Map with obstacle
                let mut obstacle_map = Box::new(map.clone());
                obstacle_map[i][j] = 'O';
                if move_on_map(obstacle_map, guard_start_pos, (i,j)).1 >= 2 {
                    looped += 1;
                }
            }
        }
    }
    println!("PART TWO: {looped}");
    // 20, 28
    // 20, 46
    // 13, 73

}
