use std::fs;

fn main() {
    let s = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file");

    let operations: Vec<&str> = s.split('\n').collect();

    part1(operations.clone());

    part2();    
}

fn part2() {
    
}

fn part1(operations: Vec<&str>) {
    let mut acc = 0;
    let mut ip = 0;
    
    
    let mut operations_with_history: Vec<(&str, bool)> = vec![];
    for operations in operations {
        operations_with_history.push((operations, false));
    }

    loop {
        let instruction = operations_with_history[ip];
        if instruction.1 {
            // we've been here before
            println!("acc: {}", acc);
            break;
        } else {
            operations_with_history[ip].1 = true;
        }
        let instruction:(&str, &str, usize) = split_instruction(instruction.0);
        let op = instruction.0;
        let dir = instruction.1;
        let val = instruction.2;
        match op {
            "acc" => {
                match dir {
                    "+" => {
                        acc += val
                    },
                    "-" => {
                        acc -= val
                    },
                    _   => ()
                } 
            },
            "jmp" => {
                match dir {
                    "+" => {
                        ip += val;
                        continue;
                    },
                    "-" => {
                        ip -= val;
                        continue;
                    }
                    _   => ()
                }
            },
            "nop" => (),
            _     => ()
        }
        ip += 1;
    }
}

fn split_instruction(instruction: &str) -> (&str, &str, usize) {
    // parse the string to get
    // op
    // direction (+ or -) if not nop
    // value if not nop
    let t: Vec<&str> = instruction.split(' ').collect();
    let first = t[0];
    let second_third = t[1].split_at(1);
    let second = second_third.0;
    let third = second_third.1.parse::<usize>().unwrap();

    (first, second, third)
}