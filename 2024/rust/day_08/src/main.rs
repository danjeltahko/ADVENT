use std::{collections::HashMap, fs};


fn part_one(antennas: HashMap<char, Vec<(i32, i32)>>, mut matrix: Vec<Vec<char>>)  -> usize {

    for (k, antenna) in &antennas {
        for compare in 0..antenna.len() - 1 {
            for i in compare..antenna.len() - 1 {

                // setting variables
                let x_compare = antenna[compare as usize].0;
                let y_compare = antenna[compare as usize].1;
                let x_next = antenna[i as usize + 1].0;
                let y_next = antenna[i as usize + 1].1;
                // setting position diff
                let x = (x_compare - x_next).abs();
                let y = (y_compare - y_next).abs();

                if x_compare <= x_next && y_compare <= y_next {
                    let frequency_x = x_compare - x;
                    let frequency_y = y_compare - y;
                    if frequency_x >= 0 && frequency_y >= 0 {
                        matrix[frequency_x as usize][frequency_y as usize] = '#';
                    } 
                    let frequency_x = x_next + x;
                    let frequency_y = y_next + y;
                    if frequency_x <= (matrix.len() - 1).try_into().unwrap() && frequency_y <= (matrix.len() - 1).try_into().unwrap() {
                        matrix[frequency_x as usize][frequency_y as usize] = '#';

                        
                    } 

                } else if x_compare <= x_next && y_compare >= y_next {
                    let frequency_x = x_compare - x;
                    let frequency_y = y_compare + y;
                    if frequency_x >= 0 && frequency_y <= (matrix.len() - 1).try_into().unwrap() {
                        matrix[frequency_x as usize][frequency_y as usize] = '#';
                    } 

                    let frequency_x = x_next + x;
                    let frequency_y = y_next - y;
                    if frequency_x <= (matrix.len() - 1).try_into().unwrap() && frequency_y >= 0 {
                        matrix[frequency_x as usize][frequency_y as usize] = '#';
                    } 
                }
            }
        }
    }
    // would be nice if would understand these fully..
    return matrix.iter().map(|row| row.into_iter().filter(|c| *c == &'#').count()).sum();
}

fn part_two(antennas: HashMap<char, Vec<(i32, i32)>>, mut matrix: Vec<Vec<char>>)  -> usize {
    let mut frequencies = 0;
    for (k, antenna) in &antennas {
        for compare in 0..antenna.len() - 1 {
            for i in compare..antenna.len() - 1 {
                // setting variables
                let x_compare = antenna[compare as usize].0;
                let y_compare = antenna[compare as usize].1;
                let x_next = antenna[i as usize + 1].0;
                let y_next = antenna[i as usize + 1].1;
                // setting position diff
                let x = (x_compare - x_next).abs();
                let y = (y_compare - y_next).abs();

                if x_compare <= x_next && y_compare <= y_next {
                    let mut frequency_x = x_compare - x;
                    let mut frequency_y = y_compare - y;
                    matrix[x_compare as usize][y_compare as usize] = '#';
                    loop {
                        if frequency_x >= 0 && frequency_y >= 0 {
                            matrix[frequency_x as usize][frequency_y as usize] = '#';
                            frequency_x = frequency_x - x;
                            frequency_y = frequency_y - y;
                        } else {
                            break;
                        }
                    }

                    let mut frequency_x = x_next + x;
                    let mut frequency_y = y_next + y;
                    matrix[x_next as usize][y_next as usize] = '#';
                    loop {
                        if frequency_x <= (matrix.len() - 1).try_into().unwrap() && frequency_y <= (matrix.len() - 1).try_into().unwrap() {
                            matrix[frequency_x as usize][frequency_y as usize] = '#';
                            frequency_x = frequency_x + x;
                            frequency_y = frequency_y + y;
                        } else {
                            break;
                        }
                    }

                } else if x_compare <= x_next && y_compare >= y_next {
                    let mut frequency_x = x_compare - x;
                    let mut frequency_y = y_compare + y;
                    matrix[x_compare as usize][y_compare as usize] = '#';
                    loop {
                        if frequency_x >= 0 && frequency_y <= (matrix.len() - 1).try_into().unwrap() {
                            matrix[frequency_x as usize][frequency_y as usize] = '#';
                            frequency_x = frequency_x - x;
                            frequency_y = frequency_y + y;
                        } else {
                            break;
                        }
                    }

                    let mut frequency_x = x_next + x;
                    let mut frequency_y = y_next - y;
                    matrix[x_next as usize][y_next as usize] = '#';
                    loop {
                        if frequency_x <= (matrix.len() - 1).try_into().unwrap() && frequency_y >= 0 {
                            matrix[frequency_x as usize][frequency_y as usize] = '#';
                            frequency_x = frequency_x + x;
                            frequency_y = frequency_y - y;
                        } else {
                            break;
                        }
                    }
                }
            }
        }
    }
    let sum : usize = matrix.iter().map(|row| row.into_iter().filter(|c| *c == &'#').count()).sum();
    return sum;
}

fn main() {
    let content = fs::read_to_string("puzzle.txt")
        .expect("Did someone remove the file?");
    
    // Get the posistion of all the antennas.
    let mut antennas: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
    let mut matrix : Vec<Vec<char>> = Vec::new();
    for (row, line) in content.lines().enumerate() {
        let mut r = Vec::new();
        for (col, c) in line.chars().enumerate() {
            r.push(c);
            if c != '.' {
                antennas.entry(c)
                    .and_modify(| antenna | antenna.push((
                                row.try_into().unwrap(),
                                col.try_into().unwrap()
                    )))
                    .or_insert(Vec::from(vec![(
                                row.try_into().unwrap(),
                                col.try_into().unwrap())
                    ]));
            }
        }
        matrix.push(r);
    }
    
    // Add frequency to antennas.
    let part_one = part_one(antennas.clone(), matrix.clone());
    let part_two = part_two(antennas.clone(), matrix.clone());

    println!("PART ONE : {part_one}"); // 394
    println!("PART TWO : {part_two}"); // 1277
}
