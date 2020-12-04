use std::fs;

fn main() {
    let s = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file"); 
    
    let vv = produce_graph(s, 1, 3);

    let soln = find_trees(vv);

    println!("Solution: {}", soln);
}


fn produce_graph(s: String, down: usize, right: usize) -> Vec<Vec<bool>> {
    // nothing clever, just put all the trees in a vec of vec.
    // expecting this to fluctuate in size for part 2 so vec over array
    let mut vvc: Vec<Vec<char>> = vec![vec![]];
    let mut vvb: Vec<Vec<bool>> = vec![vec![]];

    // not space efficient, makes a square
    let region_width = s.lines().nth(0).unwrap().chars().count();
    let region_height = s.lines().count();
    println!("rh{:?}", region_height);
    println!("rw{:?}", region_width);

    // if the amount we go down is less than the width than the region is already big enough
    // else the worst case we go right on every single row, make vec that big
    let total_width = match down * region_height < region_width {
        true => 1,
        false => (right as f32 * region_height as f32 / region_width as f32).ceil() as usize
    };
    println!("w: {}", total_width);
    s.lines().for_each(|l| vvc.push(l.chars().collect()));

    for vc in vvc {
        let mut vb = vec![];
        vc.iter().for_each(|c| if *c == '.' { vb.push(false); } else { vb.push(true); } );
        for _ in 0..total_width {
            vvb.push(vb.as_slice().repeat(total_width));
        }
    }

    vvb
}

fn find_trees(vv: Vec<Vec<bool>>, right: usize, down: usize) -> u16 {
    // we never wrap around the edge in this method, so just index magic
    let mut trees = 0;
    let mut down_posn = 0;
    let mut right_posn = 0;
    for v in vv {
        
    }
    0
}