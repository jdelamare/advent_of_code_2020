use std::fs;

fn main() {
    // Get the input lines
    let v = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file");

    let v: Vec<String> = v.lines().filter_map(|s| s.trim().parse().ok()).collect();
    
    part_1(&v);
    part_2(&v);
}

fn part_1(v: &Vec<String>) {
    let total_passwords = v.len();
    let mut bad_passwords = 0;

    for line in v {
        let mut iter = line.split(|c| c == '-');
        let lower = iter.next().unwrap();
        let lower:u8 = lower.parse().unwrap();
        let upper = iter.next().unwrap().split(|c| c == ' ').next().unwrap();
        let upper:u8 = upper.parse().unwrap();
        let iter = line.split(|c| c == ':').next().unwrap();
        let req = iter.split(|c| c == ' ' ).last().unwrap();
        let req:char = req.parse().unwrap();
        let password = line.split(|c| c == ':').last().unwrap().trim();
        
        let mut count = 0;
        for c in password.chars() {
            if c == req {
                count += 1;    
            }
        }
        if count > upper || count < lower {
            bad_passwords += 1;
        }
    }

    println!("There are {} valid passwords.", total_passwords - bad_passwords)
}

fn part_2(v: &Vec<String>) {
    let total_passwords = v.len();
    let mut bad_passwords = 0;

    let mut i = 0;
    for line in v {
        let mut iter = line.split(|c| c == '-');
        let first_posn = iter.next().unwrap();
        let first_posn:usize = first_posn.parse().unwrap();
        let second_posn = iter.next().unwrap().split(|c| c == ' ').next().unwrap();
        let second_posn:usize = second_posn.parse().unwrap(); 
        let iter = line.split(|c| c == ':').next().unwrap();
        let req = iter.split(|c| c == ' ' ).last().unwrap();
        let req:char = req.parse().unwrap();
        let mut password = line.split(|c| c == ':').last().unwrap().trim().chars();
        let mut p2 = password.clone(); 
        let left = password.nth(first_posn - 1).unwrap();
        let right = p2.nth(second_posn - 1).unwrap();
        if left == req && right != req {
            // good
        } else if left != req && right == req {
            // good
        } else {
            bad_passwords += 1;
        }
    }

    println!("There are {} valid passwords.", total_passwords - bad_passwords) 
}