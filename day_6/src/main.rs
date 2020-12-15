use std::fs;
use bitvec::prelude::*;

fn main() {

    let s = fs::read_to_string("input.txt")
    .expect("Something went wrong reading the file");

    let all_answers: Vec<&str> = s.split("\n\n").collect();

    part1(all_answers.clone());

    part2(all_answers);
}


fn part1(all_answers: Vec<&str>) {
    
    let mut total = 0;
    for answer in all_answers {
        let mut bv = bitvec![Lsb0, u32; 0; 26];
        for c in answer.chars() {
            match c.to_string().as_str() {
                "a" => bv.set(0, true),
                "b" => bv.set(1, true),
                "c" => bv.set(2, true),
                "d" => bv.set(3, true),
                "e" => bv.set(4, true),
                "f" => bv.set(5, true),
                "g" => bv.set(6, true),
                "h" => bv.set(7, true),
                "i" => bv.set(8, true),
                "j" => bv.set(9, true),
                "k" => bv.set(10, true),
                "l" => bv.set(11, true),
                "m" => bv.set(12, true),
                "n" => bv.set(13, true),
                "o" => bv.set(14, true),
                "p" => bv.set(15, true),
                "q" => bv.set(16, true),
                "r" => bv.set(17, true),
                "s" => bv.set(18, true),
                "t" => bv.set(19, true),
                "u" => bv.set(20, true),
                "v" => bv.set(21, true),
                "w" => bv.set(22, true),
                "x" => bv.set(23, true),
                "y" => bv.set(24, true),
                "z" => bv.set(25, true),
                _ => ()
            } 
        }
        total += bv.iter().filter(|b| **b == true).fold(0, |acc, _| acc + 1);
    }

    println!("Part 1: {}", total);
}


fn part2(all_answers: Vec<&str>) {
    
    let mut total = 0;
    for group in all_answers {
        let mut init: bool = false;
        let mut result_bv = bitvec![Lsb0, u32; 0; 26];
        let answers: Vec<&str> = group.split('\n').collect();
        for answer in answers {
            let mut bv = bitvec![Lsb0, u32; 0; 26];
            for c in answer.chars() {
                match c.to_string().as_str() {
                    "a" => bv.set(0, true),
                    "b" => bv.set(1, true),
                    "c" => bv.set(2, true),
                    "d" => bv.set(3, true),
                    "e" => bv.set(4, true),
                    "f" => bv.set(5, true),
                    "g" => bv.set(6, true),
                    "h" => bv.set(7, true),
                    "i" => bv.set(8, true),
                    "j" => bv.set(9, true),
                    "k" => bv.set(10, true),
                    "l" => bv.set(11, true),
                    "m" => bv.set(12, true),
                    "n" => bv.set(13, true),
                    "o" => bv.set(14, true),
                    "p" => bv.set(15, true),
                    "q" => bv.set(16, true),
                    "r" => bv.set(17, true),
                    "s" => bv.set(18, true),
                    "t" => bv.set(19, true),
                    "u" => bv.set(20, true),
                    "v" => bv.set(21, true),
                    "w" => bv.set(22, true),
                    "x" => bv.set(23, true),
                    "y" => bv.set(24, true),
                    "z" => bv.set(25, true),
                    _ => ()
                } 
            }
            if !init {
                result_bv |= bv.clone();
                init = true;
            }
            result_bv &= bv;
        }
        total += result_bv.iter().filter(|b| **b == true).fold(0, |acc, _| acc + 1);
    }
    
    println!("Part 2: {:?}", total);
}