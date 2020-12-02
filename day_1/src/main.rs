use std::collections::HashMap;
use std::fs;

fn main() {

    let target: u32 = 2020;
    
    // retrieve the data
    let v = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file");

    let v: Vec<u32> = v.lines().filter_map(|s| s.trim().parse().ok()).collect();

    let result = two_sum(v, target);

    match result{
        Some(x) => println!("Solution: {}", x.0 * x.1),
        None => println!("No solution was found")
    }
}

fn two_sum(data: Vec<u32>, target: u32) -> Option<(u32, u32)>{

    let mut hm = HashMap::new();

    // keys are our data values that we care about, value doesn't matter
    // we can look backwards and cease iterating if we find a sum. we
    // know that there is only one combination which sums to the target.
    for (i, e) in data.iter().enumerate() {
        let complement: u32 = target - e;
        if hm.contains_key(&complement){
            return Some((*e, complement));
        }
        hm.insert(e, i);
    }

    None
}

// Unfortunately, this is not my idea.
// https://leetcode.com/problems/two-sum/solution/
// https://stackoverflow.com/questions/28193463/how-to-simplify-parsing-a-text-file-to-a-vector-of-values