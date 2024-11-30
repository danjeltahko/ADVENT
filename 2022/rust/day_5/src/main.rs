use std::{char, collections::HashMap, fs};

fn create_stacks(stacks: &Vec<&str>) -> HashMap<char, Vec<char>> {
    
    // Creates the puzzle stack with all the containers
    let mut map: HashMap<char, Vec<char>> = HashMap::new();
    // Find all the stacks
    for stack in stacks[stacks.len() - 1].chars() {
        if !stack.is_whitespace() {
            map.insert(stack, Vec::new());
        } 
    }
    // Reverse iterate through each line
    for i in (0..(stacks.len() -1)).rev() {
        // Clean the line of whitespace to only include chars
        // [W]         [W][G][V][D][G][C]
        // [W][-][-][-][W][G][V][D][G][C]
        let clean_line = stacks[i]
            .replace("    ", "[-]")
            .replace(" ", "")
            .replace("[", "")
            .replace("]", "");
        // iterate through the line to add correct container to stack 
        for (index, c) in clean_line.chars().enumerate() {
            if c != '-' {
                let key = char::from_digit((index+1) as u32, 10).unwrap();
                let v = map.get_mut(&key).unwrap();
                v.push(c);
            }
            
        }
    }
    return map
}

fn moving_schedule(line: &str) -> (u8, u8, u8){

    let mut order = Vec::<u8>::new();
    // Extract the integers from the puzzle string
    // to get move, to and from.
    for c in line.split(" ") {
        let number: u8 = match c.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        order.push(number);
    }
    // Return tuple with order schedule.
    return (order[0], order[1], order[2])
}

fn cratemover9000(orders: &Vec<&str>, mut stack: HashMap<char, Vec<char>>) -> HashMap<char, Vec<char>> {
    // Iterate through all the lines with orders
    for line in orders {
        // Get the scheduled tuple
        let (mov, from, to) = moving_schedule(line);
        // Set the u8 values to characters.
        let from_char = (from + b'0') as char;
        let to_char = (to + b'0') as char;
        // Move the amount of containers according to schedule
        for _ in 0..mov {
            let container = stack.get_mut(&from_char).unwrap().pop().unwrap();
            stack.get_mut(&to_char).unwrap().push(container);
        }
    }
    return stack
}

fn cratemover9001(orders: &Vec<&str>, mut stack: HashMap<char, Vec<char>>) -> HashMap<char, Vec<char>> {
    // In the second part of the puzzle we see that it's a newer
    // version of the CrateMover (9001) which can pick up multiple
    // containers in one go, so instead of first in last out,
    // we need to move all the container in one go..


    // Iterate through all the lines with orders
    for line in orders {
        // Get the scheduled tuple
        let (mov, from, to) = moving_schedule(line);
        // Set the u8 values to characters.
        let from_char = (from + b'0') as char;
        let to_char = (to + b'0') as char;
        // Move the amount of containers according to schedule
        let mut crane = Vec::new();
        for _ in 0..mov {
            let container = stack.get_mut(&from_char).unwrap().pop().unwrap();
            crane.push(container);
        }
        for container in crane.iter().rev() {
            stack.get_mut(&to_char).unwrap().push(*container);
        }
    }
    return stack
    
}

fn get_puzzle_answer(mut stack: HashMap<char, Vec<char>>) -> String{

    let mut message = String::new();
    
    // Iterate through the hashmap to get the last CHAR from each vector.
    for i in 0..stack.len() {
        let key = char::from_digit((i+1) as u32, 10).unwrap();
        message.insert(i, stack.get_mut(&key).unwrap().pop().unwrap());
    }

    return message
}

fn main() {

    let contents = fs::read_to_string("puzzle.txt")
        .expect("Something went wrong reading the file.");

    let mut orders: Vec<&str> = Vec::new();
    
    // Read the first lines until whitespace
    // to be able to create the puzzle stack
    let mut stacks: Vec<&str> = Vec::new();
    let mut order_flag = false;
    for line in contents.lines() {
        if order_flag {
            orders.push(line)
        } else {
            if line.is_empty() {
                order_flag = true;
            } else {
                stacks.push(line);
            }
        } 
    }
    let stack_one: HashMap<char, Vec<char>> = create_stacks(&stacks);
    let stack_two: HashMap<char, Vec<char>> = create_stacks(&stacks);

    let part_one = cratemover9000(&orders, stack_one);
    let part_two = cratemover9001(&orders, stack_two);
    
    let part_one_answer = get_puzzle_answer(part_one);
    let part_two_answer = get_puzzle_answer(part_two);

    println!("PART ONE: {}", part_one_answer);
    println!("PART TWO: {}", part_two_answer);
}
