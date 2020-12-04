use std::fs;

fn main() {
    let s = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file"); 
    
    let vv = produce_graph(s, 1, 3);

    let soln = find_trees(vv);

    println!("Solution: {}", soln);
}


fn produce_graph(s: String, down: usize, right: usize) -> Vec<Vec<bool>> {
    // why wouldn't I just mod by the width?! 
    // really should get some sleep.
}

fn find_trees(vv: Vec<Vec<bool>>, right: usize, down: usize) -> u16 {

    0
}