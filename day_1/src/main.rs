use std::collections::HashMap;
use std::fs;

fn main() {

    let target: u32 = 2020;
    
    // retrieve the data
    let v = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file");

    let mut v: Vec<u32> = v.lines().filter_map(|s| s.trim().parse().ok()).collect();

    let result = two_sum(v.clone(), target);


    match result {
        Some(x) => println!("Solution to part 1: {}", x.0 * x.1),
        None => println!("No solution was found")
    }

    let result = three_sum(&mut v, target);

    match result {
        Some(x) => println!("Solution to part 2: {}", x.0 * x.1 * x.2),
        None => println!("No solution was found")
    }
}


fn two_sum(v: Vec<u32>, target: u32) -> Option<(u32, u32)> {

    let mut hm = HashMap::new();

    // keys are our data values that we care about, value doesn't matter
    // we can look backwards and cease iterating if we find a sum. we
    // know that there is only one combination which sums to the target.
    for (i, &e) in v.iter().enumerate() {
        let complement: u32 = target - e;
        if hm.contains_key(&complement){
            return Some((e, complement));
        }
        hm.insert(e, i);
    }

    None
}


fn three_sum(v: &mut Vec<u32>, target: u32) -> Option<(u32, u32, u32)> {
    // sort and brute?
    v.sort_by(|a,b| a.cmp(b));

    for i in 0..v.len() {
        for j in 0..v.len() {
            if v[i] + v[j] > target {
                break
            }
            for k in 0..v.len() {
                if v[i] + v[j] + v[k] == target {
                    return Some((v[i], v[j], v[k]));
                }
            }
        }
    }

    None 
}

// Unfortunately, the hashmap one pass was not my idea.
// https://leetcode.com/problems/two-sum/solution/
// https://stackoverflow.com/questions/28193463/how-to-simplify-parsing-a-text-file-to-a-vector-of-values