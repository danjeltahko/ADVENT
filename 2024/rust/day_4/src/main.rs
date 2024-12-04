use std::fs;
use regex::Regex;

fn check_row(word_search: &Vec<Vec<char>>, xmas: &Regex) -> usize {
    let mut has_xmas = 0;
    for i in 0..word_search.len() {
        let joined: String = word_search[i].iter().collect();
        let reversed: String = word_search[i].iter().rev().collect();
        has_xmas += xmas.find_iter(&joined).count();
        has_xmas += xmas.find_iter(&reversed).count();
    }
    return has_xmas;
}
fn check_column(word_search: &Vec<Vec<char>>, xmas: &Regex) -> usize {
    let mut has_xmas = 0;
    for col in 0..word_search.len() {
        let mut vertical: Vec<char> = Vec::new();
        for row in 0..word_search.len() {
            vertical.push(word_search[row][col]);

        }
        let joined: String = vertical.iter().collect();
        let reversed: String = vertical.iter().rev().collect();
        has_xmas += xmas.find_iter(&joined).count();
        has_xmas += xmas.find_iter(&reversed).count();
    }
    return has_xmas;
}

fn check_right_diagonal(word_search: &Vec<Vec<char>>, xmas: &Regex) -> usize {
    let mut has_xmas = 0;
    for row in 0..word_search.len() {
        let mut diagonal: Vec<char> = Vec::new();
        if row < 3 {
            continue;
        } else {
            // [3][0], [2][1], [1][2], [0][3]
            // [9][1], [8][2], [7][3], [6][4]
            let mut dec = row.clone();
            for col in 0..row+1 {
                diagonal.push(word_search[dec][col]);
                if dec != 0 {dec -= 1;}
            }
        }
        let joined: String = diagonal.iter().collect();
        let reversed: String = diagonal.iter().rev().collect();
        has_xmas += xmas.find_iter(&joined).count();
        has_xmas += xmas.find_iter(&reversed).count();
    }
    for col in 1..word_search.len() {
        let mut diagonal: Vec<char> = Vec::new();
        if (word_search.len()-1 - col) < 3 {
            continue;
        } else {
            // [9][1], [8][2], [7][3], [6][4]
            // [9][6], [8][7], [7][8], [6][9]
            let mut dec = col.clone();
            for row in (col..word_search.len()).rev() {
                diagonal.push(word_search[row][dec]);
                if dec != word_search.len() -1 {dec += 1;}
            }
        }
        let joined: String = diagonal.iter().collect();
        let reversed: String = diagonal.iter().rev().collect();
        has_xmas += xmas.find_iter(&joined).count();
        has_xmas += xmas.find_iter(&reversed).count();
    }
    return has_xmas;
}

fn check_left_diagonal(word_search: &Vec<Vec<char>>, xmas: &Regex) -> usize {
    let mut has_xmas = 0;
    for col in (0..word_search.len()).rev() {
        let mut diagonal: Vec<char> = Vec::new();
        if (word_search.len()-1 - col) < 3 {
            continue;
        } else {
            let mut dec = col.clone();
            for row in 0..(word_search.len()-col) {
                diagonal.push(word_search[row][dec]);
                if dec != word_search.len() {dec += 1;}
            }
        }
        let joined: String = diagonal.iter().collect();
        let reversed: String = diagonal.iter().rev().collect();
        has_xmas += xmas.find_iter(&joined).count();
        has_xmas += xmas.find_iter(&reversed).count();
    }
    for row in 1..word_search.len() {
        let mut diagonal: Vec<char> = Vec::new();
        if (word_search.len()-1 - row )< 3 {
            continue;
        } else {
            let mut dec = row.clone();
            for col in 0..(word_search.len()-row) {
                diagonal.push(word_search[dec][col]);
                if dec != word_search.len() {dec += 1;}
            }
        }
        let joined: String = diagonal.iter().collect();
        let reversed: String = diagonal.iter().rev().collect();
        has_xmas += xmas.find_iter(&joined).count();
        has_xmas += xmas.find_iter(&reversed).count();
    }
    return has_xmas;
}

fn it_is_x(word_search: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
    if (x + 2) >= word_search.len() || ((y as i16) - 2) < 0{
        // println!("Failed");
        return false;
    } else {
        // Is there a better way to this? i have no idea
        let matching: String = Vec::from([word_search[x][y-2], word_search[x+1][y-1], word_search[x+2][y]]).iter().collect();
        if matching == "MAS" || matching == "SAM" {
            return true;
        } else {
            return false;
        }
    }
}

fn check_mas(word_search: &Vec<Vec<char>>) -> usize {
    let mut has_mas = 0;
    // first half of diagonal
    for row in 0..word_search.len() {
        if row < 3 {
            continue;
        } else {
            let mut x = String::new();
            let mut dec = row.clone();
            for col in 0..row+1 {
                x.push(word_search[dec][col]);
                if x.len() == 3 && x == "MAS" || x == "SAM" {
                    // println!("Found MAS on [{}][{}]", dec, col);
                    if it_is_x(&word_search, dec, col) {
                        has_mas += 1;
                    }
                    x.remove(0);
                } else if x.len() == 3 {
                    x.remove(0);
                }
                if dec != 0 {dec -= 1;}
            }
        }
    }
    for col in 1..word_search.len() {
        if (word_search.len()-1 - col) < 3 {
            continue;
        } else {
            let mut x = String::new();
            let mut dec = col.clone();
            for row in (col..word_search.len()).rev() {
                x.push(word_search[row][dec]);
                if x.len() == 3 && x == "MAS" || x == "SAM" {
                    // println!("Found MAS on [{}][{}]", row, dec);
                    if it_is_x(&word_search, row, dec) {
                        has_mas += 1;
                    }
                    x.remove(0);
                } else if x.len() == 3 {
                    x.remove(0);
                }
                if dec != word_search.len() -1 {dec += 1;}
            }
        }
    }
    return has_mas;
}
fn main() {

    // Read the puzzle input
    let word_search_raw = fs::read_to_string("puzzle.txt")
        .expect("Did someone remove the file?");


    let xmas = Regex::new(r"XMAS").unwrap();
    let mut has_xmas = 0;
    let mut has_mas = 0;
    let mut word_search = Vec::new();
    // iterate through each line in puzzle
    for line in word_search_raw.lines(){
        // iterate through each character
        // and add it to a 'row'
        let mut row: Vec<char> = Vec::new();
        for c in line.chars() {
            row.push(c);
        }
        word_search.push(row.clone());
        println!("{row:?}")
    }
    has_xmas += check_row(&word_search, &xmas);
    has_xmas += check_column(&word_search, &xmas);
    has_xmas += check_right_diagonal(&word_search, &xmas);
    has_xmas += check_left_diagonal(&word_search, &xmas);

    has_mas += check_mas(&word_search);

    println!("PART ONE: {has_xmas}"); // 2500
    println!("PART TWO: {has_mas}"); // 1933

}
