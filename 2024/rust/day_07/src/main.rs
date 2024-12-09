use std::fs;
use itertools::Itertools;

fn combine(characters: Vec<&str>, numbers: Vec<i64>) -> Vec<String>{
    let positions = &numbers.len() -1;
    if numbers.len() == 2 {
        let mut clean_characters: Vec<String> = vec![];
        for i in characters {
            clean_characters.push(i.to_string());
        }
        return clean_characters;
    }
    // thank you stack overflow.. (https://stackoverflow.com/questions/67746583/get-all-combinations-of-a-vector-of-n-chars)
    // there is Itertools - combinations, but it is based on the array,
    // not the n:times.. therefore had no idea how to do this, but tbh
    // rust is really cool, only wish that i could have come up with 
    // this on my own...
    let combination : Vec<_> = (2..positions).fold(
        characters.iter().map(
            |c| characters.iter().map(
                move |&d| d.to_owned() + *c
            )
        ).flatten().collect(),
        |acc,_| acc.into_iter().map(
            |c| characters.iter().map(
                move |&d| d.to_owned() + &*c
            )
        ).flatten().collect()
    );
    return combination;
}

fn calculate(combination: String, numbers: Vec<i64>) -> i64 {
    let mut sum = 0;
    if combination.is_empty() {
        return numbers[0];
    } else {
        for (i, operator) in combination.chars().enumerate() {
            if operator == 'X' {
                if i == 0 {
                    sum = numbers[i] * numbers[i+1];
                } else {
                    sum = sum * numbers[i+1];
                } 
            } else if operator == '+' {
                if i == 0 {
                    sum = numbers[i] + numbers[i+1];
                } else {
                    sum = sum + numbers[i+1];
                } 
            } else {
                if i == 0 {
                    let mut first = numbers[i].to_string();
                    let second = numbers[i+1].to_string();
                    first.push_str(&second);
                    sum = first.parse::<i64>().unwrap();
                } else {
                    let mut first = sum.to_string();
                    let second = numbers[i+1].to_string();
                    first.push_str(&second);
                    sum = first.parse::<i64>().unwrap();
                }

            }
        }
    }
    return sum;
}

fn calculate_numbers(combinations: Vec<String>, value: i64 , numbers: Vec<i64>) -> usize {
    let mut correct_values = 0;
    for comb in combinations {
        let mut sum = 0;
        sum = calculate(comb, numbers.clone());

        if sum == value {
            correct_values += value;
            break;
        } 
    }
    return correct_values.try_into().unwrap();
}

fn main() {
    // Read the puzzle input
    let content = fs::read_to_string("puzzle.txt")
        .expect("Did someone remove the file?");
    let mut part_one = 0; 
    let mut part_two = 0; 
    for line in content.lines() {
        let value: i64 = line.split(": ").collect::<Vec<_>>()[0].trim().parse().unwrap();
        let numbers: Vec<i64> = line.split(": ").collect::<Vec<_>>()[1].trim().split(" ").map(
            |x| x.parse::<i64>().unwrap()
        ).collect();
        let combinations = combine(vec!["X", "+"], numbers.clone());
        part_one += calculate_numbers(combinations.clone(), value, numbers.clone());

        let combinations = combine(vec!["X", "+", "|"], numbers.clone());
        part_two += calculate_numbers(combinations, value, numbers);
    }
    println!("PART ONE: {part_one}");
    println!("PART ONE: {part_two}");
}
