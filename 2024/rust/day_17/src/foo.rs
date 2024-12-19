use std::{char, fs, usize};

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
                print!("0: {register_a:?} / 8 = ");
                register_a = register_a / 2usize.pow(combo_operand as u32);
                print!("{register_a:?}\n");
            },
            // bxl instruction
            1 => {
                // this took me some time, but finally found '^'...
                let value = register_b ^ instructions[index+1];
                println!("1: {register_b} ^ {} = {value:?}",instructions[index+1] );
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
                println!("2: {combo_operand:?} % 8 = {value:?}");
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
                print!("4: {register_b} ^ {register_c} = ");
                register_b = register_b ^ register_c;
                print!("{register_b:?}\n");
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
                println!("Value = ({combo_operand:?} % 8) = {value:?}\n");
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
        // println!("\nindex: {index:?} = {}", instructions[index]);
        // println!("register_a: {register_a:?}");
        // println!("register_b: {register_b:?}");
        // println!("register_c: {register_c:?}");
        index += 2;
    }
    let part_one: String = output.into_iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>().join(",");

    println!("\nPART ONE: {part_one:?}"); // 7,4,2,5,1,4,6,0,4


    // PART TWO
    // (0,3), (5,4), (3,0)
    //
    // 0 => A = A / 2**3
    // 5 => A % 8 OUT
    // 3 => 
    // 
    // => 0, 3, 5, 4, 3, 0
    //
    // A = 
    // A % 8 = 0
    // A = 
    // A % 8 = 3
    // A = 
    // A % 8 = 5
    // A = 
    // A % 8 = 4
    // 
    // A / 8 = 3
    // A % 8 = 3
    // 
    // A = A / 8 = 0
    // A % 8 = 0
    //
    // 8 * 0 = 0
    //

    // Reverse the instructions:
    // [0, 3, 5, 4, 3, 0]
    // => [3, 0, 5, 4, 0, 3]
    // let instructions = [2,4,1,1,7,5,1,5,4,1,5,5,0,3,3,0];
    //
    // This solution will only work for my input and only the
    // puzzle input..
    
    let instructions: Vec<usize> = Vec::from([3,0, 0,3, 5,5, 4,1, 1,5, 7,5, 1,1, 2,4]);
    let org: Vec<usize> = Vec::from([2,4,1,1,7,5,1,5,4,1,5,5,0,3,3,0]);
    
    let mut index = 0;
    let mut register_a = 0;
    let mut register_b = 0; // [0, 8, 16..]
    let mut register_c = 0;
    loop {
        let mut reg = Vec::from([register_a, register_b, register_c]);
        let mut output = Vec::new();
        if part_two(index,
            &instructions,
            &org,
            &mut reg,
            output,
            register_a,
            register_b,
            register_c) {
            println!("REGISTER: {reg:?}");
            break;
        } else {
            register_b += 8;
        }
    }
    loop {
        break;
        
        // B ^ C = [0, 8, 16..]
        //
        // 0000001001001001 = 585
        // 0000010000100100 = 1060
        //
        // 0000011001101101 = 1645
        //  
        // 0000000000000000
        // 0000000000001000 = 8
        // ---------------------
        // 0000000000000000
        // 0000000000010000 = 16
        // ---------------------
        // 0000000000010000 : 16
        // 0000000000001000 : 8
        // 0000000000000000 : 0
        

        // 2,4,1,1,7,5,1,5,4,1,5,5,0,3,3,0
        //
        // 2 => A % 8 = B
        // 1 => B ^ 1 = B
        // 7 => A / 2**B = C
        // 1 => B ^ 5 = B
        // 4 => B ^ C = B
        // 5 => B % 8 = [2,4,1,1,7,5,1,5,4,1,5,5,0,3,3,0]
        // 0 => A / 8 = A
        // 3 => 0
        //
        // B => 0 % 8 = 3
        // 
        // B => B ^ 5 = B
        // B => B ^ C = 0 ( B == C )
        // B => 0 % 8 = [0]
        //
        // A = 0
        // B = 4
        // C = 1
        // 3 => halt
        // 0 => 0 / 8 = 0 ( can be )
        // 5 => 0 % 8 = [0]
        // 4 => 1 ^ 1 = 0   === B & C needs to be the same.
        // 1 => 4 ^ 5 = 1
        // 7 => 0 / 2**5 = 0
        // 1 => B ^ 1 = 4
        // 2 => 12 % 8 = 4 (this is the one to change [4, 12, 20, 28] +8)
        //
        // A = 12
        // B = 4
        // C = 0
        // 3 =>
        // 0 => 12 / 8 = 
        // 5 => B % 8 = [3] (B= [3, ])
        // 0000000000000001 - 1
        // 0000000000000101 - B
        // 0000000000000100 - 4
        // 
        // 0000000000000101 - 5
        //
    }
}

