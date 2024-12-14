use std::{char, collections::HashMap, fs, process::exit, usize};
use itertools::Itertools;

#[derive(Debug)]
#[derive(Clone)]
struct Plot {
    garden: char,
    checked: bool,
    start: bool,
    count: usize
}

fn add_perimeter(garden: char, map: &Vec<Vec<Plot>>, x:usize, y:usize) -> usize {
    let mut perimeter = 0;
    let length = map.clone().len() - 1;
    // outside of map to west
    if y as i16 - 1 < 0 {
        perimeter += 1;
    } else if map[x][y-1].garden != garden {
        perimeter += 1;
    }
    // outside of map to east 
    if y + 1 > length {
        perimeter += 1;
    } else if map[x][y+1].garden != garden {
        perimeter += 1;
    }
    // outside of map to north
    if x as i16 - 1 < 0 {
        perimeter += 1;
    } else if map[x-1][y].garden != garden {
        perimeter += 1;
    }
    // outside of map to south 
    if x + 1 > length {
        perimeter += 1;
    } else if map[x+1][y].garden != garden {
        perimeter += 1;
    }
    return perimeter
}

fn check_side(i:&usize, x:&usize, v:&Vec<usize>, sides:&mut usize, same:&mut bool) {
    if *i == 0 {
        *sides += 1;
        *same = true;
    } else if v[i-1] != x - 1{
        *sides += 1;
        *same = true;
    } else if *same == false {
        *sides += 1;
        *same = true;
    }
}

fn inside_of_map(map: &mut Vec<Vec<Plot>>, garden: char, x: i32, y: i32) -> bool {
    let length = (map.len() - 1) as i32;
    if 0 <= x && x <= length && 0 <= y && y <= length {
        if map[x as usize][y as usize].garden == garden {
            return true;
        } else {
            return false;
        }
    } else {
        return false;
    }
}

fn traverse(map: &mut Vec<Vec<Plot>>,
    garden: char,
    x:usize,
    y:usize,
    mut row:HashMap<usize, Vec<usize>>,
    mut col:HashMap<usize, Vec<usize>>,
    direction: char) ->
    (HashMap<usize, Vec<usize>>, HashMap<usize, Vec<usize>>) {

    // return all edges if position is start and checked.
    if map[x][y].checked && map[x][y].start {
        // in case the garden is a +
        map[x][y].count += 1;
        if map[x][y].count == 4 {
            return (row, col);
        }
    }
    // Check for single points..
    else if map[x][y].start &&
        !inside_of_map(map, garden, x as i32-1, y as i32) &&
        !inside_of_map(map, garden, x as i32, y as i32+1) &&
        !inside_of_map(map, garden, x as i32+1, y as i32) &&
        !inside_of_map(map, garden, x as i32, y as i32-1) {
        row.entry(x).and_modify(|cols| if !cols.contains(&y) {cols.push(y)}).or_insert(vec![y]);
        col.entry(y).and_modify(|rows| if !rows.contains(&x) {rows.push(x)}).or_insert(vec![x]);
        map[x][y].checked = true;
        return (row, col);
    }

    if direction == '^'{
        // Check if UP is inside of map and same garden..
        if inside_of_map(map, garden, x as i32 -1, y as i32) {
            map[x][y].checked = true;
            row.entry(x).and_modify(|cols| if !cols.contains(&y) {cols.push(y)}).or_insert(vec![y]);
            col.entry(y).and_modify(|rows| if !rows.contains(&x) {rows.push(x)}).or_insert(vec![x]);
            return traverse(map, garden, x-1, y, row, col, '<')
        } else {
            return traverse(map, garden, x, y, row, col, '>')
        }
    } else if direction == '>' {
        // Check if RIGHT is inside of map and same garden..
        if inside_of_map(map, garden, x as i32, y as i32 +1) {
            map[x][y].checked = true;
            row.entry(x).and_modify(|cols| if !cols.contains(&y) {cols.push(y)}).or_insert(vec![y]);
            col.entry(y).and_modify(|rows| if !rows.contains(&x) {rows.push(x)}).or_insert(vec![x]);
            return traverse(map, garden, x, y+1, row, col, '^')
        } else {
            return traverse(map, garden, x, y, row, col, 'v')
        }
    } else if direction == 'v' {
        // Check if DOWN is inside of map and same garden..
        if inside_of_map(map, garden, x as i32 +1, y as i32) {
            map[x][y].checked = true;
            row.entry(x).and_modify(|cols| if !cols.contains(&y) {cols.push(y)}).or_insert(vec![y]);
            col.entry(y).and_modify(|rows| if !rows.contains(&x) {rows.push(x)}).or_insert(vec![x]);
            return traverse(map, garden, x+1, y, row, col, '>')
        } else {
            return traverse(map, garden, x, y, row, col, '<')
        }
    } else if direction == '<' {
        // Check if LEFT is inside of map and same garden..
        if inside_of_map(map, garden, x as i32, y as i32 -1) {
            map[x][y].checked = true;
            row.entry(x).and_modify(|cols| if !cols.contains(&y) {cols.push(y)}).or_insert(vec![y]);
            col.entry(y).and_modify(|rows| if !rows.contains(&x) {rows.push(x)}).or_insert(vec![x]);
            return traverse(map, garden, x, y-1, row, col, 'v')
        } else {
            return traverse(map, garden, x, y, row, col, '^')
        }
    } else {
        println!("Something went wrong!!");
        exit(1);
    }
}

