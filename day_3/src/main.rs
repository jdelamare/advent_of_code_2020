use std::fs;

fn main() {
    let s = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file"); 
    
    let width = s.lines().next().unwrap().len();
    let vv = produce_graph(s);
    let height = vv.len();
    let mut soln: Vec<u64> = vec![];

    soln.push(find_trees(&vv, 1, 3, width, height));
    soln.push(find_trees(&vv, 1, 1, width, height));
    soln.push(find_trees(&vv, 1, 5, width, height));
    soln.push(find_trees(&vv, 1, 7, width, height));
    soln.push(find_trees(&vv, 2, 1, width, height));

    println!("Solution: {:?}", soln.iter().product::<u64>());
}


fn produce_graph(s: String) -> Vec<Vec<bool>> {

    let mut vvb: Vec<Vec<bool>> = vec![];

    for l in s.lines() {
        let mut vb = vec![];
        let cs: Vec<char> = l.chars().collect();
        cs.iter().for_each(|c| if *c == '.' { vb.push(false); } else { vb.push(true); } );
        vvb.push(vb);
    }

    vvb
}

fn find_trees(vv: &Vec<Vec<bool>>, down: usize, right: usize, width: usize, height: usize) -> u64 {
    let mut trees = 0;
    let mut x = 0;
    let mut y = 0;

    loop {
        if y + down > height {
            break;
        }

        if vv[y][x] {
            // hit a tree
            trees += 1; 
        } 

        y = y + down; // got the right answer, not sure if we handled trees on the line that goes over
        x = (x + right) % width;
    }

    trees 
}