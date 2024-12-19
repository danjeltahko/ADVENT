use std::{fs, usize};


fn program(
    mut register_a: usize,
    mut register_b: usize,
    mut register_c: usize,
    instructions: Vec<usize>) -> Vec<usize> {

    let mut output: Vec<usize> = Vec::new();
    let mut index = 0;
    loop {
        if index >= instructions.len() {
            break;
        }
        let opcode = instructions[index];
        match opcode {
            
            // adv instruction
            0 => { 
                let combo_operand = match instructions[index+1] {
                    4 => register_a,
                    5 => register_b,
                    6 => register_c,
                    _ => instructions[index+1]
                };
                // print!("0: {register_a:?} / 8 = ");
                register_a = register_a / 2usize.pow(combo_operand as u32);
                // print!("{register_a:?}\n");
            },
            // bxl instruction
            1 => {
                // this took me some time, but finally found '^'...
                let value = register_b ^ instructions[index+1];
                // println!("1: {register_b} ^ {} = {value:?}",instructions[index+1] );
                register_b = value;
            },
            // bst instruction
            2 => {
                let combo_operand = match instructions[index+1] {
                    4 => register_a,
                    5 => register_b,
                    6 => register_c,
                    _ => instructions[index+1]
                };
                let value = combo_operand % 8;
                register_b =  value;
                // println!("2: {combo_operand:?} % 8 = {value:?}");
            },
            // jnz instruction
            3 => {
                if register_a != 0 {
                    index = instructions[index+1] as usize;
                    continue;
                }
            },
            // bxc instruction
            4 => {
                // print!("4: {register_b} ^ {register_c} = ");
                register_b = register_b ^ register_c;
                // print!("{register_b:?}\n");
            },
            // out instruction
            5 => {
                let combo_operand = match instructions[index+1] {
                    4 => register_a,
                    5 => register_b,
                    6 => register_c,
                    _ => instructions[index+1]
                };
                let value = (combo_operand % 8).try_into().unwrap();
                // println!("Value = ({combo_operand:?} % 8) = {value:?}\n");
                output.push(value);
            },
            // bdv instruction
            6 => { 
                let combo_operand = match instructions[index+1] {
                    4 => register_a,
                    5 => register_b,
                    6 => register_c,
                    _ => instructions[index+1]
                };
                register_b = register_a / 2usize.pow(combo_operand as u32);
            },
            7 => { 
                let combo_operand = match instructions[index+1] {
                    4 => register_a,
                    5 => register_b,
                    6 => register_c,
                    _ => instructions[index+1]
                };
                register_c = register_a / 2usize.pow(combo_operand as u32);
            },
            _ => println!("Something wrong..")
        }
        index += 2;
    }
    return output

}
fn main() {
    // read input file to string
    let content = fs::read_to_string("puzzle.txt")
        .expect("Did someone remove the file?");

    let mut register_a = 0 as usize;
    let mut register_b = 0 as usize;
    let mut register_c = 0 as usize;
    let mut instructions: Vec<usize> = Vec::new();
    
    for (i, line) in content.lines().enumerate() {
        if i == 0 {
            register_a = line.split(": ").collect::<Vec<&str>>()[1].trim().parse().unwrap();
        } else if i == 1 {
            register_b = line.split(": ").collect::<Vec<&str>>()[1].trim().parse().unwrap();
        } else if i == 2 {
            register_c = line.split(": ").collect::<Vec<&str>>()[1].trim().parse().unwrap();
        } else if i == 4 {
            let instruction = line.split(": ").collect::<Vec<&str>>()[1];
            for opcode in instruction.split(",") {
                instructions.push(opcode.parse().unwrap())
            }
        } else {
            continue;
        }
    }
    println!("START:");
    println!("register_a: {register_a:?}");
    println!("register_b: {register_b:?}");
    println!("register_c: {register_c:?}");
    println!("instructions: {instructions:?}");
    let output = program(register_a, register_b, register_c, instructions.clone()); 
    let part_one: String = output.into_iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>().join(",");

    println!("\nPART ONE: {part_one:?}"); // 7,4,2,5,1,4,6,0,4
    
    let mut register_a = 35184372088840; // lowest number to get same length..
    let mut index = 0;
    loop {
        let output = program(register_a, register_b, register_c, instructions.clone());
        if output == instructions {
            println!("\nPART TWO: {register_a:?}");
            println!("ins: {instructions:?}");
            println!("out: {output:?}");
            break;
        }
        if output.len() == instructions.len() {
            if output[output.len()-1] != instructions[output.len()-1] {
                register_a += 10000000000000;
            } else if output[output.len()-2] != instructions[output.len()-2] {
                register_a += 1000000000000;
            } else if output[output.len()-3] != instructions[output.len()-3] {
                register_a += 100000000000;
            } else if output[output.len()-4] != instructions[output.len()-4] {
                register_a += 10000000000;
            } else if output[output.len()-5] != instructions[output.len()-5] {
                register_a += 1000000000;
            } else if output[output.len()-6] != instructions[output.len()-6] {
                register_a += 100000000;
            } else if output[output.len()-7] != instructions[output.len()-7] {
                register_a += 10000000;
            } else if output[output.len()-8] != instructions[output.len()-8] {
                register_a += 1000000;
            } else if output[output.len()-9] != instructions[output.len()-9] {
                register_a += 100000;
            } else if output[output.len()-10] != instructions[output.len()-10] {
                register_a += 10000;
            } else if output[output.len()-11] != instructions[output.len()-11] {
                register_a += 1000;
            } else if output[output.len()-12] != instructions[output.len()-12] {
                register_a += 100;
            } else if output[output.len()-13] != instructions[output.len()-13] {
                register_a += 10;
            } else {
                register_a += 1;
            }
        } else if output.len() > instructions.len() {
            register_a -= register_a / 2
        } else if output.len() < instructions.len() {
            register_a += register_a / 2
        }
    }
}







