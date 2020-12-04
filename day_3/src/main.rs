use std::fs;

fn main() {
    let s = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file"); 
    
    let vv = produce_graph(s);

    let soln = find_trees(vv);

    println!("Solution: {}", soln);
}


fn produce_graph(s: String) -> Vec<Vec<bool>> {
    // nothing clever, just put all the trees in a vec of vec.
    // expecting this to fluctuate in size for part 2 so vec over array

    vec![vec![true]]
}

fn find_trees(vv: Vec<Vec<bool>>) -> u16 {
    // we never wrap around the edge in this method, so just index magic

    0
}