use std::fs;

fn main() {
    let boarding_pass = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file");

    let boarding_pass: Vec<&str> = boarding_pass.split('\n').collect();

    let mut parsed_boarding_pass: Vec<Vec<char>> = vec![];

    for pass in boarding_pass {
        parsed_boarding_pass.push(pass.chars().collect());
    }

    let mut max = 0;
    for mut pass in parsed_boarding_pass {
        let row = bin_search_row(&mut pass, 0, 127);
        let col = bin_search_col(&mut pass, 0, 7); 
        let seat_id = row * 8 + col;
        if seat_id > max {
            max = seat_id;
        }
    }

    println!("{}", max);
}

fn bin_search_row(boarding_pass: &mut Vec<char>, lower: usize, upper: usize) -> usize {
    let mut mid: usize = (upper - lower) / 2;
    if boarding_pass[0] == 'F' {
        mid = lower + mid;
        boarding_pass.remove(0);
        return bin_search_row(boarding_pass, lower, mid)
    }
    else if boarding_pass[0] == 'B' {
        mid = lower + mid + 1;
        boarding_pass.remove(0);
        return bin_search_row(boarding_pass, mid, upper);
    }

    upper
}

fn bin_search_col(boarding_pass: &mut Vec<char>, lower: usize, upper: usize) -> usize {
    let mut mid: usize = (upper - lower) / 2;
    if boarding_pass.len() > 0 && boarding_pass[0] == 'L' {
        mid = lower + mid;
        boarding_pass.remove(0);
        return bin_search_col(boarding_pass, lower, mid);
    } else if boarding_pass.len() > 0 && boarding_pass[0] == 'R' {
        mid = lower + mid + 1;
        boarding_pass.remove(0);
        return bin_search_col(boarding_pass, mid, upper);
    }

    upper
}