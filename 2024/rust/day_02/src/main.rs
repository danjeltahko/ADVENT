use std::fs;

fn rule(x: &i32, y: &i32) -> bool {

    if (x - y).abs() > 3 || (x - y).abs() < 1 {
        return false
    } else {
        return true
    }
}

fn is_safe(report: &mut Vec<i32>) -> bool {
    let mut decreasing_errors = 0;
    for i in 1..report.len() {
        if report[i-1] > report[i] && rule(&report[i-1], &report[i]) {
            continue;
        } else {
            decreasing_errors += 1;
        }
    } if decreasing_errors == 0 {
        return true;
    }
    let mut increasing_errors = 0;
    for i in 1..report.len() {
        if report[i-1] < report[i] && rule(&report[i-1], &report[i]) {
            continue;
        } else {
            increasing_errors += 1;
        }
    } if increasing_errors == 0 {
        return true;
    } 
    return false
}

fn main() {
    // Read the puzzle input
    let data = fs::read_to_string("puzzle.txt")
        .expect("Did someone remove the file?");
    
    let mut safe_reports_one = 0;
    let mut safe_reports_two = 0;
    for report in data.lines() {
        let mut report_vec: Vec<i32> = report.split(" ").map(|x| x.parse::<i32>().unwrap()).collect(); 
        let report_len = report_vec.clone().len();
        if is_safe(&mut report_vec) {safe_reports_one += 1;} 
        if is_safe(&mut report_vec) {safe_reports_two += 1;} else {
            for i in 0..report_vec.len() {
                let mut report_clone = report_vec.clone();
                report_clone.remove(i);
                if is_safe(&mut report_clone) {safe_reports_two += 1; break;}

            }
        }

    }

    println!("PART ONE {}", safe_reports_one); // 670
    println!("PART TWO {}", safe_reports_two); // 700
}
