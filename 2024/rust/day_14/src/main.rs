use std::{fs, usize};

#[derive(Clone)]
struct Tile {
    robots: usize,
}

fn move_robot(
    map: Vec<Vec<Tile>>,
    pos: Vec<i32>,
    vel: Vec<i32>,
    sec: i32) -> (i32, i32) {
    
    let width = map[0].len() as i32;
    let height = map.len() as i32;
    
    let move_x = pos[1] + (vel[1] * sec);
    let x = {
         if move_x % height < 0 {
           height + (move_x%height) 
        } else {
            move_x%height
        }
    };
    
    let move_y = pos[0] + (vel[0] * sec);
    let y = {
         if move_y % width < 0 {
           width + (move_y%width) 
        } else {
            move_y%width
        }
    };
    
    return (x, y)

}

fn main() {
    let content = fs::read_to_string("puzzle.txt")
        .expect("Did someone remove the file?");
    
    let space: (i32, i32) = (103, 101);
    
    let mut map: Vec<Vec<Tile>> = Vec::new();
    for _ in 0..space.0 {
        let mut row = vec![];
        for _ in 0..space.1 {
            row.push(Tile { robots:0 });
        }
        map.push(row);
    }
    
    let mut movement: Vec<Vec<Vec<i32>>> = Vec::new();
    for line in content.lines() {
        let pos_s: &str = line.split(" ").collect::<Vec<&str>>()[0];
        let vel_s: &str = line.split(" ").collect::<Vec<&str>>()[1];
        let pos: Vec<i32> = pos_s.split("p=")
            .collect::<Vec<&str>>()[1]
            .split(",")
            .map(|p| p.parse().unwrap())
            .collect();
        let vel: Vec<i32> = vel_s.split("v=")
            .collect::<Vec<&str>>()[1]
            .split(",")
            .map(|p| p.parse().unwrap())
            .collect();
        
        let new_pos: (i32, i32) = move_robot(map.clone(), pos.clone(), vel.clone(), 100 as i32);
        let x = new_pos.0 as usize;
        let y = new_pos.1 as usize;
        map[x][y].robots += 1;
        movement.push(vec![pos, vel]);
    }
    
    let mut safety_factor = 1;
    
    let mut first = 0;
    let mut second = 0;
    let mut third = 0;
    let mut fourth = 0;
    for x in 0..map.len()/2 {
        for y in 0..map[0].len()/2 {
            first += map[x][y].robots;
        }
        for y in map[0].len()/2+1..map[0].len() {
            second += map[x][y].robots;
        }
    }
    safety_factor *= first;
    safety_factor *= second;
    for x in map.len()/2+1..map.len() {
        for y in 0..map[0].len()/2 {
            third += map[x][y].robots;
        }
        for y in map[0].len()/2+1..map[0].len() {
            fourth += map[x][y].robots;
        }
    }
    safety_factor *= third;
    safety_factor *= fourth;
    println!("\nPART ONE: {safety_factor:?}"); // 229069152

    // let mut new_map: Vec<Vec<char>> = Vec::new();
    // for _ in 0..space.0 {
    //     let mut row = vec![];
    //     for _ in 0..space.1 {
    //         row.push(' ');
    //     }
    //     new_map.push(row);
    // }
    // // I found a line at iteration 9, and every 101 iteration.
    // // therefore i just tried with step 101 and found the tree
    // // at iteration 7382, but it was of course 7383.. :)
    // for i in (9..7383).step_by(101) {
    //     let mut christmas_map = new_map.clone(); 
    //     for m in &movement {
    //         let new_pos: (i32, i32) = move_robot(map.clone(), m[0].clone(), m[1].clone(), i+1 as i32);
    //         christmas_map[new_pos.0 as usize][new_pos.1 as usize] = '#';
    //     }
    //     println!("--------------------------------------------------------------------------");
    //     println!("Iteration: {i:?}");
    //     for line in christmas_map {
    //         for c in line {
    //             print!("{}", c);
    //         }
    //         println!("");
    //     }
    //     println!("\n")
    // }
}
