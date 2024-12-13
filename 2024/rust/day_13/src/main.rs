use std::{collections::HashMap, fs, usize};

fn calculate_moves(arcade: HashMap<&str, Vec<usize>>, part: usize) -> usize {
    println!("\n{arcade:?}");
    // A = 3 tokens
    // B = 1 tokens
    // {"prize": ["8400", "5400"], "A": ["94", "34"], "B": ["22", "67"]}
    let px: i64; 
    let py: i64;
    if part == 1 {
        px = arcade["prize"][0] as i64;
        py = arcade["prize"][1] as i64;
    } else {
        px = (arcade["prize"][0] + 10000000000000) as i64;
        py = (arcade["prize"][1] + 10000000000000) as i64;
    }

    let b1 = arcade["B"][0] as i64;
    let b2 = arcade["B"][1] as i64;
    let a1 = arcade["A"][0] as i64;
    let a2 = arcade["A"][1] as i64;

    // System of Linear Equations
    // { A1x + B1y = Px } = { 94x + 22y = 8400 }
    // { A2x + B2y = Py } = { 34x + 67y = 5400 }
    //
    // --------------------------------------------------
    // 94x + 22y = 8400
    // 34x + 67y = 5400
    //
    // 34(94x + 22y) = 34(8400) => 3196x + 748y = 285600
    // 94(34x + 67y) = 94(5400) => 3196x + 6298y = 507600
    //
    // (3196x + 6298y) - (3196x + 748y) = 507600 - 285600
    // 5550y = 222000
    //
    // y = 222000/5550 = 40
    // --------------------------------------------------
    //
    // A2(A1x + B1y) = A2(Px)
    // A1(A2x + B2y) = A1(Py)
    // 
    // ((A1*A2x) + (A1*B2y)) - ((A2*A1x) + (A2*B1y)) = A1(Py) - A2(Px)
    // y((A1*B2y) - (A2*B1y)) = A1(Py) - A2(Px)
    //
    // y = A1(Py) - A2(Px) / ((A1*B2) - (A2*B1))
    //
    let y = ((a1*py)-(a2*px))/((a1*b2)-(a2*b1));
    // 
    // 94x + (22 * 40) = 8400
    // 94x = 8400 - (22 * 40)
    // x = 8400 - (22 * 40) / 94 = 80
    let x = (px - (b1 * y)) / a1;
    if ((b1*y)+(a1*x)) == px &&
        ((b2*y)+(a2*x)) == py {
            return ((y*1) + (x*3)) as usize;
    } else {
        return 0;
    }
}

fn main() {
    // alright, im starting to get a hang on this i think..
    let content = fs::read_to_string("puzzle.txt")
        .expect("Did someone remove the file?");
    
    let mut arcade: HashMap<&str, Vec<usize>> = HashMap::new();
    let mut total_tokens_one = 0;
    let mut total_tokens_two = 0;
    for line in content.lines() {
        if line.is_empty() {
            continue;
        } else {
            // splitting the string: ["Button A", "X+94, Y+34"]
            let splitted_line: Vec<&str> = line.split(": ").collect();
            // println!("{splitted_line:?}");
            // Get X & Y for buttons
            if splitted_line[0].contains("Button") {
                let button = splitted_line[0]
                    .split(" ")
                    .collect::<Vec<&str>>()[1];
                let config = splitted_line[1]
                    .split(", ")
                    .map(|x| x
                        .split("+")
                        .collect::<Vec<&str>>()[1])
                    .map(|d| d.parse().unwrap())
                    .collect::<Vec<usize>>();
                *arcade.entry(button)
                    .or_insert(config.clone()) = config.clone();
            } else {
                // Get the score for the Prize
                let prize = splitted_line[1]
                    .split(", ")
                    .map(|x| x
                        .split("=")
                        .collect::<Vec<&str>>()[1])
                    .map(|d| d.parse().unwrap())
                    .collect::<Vec<usize>>();
                *arcade.entry("prize")
                    .or_insert(prize.clone()) = prize.clone();
                total_tokens_one += calculate_moves(arcade.clone(), 1);
                total_tokens_two += calculate_moves(arcade.clone(), 2);
            }
        }
    }
    println!("PART ONE: {total_tokens_one:?}"); // 36250
    println!("PART TWO: {total_tokens_two:?}"); // 83232379451012
}
