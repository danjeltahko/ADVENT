use std::{collections::LinkedList, fs};


fn signal_system(buffer: &str, length: usize) -> usize {
    // initialize linked list for easier FIFO
    let mut subroutine: LinkedList<char> = LinkedList::new();
    let mut marker = 0;
    // iterate through all the characters in puzzle
    for (i, c) in buffer.chars().enumerate() {
        // check for stop marker in linked list
        // when we have 4 characters in the list.
        if subroutine.len() == length {
            let mut stop_marker = true;
            // iterate through all the characters
            // to see if all characters are unique.
            // and if they are, break and set marker
            // to index value (markers point)
            for check in subroutine.iter() {
                let mut counter = 0;
                for item in subroutine.iter() {
                    if check == item {
                        counter += 1;
                    }
                } 
                if counter > 1 {
                    stop_marker = false;
                }
            }
            if stop_marker {
                marker = i;
                break;
            }
            subroutine.pop_front();
        } 
        subroutine.push_back(c);
    }
    return marker
}

fn main() {
    
    // read the puzzle input
    let buffer = fs::read_to_string("puzzle.txt")
        .expect("where is the file?");
    
    let start_of_packet = signal_system(&buffer, 4);
    let start_of_message = signal_system(&buffer, 14);
    
    println!("PART ONE: {start_of_packet}");
    println!("PART TWO: {start_of_message}");
}