fn part_two(
    // current index position at instuctions.
    mut index: usize,
    // vector with the instructions in reverse order.
    ins: &Vec<usize>,
    // original vector with the instructions
    org: &Vec<usize>,
    // registry vector [0..3] for a,b,c
    reg: &mut Vec<usize>,
    // output vector to push the instructions.
    mut out: Vec<usize>,
    mut r_a: usize,
    mut r_b: usize,
    mut r_c: usize) -> bool {

    println!("ORIGIN: {org:?}");
    println!("OUTPUT: {out:?}");
    println!("A: {r_a:?}");
    println!("B: {r_b:?}");
    println!("C: {r_c:?}\n");
    
    // compare if output is same as org instructions,
    // and in that case return the registry.
    if out == *org {
        reg[0] = r_a;
        reg[1] = r_b;
        reg[2] = r_c;
        return true;
    }
    if index >= org.len() {
        index = 0;
    }

    let opcode = ins[index] as usize;
    match opcode {
        0 => {
            let combo_operand = match ins[index+1] {
                4 => r_a,
                5 => r_b,
                6 => r_c,
                _ => ins[index+1]
            };
            r_a *= 2usize.pow(combo_operand as u32)
        },
        1 => r_b = r_b ^ ins[index+1],
        2 => {
            if ins[index+1] == 4 {
                loop {
                    r_a = 8 * r_b;
                    if part_two(index+2, ins, &org, reg, out.clone(), r_a, r_b, r_c) {
                        return true
                    } else {
                        r_b += 8;
                        // return false;
                    };
                }
            }
            else if ins[index+1] == 5 {
                loop {
                    r_b = 8 * r_b;
                    if part_two(index+2, ins, org, reg, out.clone(), r_a, r_b, r_c) {
                        return true
                    } else {
                        r_b += 8;
                        // return false;
                    };
                }
                
            }
            else if ins[index+1] == 6 {
                loop {
                    r_c = 8 * r_b;
                    if part_two(index+2, ins, org, reg, out.clone(), r_a, r_b, r_c) {
                        return true
                    } else {
                        r_b += 8;
                        // return false;
                    };
                }

            } else {
                if ins[index+1] % 8 != r_b {
                    return false
                }
            }
        },
        3 => index += 2,
        4 => r_b = r_b ^ r_c,
        5 => {
            if ins[index+1] == 4 {
                if org[org.len() - out.len() - 1]  == r_a % 8 {
                    out.push(r_a%8)
                } else {
                    return false
                }
            }
            else if ins[index+1] == 5 {
                if org[org.len() - out.len() - 1]  == r_b % 8 {
                    out.push(r_b%8)
                } else {
                    return false
                }
            }
            else if ins[index+1] == 6 {
                if org[org.len() - out.len() - 1]  == r_c % 8 {
                    out.push(r_c%8)
                } else {
                    return false
                }
            } else {
                if org[org.len() - out.len() - 1]  == ins[index+1] % 8 {
                    out.push(ins[index+1]%8)
                } else {
                    return false
                }
            }
        },
        6 => {
            let combo_operand = match ins[index+1] {
                4 => r_a,
                5 => r_b,
                6 => r_c,
                _ => ins[index+1]
            };
            r_b *= 2usize.pow(combo_operand as u32)
        },
        7 => {
            let combo_operand = match ins[index+1] {
                4 => r_a,
                5 => r_b,
                6 => r_c,
                _ => ins[index+1]
            };
            r_c *= 2usize.pow(combo_operand as u32)
        },
        _ => return false
    }
    if part_two(index+2, ins, org, reg, out.clone(), r_a, r_b, r_c) {
        return true
    } else {
        return false
    }
}