fn get_price(garden: char,
    map:&mut Vec<Vec<Plot>>,
    row:&HashMap<usize, Vec<usize>>,
    col:&HashMap<usize, Vec<usize>>) -> (usize, usize) {

    let mut filled_row = HashMap::<usize, Vec<usize>>::new();
    let mut filled_col = HashMap::<usize, Vec<usize>>::new();
    let mut area = 0;
    let mut perimeter = 0;
    // Iterate through each row in sorted order
    for x in row.keys().sorted().cloned() {
        let cols: Vec<usize> = row[&x].clone().into_iter().sorted().collect();
        let x_min: usize = *cols.iter().min().unwrap();
        let x_max: usize = *cols.iter().max().unwrap();
        // iterate through the min & max column on the row
        for y in x_min..x_max + 1 {
            // Check so the col is inside of area..
            let rows: Vec<usize> = col[&y].clone().into_iter().sorted().collect();
            let y_min: usize = *rows.iter().min().unwrap();
            let y_max: usize = *rows.iter().max().unwrap();
            // Getting the complete AREA
            if x >= y_min && x <= y_max && map[x][y].garden == garden {
                map[x][y].checked = true;
                area += 1;
                filled_row.entry(x)
                    .and_modify(|cols| if !cols.contains(&y) {cols.push(y.clone())})
                    .or_insert(vec![y.clone()]);
                filled_col.entry(y.clone())
                    .and_modify(|rows| if !rows.contains(&x) {rows.push(x.clone())})
                    .or_insert(vec![x.clone()]);
                perimeter += add_perimeter(garden, &map, x, y);
            } 
        }
    }
    let mut sides = 0;
    for x in filled_row.keys().sorted() {
        let row_v = filled_row[x].clone().into_iter().sorted().collect_vec();
        let mut top_same = false;
        let mut bot_same = false;
        for (i, y) in row_v.clone().into_iter().enumerate() {
            let col_for_row = filled_col[&y].clone().into_iter().sorted().collect_vec();
            // check if garden exists above or beneath
            // ABOVE:
            if *x == 0 {
                check_side(&i, &y, &row_v, &mut sides, &mut top_same);
            } else if !col_for_row.contains(&(x-1)) {
                check_side(&i, &y, &row_v, &mut sides, &mut top_same);
            } else { top_same = false;}
            // BELOW:
            if !col_for_row.contains(&(x+1)) {
                check_side(&i, &y, &row_v, &mut sides, &mut bot_same);
            } else { bot_same = false; }
        }
    }
    for y in filled_col.keys().sorted() {
        let col_v = filled_col[y].clone().into_iter().sorted().collect_vec();
        let mut top_same = false;
        let mut bot_same = false;
        for (i, x) in col_v.clone().into_iter().enumerate() {
            let row_for_col = filled_row[&x].clone().into_iter().sorted().collect_vec();
            // check if garden exists above or beneath
            // ABOVE:
            if *y == 0 {
                check_side(&i, &x, &col_v, &mut sides, &mut top_same);
            } else if !row_for_col.contains(&(y-1)) {
                check_side(&i, &x, &col_v, &mut sides, &mut top_same);
            } else { top_same = false;}
            // BELOW:
            if !row_for_col.contains(&(y+1)) {
                check_side(&i, &x, &col_v, &mut sides, &mut bot_same);
            } else { bot_same = false; }
        }
    }
    return ((area*perimeter), (area*sides));
}

fn main() {
    // alright, im starting to get a hang on this i think..
    let mut map: Vec<Vec<Plot>> = fs::read_to_string("puzzle.txt")
        .expect("Did someone remove the file?")
        .lines()
        .map(|x| x.chars()
            .map(|c| Plot {
                garden:c,
                checked:false,
                start:false,
                count: 0
            })
            .collect())
        .collect();

    
    let mut price_one = 0;
    let mut price_two = 0;
    for x in 0..map.len() {
        for y in 0..map.len() {
            if map[x][y].checked {
                continue;
            } else {
                let mut row = HashMap::<usize, Vec<usize>>::new();
                let mut col = HashMap::<usize, Vec<usize>>::new();
                let garden = map[x][y].garden;
                map[x][y].start = true;
                println!("Starting to traverse {} at ({x:?}, {y:?})", map[x][y].garden);
                (row, col) = traverse(&mut map, garden, x, y, row, col, '^');
                let (p1, p2) = get_price(garden, &mut map, &row, &col);
                price_one += p1;
                price_two += p2;
            }
        }
    }
    println!("\nPART ONE: {price_one:?}"); // 1451030
    println!("PART TWO: {price_two:?}"); // 859494
}
