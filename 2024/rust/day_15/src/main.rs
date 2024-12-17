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

fn move_all_boxes(map: &mut Vec<Vec<char>>,
    boxes: Vec<Vec<(usize, usize)>>,
    movement: char) {
    
    for b in &boxes {
        // Reset pos for the box that can be moved
        map[b[0].0][b[0].1] = '.';
        map[b[1].0][b[1].1] = '.';
    }
    for b in boxes {
        
        if movement == '^'{
            // Move the box up
            map[b[0].0-1][b[0].1] = '[';
            map[b[1].0-1][b[1].1] = ']';
        } else {
            // Move the box up
            map[b[0].0+1][b[0].1] = '[';
            map[b[1].0+1][b[1].1] = ']';
        }
    }
}

fn check_boxes(map: &mut Vec<Vec<char>>,
    boxes: &mut Vec<Vec<(usize, usize)>>,
    left: (usize, usize),
    right: (usize, usize),
    movement: char) -> bool {

    boxes.push(vec![left, right]);
    
    // up/down from left side of box 
    let (lx, ly) = direction(left, movement);
    let (rx, ry) = direction(right, movement);
    let mut right = false;
    let mut left = false;

    if map[lx][ly] == '.' && map[rx][ry] == '.' {
        return true;
    }

    if map[rx][ry] == '[' {
        right = check_boxes(map, boxes, (rx, ry), (rx, ry+1), movement)
    }
    else if map[rx][ry] == ']' {
        right = check_boxes(map, boxes, (rx, ry-1), (rx, ry), movement)
    }
    else if map[rx][ry] == '.' {
        right = true;
    } else {
        right = false;
    }

    if map[lx][ly] == '[' {
        left = check_boxes(map, boxes, (lx, ly), (lx, ly+1), movement)
    }
    else if map[lx][ly] == ']' {
        left = check_boxes(map, boxes, (lx, ly-1), (lx, ly), movement)
    }
    else if map[lx][ly] == '.' {
        left = true;
    } else {
        left = false;
    }

    if right && left {
        return true;
    } else {
        boxes.clear();
        return false;
    }

}

fn move_bigger_robot(map: &mut Vec<Vec<char>>,
    pos: (usize, usize),
    movement: char) -> bool {
    let (x, y) = direction(pos, movement);
    if map[x][y] == '#' {
        return false;
    }

    else if map[x][y] == '[' {
        if movement == '<' || movement == '>' {
            if move_bigger_robot(map, (x,y), movement) {
                map[x][y] = ']';
                return true;
            } else {
                return false;
            }
        }
        else if movement == '^' || movement == 'v'{
            let mut boxes = Vec::new();
            if check_boxes(map, &mut boxes, (x,y), (x,y+1), movement) {
                move_all_boxes(map, boxes, movement);
                map[x][y+1] = '.';
                return true;
            } else {
                return false;
            }
        } else {
            return false;
        }
    }

    else if map[x][y] == ']' {
        if movement == '<' || movement == '>' {
            if move_bigger_robot(map, (x,y), movement) {
                map[x][y] = '[';
                return true;
            } else {
                return false;
            }
        }
        else if movement == '^' || movement == 'v' {
            let mut boxes = Vec::new();
            if check_boxes(map, &mut boxes, (x,y-1), (x,y), movement) {
                move_all_boxes(map, boxes, movement);
                map[x][y-1] = '.';
                return true;
            } else {
                return false;
            }
        } else {
            return false;
        }
    }
    // if going right and left.
    else {
        if movement == '<' {
            map[x][y] = '['
        } else if movement == '>' {
            map[x][y] = ']'
            
        }
        return true;
    }
}

fn get_start_position(map: &Vec<Vec<char>>) -> (usize, usize) {
    for x in 0..map.len() {
        for y in 0..map[0].len() {
            if map[x][y] == '@' {
                return (x,y);
            }
        }
    }
    return (0,0)
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

    // create a clone of map, but bigger for part 2
    let mut bigger_map: Vec<Vec<char>> = Vec::new();
    for line in map.clone() {
        let mut row: Vec<char> = Vec::new();
        for c in line {
            if c == '#' {
                row.push('#');
                row.push('#');
            } else if c == '.' {
                row.push('.');
                row.push('.');
            } else if c == 'O' {
                row.push('[');
                row.push(']');
            } else {
                row.push('@');
                row.push('.');
            }
        }
        bigger_map.push(row);
    }

    // transform move vector with strings to string
    let moves: String = content[1]
        .split("\n")
        .collect::<Vec<&str>>()
        .into_iter()
        .flat_map(|x| x.chars())
        .collect();
    
    // PART ONE
    let mut start_pos: (usize, usize) = get_start_position(&map);
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
            if c == 'O' {
                gps += (100 * x) + y;
            }
        }
    }
    println!("PART ONE: {gps:?}"); // 1492518

    // Part Two
    let mut start_pos: (usize, usize) = get_start_position(&bigger_map);
    // move the robot according to movements.
    for movement in moves.chars() {
        if move_bigger_robot(&mut bigger_map, start_pos, movement) {
            bigger_map[start_pos.0][start_pos.1] = '.';
            start_pos = direction(start_pos, movement);
            bigger_map[start_pos.0][start_pos.1] = '@';
        } else {
            continue;
        }
    }

    let mut gps = 0;
    for (x, line) in bigger_map.into_iter().enumerate() {
        for (y, c) in line.into_iter().enumerate() {
            if c == '[' {
                gps += (100 * x) + y;
            }
        }
    }
    println!("PART TWO: {gps:?}"); // 1512860
}
