use std::{fs, usize};

fn rearrange_part_one(mut map: Vec<usize>) -> Vec<usize> {

    let mut right_most = map.len();
    for left_i in 0..map.len() {
        if map[left_i] == 0{
            for right_i in (0..right_most).rev() {
                if map[right_i] != 0 && right_i > left_i {
                    map.swap(left_i, right_i);
                    break;
                }
                right_most = right_i;
            }
        }
        if left_i > right_most {
            break;
        }
    }
    return map;
}

fn get_free_space(map: &Vec<usize>, stop: usize) -> Vec<Vec<usize>> {
    
    let mut free_space = Vec::new();
    let mut indexes = Vec::new();
    for i in 0..stop + 2 {
        // add indexes of the empty space to the left
        if map[i] == 0 {
            indexes.push(i);
        } else {
            if !indexes.is_empty() {
                free_space.push(indexes.clone());
                indexes.clear();
            }
        }
    }
    return free_space;
}

fn rearrange_part_two(mut map: Vec<usize>) -> Vec<usize> {

    let mut current_id: i64 = -1;
    let mut blocks = Vec::new(); 
    // iterate backwards to get all the ID blocks
    for right_i in (0..map.len()).rev() {
        // set the id for the block first time.
        if current_id == -1 { current_id = map[right_i] as i64 };
        // push all the id's for the block.
        if map[right_i] as i64 == current_id {
            blocks.push(map[right_i]);
        }  
        else {
            // add the block where it fits
            if !blocks.is_empty() {
                let free_space = get_free_space(&map, right_i);
                if free_space.is_empty() { break; }
                for space_block in &free_space {
                    if space_block.len() >= blocks.len() {
                        for (i, value) in ((right_i +1)..((right_i +1) + blocks.len())).enumerate() {
                            map.swap(space_block[i], value);
                        }
                        break;
                    }
                }
            }
            // clear and set new id for block.
            blocks.clear();
            if map[right_i] != 0 {
                current_id = map[right_i] as i64;
                blocks.push(map[right_i]);
            } 
        }

    }
    return map;
}

fn calculate(map: Vec<usize>) -> usize {
    let mut sum = 0;
    for (i, block) in map.into_iter().enumerate() {
        if block == 0 {
            continue;
        }
        sum += i * (block - 1);
    }
    return sum;
}

fn main() {
    let content = fs::read_to_string("puzzle.txt")
        .expect("Did someone remove the file?");
    
    let mut disk_map = Vec::new();
    let mut id = 1;
    for (i, foo) in content.trim().chars().enumerate() {
        if i % 2 == 0 {
            for _ in 0..foo.to_digit(10).expect("Not a digit") {
                disk_map.push(id);
            }
            id += 1;
        } else {
            for _ in 0..foo.to_digit(10).expect("Not a digit") {
                disk_map.push(0);
            }
        }
    }
    let disk_map_one = rearrange_part_one(disk_map.clone());
    let checksum = calculate(disk_map_one.clone());
    println!("PART ONE : {checksum:?}"); // 6446899523367
    
    let disk_map_two = rearrange_part_two(disk_map);
    let checksum = calculate(disk_map_two.clone());
    println!("PART TWO : {checksum:?}"); // 6478232739671
}

