// Alright lets try to write a general tree data structure..
use std::{fs, ptr::null};

struct Tree {
    name: String,
    size: u64,
    parent: Box<Tree>,
    child: Vec<Box<Tree>>
}

impl Tree {
    fn new(name: String, size: u64) -> Self {
        Self {
            name,
            size,
            parent: None,
            child: vec![] 
        }
    }
    // fn add_child(&mut self, _name: String, _size: u64) {
    //     self.child.push(Self::new(_name, _size, self))
    // }
}

// fn traverse_tree(tree: &Tree) {
//     
//     println!("{}", tree.name);
//     for sub in tree.child.iter() {
//         println!("{}", sub.name);
//     }
// }



fn main() {

    // read the puzzle input
    // let commands = fs::read_to_string("puzzle.txt")
    let binding = fs::read_to_string("test.txt")
        .expect("where is the file?");
    
    let commands: Vec<Vec<&str>> = binding 
        .lines()
        .map(|x| x.split(" ").collect::<Vec<&str>>())
        .collect();
    
    let mut root = Tree::new(String::from("/"), 0, null());
    // root.add_child(String::from("a"), 30);

    for line in commands.clone() {
        // println!("{line:?}");
        // User command
        if line[0] == "$" {
            if line[1] == "cd" {
                println!("Change directory to {}", line[2]);
            } else if line[1] == "ls" {
                println!("List items in directory");
            }
            // println!("{line:?}");
        }
        else {
            println!("{line:?}");
        }

    }

    // let mut root = Tree::new(String::from("/"), 0, null());
    // root.add_child(String::from("a"), 30);
    
    // for command in commands.lines() {
    //     // Split the line 
    //     let args = command.split(" ").collect::<Vec<&str>>();
    //     // User command
    //     if args[0] == "$" {
    //         println!("{}", args[1])
    //     }
    // }


    // traverse_tree(&root);

}
