use std::{collections::{HashMap, VecDeque}, fs, usize};
use regex::{Regex, RegexSet};

fn pattern_works(matches: HashMap<usize, Vec<usize>>, size: usize) -> bool {
    
    if !matches.contains_key(&0) {
        return false;
    }
    else {
        let mut queue: VecDeque<usize> = VecDeque::from(matches[&0].clone());
        let mut visited: Vec<usize> = Vec::new();
        let mut counter = 0;
        while !queue.is_empty() {
            let node = queue.pop_front().unwrap();
            if !visited.contains(&node) {
                if node == size {
                    return true;
                }
                else if matches.contains_key(&node) {
                    visited.push(node);
                    for i in matches[&node].clone() {
                        queue.push_back(i);
                    }
                }
            } 
        }
        return false;
    }
}
fn find_all(matches: HashMap<usize, Vec<usize>>, size: usize) -> usize {
    
    let mut queue: VecDeque<usize> = VecDeque::from([0]);
    let mut visited: Vec<usize> = Vec::new();
    let mut parents: HashMap<usize, Vec<usize>> = HashMap::new();
    while !queue.is_empty() {
        let node = queue.pop_front().unwrap();
        // need to first count all the parents, before
        // adding the total splits, so we don't pass
        // the splitted value to the next node, if all
        // occurences hasn't been counted yet..
        if !visited.contains(&node) && matches.contains_key(&node) {
            for n in matches[&node].clone() {
                parents.entry(n)
                    .and_modify(|x| x.push(node))
                    .or_insert(vec![node]);
                queue.push_back(n);
            }
            visited.push(node);
        }
    }
    // println!("PARENTS:");
    // for (k,v) in &parents {
    //     println!("{k:?} : {v:?}");
    // }

    let mut paths: HashMap<usize, usize> = HashMap::new();
    paths.insert(0, 1);
    // iterate through each index
    for i in 1..size + 1 {
        if parents.contains_key(&i) {
            for n in parents[&i].clone() {
                let cost = paths[&n];
                paths.entry(i)
                    .and_modify(|mut x| *x += cost)
                    .or_insert(cost);
            }
        }
    }
    // println!("\nPATHS:");
    // for (k,v) in &paths {
    //     println!("{k:?} : {v:?}");
    // }

    return paths[&size];
}

fn main() {
    
    // Read the puzzle input
    let binding = fs::read_to_string("puzzle.txt")
        .expect("Did someone remove the file?");
    let content: Vec<&str> = binding.split("\n\n").collect();
    let towel_patterns: Vec<&str> = content[0].trim().split(", ").collect();
    let towel_designs: Vec<&str> = content[1].trim().split("\n").collect();

    let mut possible: usize = 0;
    let mut possible_patterns: usize = 0;
    
    for design in &towel_designs {
        let mut counter = 0;

        // (pattern, pattern count)
        let mut matches: HashMap<usize, Vec<usize>> = HashMap::new();
        // iterate through each of the towel patterns
        for pattern in &towel_patterns {
            // find all the matched patterns: m["b", "b", "b"]
            let re = Regex::new(pattern).unwrap();
            for m in re.find_iter(&design) {
                // println!("{m:?}");
                matches.entry(m.start())
                    .and_modify(|v| v.push(m.end()))
                    .or_insert(vec![m.end()]);
            }
        }


        if pattern_works(matches.clone(), design.len()) {
            possible += 1;
            // for (k,v) in &matches {
            //     println!("{k:?}: {v:?}");
            // }
            let patterns = find_all(matches.clone(), design.len());
            println!("{patterns:?} found for {design:?}!");
            possible_patterns += patterns;
            break;
        }
    }
    println!("PART ONE: {possible:?}"); // 322
    println!("PART TWO: {possible_patterns:?}");
    // too low 17301345680744
    // too low 491870080144208
}
