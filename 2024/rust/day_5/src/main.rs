use std::{collections::HashMap, fs};

fn sort_it(manual: &mut Vec<i32>, order: &HashMap<i32, Vec<i32>>) -> i32 {
    let mut is_correct = true;
    for i in 0..manual.len() {
        if i == 0 {
            continue;
        } else {
            // check if the value exists as a key
            // in the ordering
            if order.contains_key(&manual[i]) {
                // get the order key values to
                // check if the values that should
                // be produced after is in case before.
                let not_before:Vec<i32> = order.get(&manual[i]).unwrap().to_vec();
                // iterate backwards to check it.
                for v in (0..i).rev() {
                    // incorrect oreder
                    if not_before.contains(&manual[v]) {
                        manual.swap(i, v);
                        is_correct = false;
                    } 
                }
            }
        }
    }
    if is_correct {
        return manual[manual.len()/2];
    } else {
        return sort_it(manual, order)
    }
}
fn main() {
    // Read the puzzle input
    let content = fs::read_to_string("puzzle.txt")
        .expect("Did someone remove the file?");
    
    let mut order: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut ordering_done = false;
    let mut middle_pages = 0;
    let mut middle_incorrect_pages = 0;
    for line in content.lines() {
        // go through the produce manuel, is it correct?
        if ordering_done {
            let mut produce_manual: Vec<i32> = line.split(",").map(
                |x| x.parse::<i32>().unwrap()).collect(); 
            // iterate thorugh the produce array
            // and check backwards if each value
            // is in correct order according to
            // the ordering.
            let mut correct_order = true;
            for (i, value) in produce_manual.iter().enumerate() {
                if i == 0 {
                    continue;
                } else {
                    // check if the value exists as a key
                    // in the ordering
                    if order.contains_key(value) {
                        // get the order key values to
                        // check if the values that should
                        // be produced after is in case before.
                        let not_before:Vec<i32> = order.get(value).unwrap().to_vec();
                        // iterate backwards to check it.
                        for v in (0..i).rev() {
                            // incorrect oreder
                            if not_before.contains(&produce_manual[v]) {
                                correct_order = false;
                            } 
                        }
                    }
                }
            } if correct_order {
                middle_pages += produce_manual[produce_manual.len() / 2];
            } else {
                // sort the incorrect orders.
                middle_incorrect_pages += sort_it(&mut produce_manual, &order)
            }

        }
        // add all the page orderings to a dict.
        else {
            // if whitespace, move on to next step.
            if line.is_empty() {
                ordering_done = true;
            } else {
                // splitting the ordering [47][53]
                let order_numbers: Vec<i32> = line
                    .split("|")
                    .map(|x| x.parse::<i32>().unwrap())
                    .collect(); 
                // add entry to hashmap with key and either
                // push value to vec or create new vec.
                order.entry(order_numbers[0])
                    .and_modify(|orders| orders.push(order_numbers[1]))
                    .or_insert(Vec::from(vec![order_numbers[1]]));
            }
        }
    }
    println!("PART ONE: {}", middle_pages); // 6034
    println!("PART TWO: {}", middle_incorrect_pages); // 6305
}
