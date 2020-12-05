use std::fs;
use regex::Regex;

fn main() {
    let s = fs::read_to_string("input.txt")
    .expect("Something went wrong reading the file");

    let v: Vec<&str> = s.split("\n\n").collect();

    let mut soln_1 = 0;
    let mut soln_2 = 0;

    for passport in &v {
        if passport.contains("byr") &&
           passport.contains("iyr") &&
           passport.contains("eyr") &&
           passport.contains("hgt") &&
           passport.contains("hcl") &&
           passport.contains("ecl") &&
           passport.contains("pid") {
        
            soln_1 += 1;
        }
    }
    println!("Part 1 soln: {:?}", soln_1);

    for passport in v {
        let mut valid = true;
        let passport = passport.replace('\n', " ");
        let passport: Vec<&str> = passport.split_ascii_whitespace().collect();
        for field in passport {
            let kv: Vec<&str> = field.split(':').collect();
            let k = kv[0];
            let v = kv[1];
            match k {
                "byr" => {
                    let t = v.parse::<u16>(); // may need to catch exceptions here
                    match t {
                        Ok(x) => if x < 1920 || x > 2020 {
                            valid = false;
                        },
                        Err(_) => valid = false
                    };
                },
                "iyr" => {
                    let t = v.parse::<u16>(); 
                    match t {
                        Ok(x) => if x < 2010 || x > 2020 {
                            valid = false;
                        },
                        Err(_) => valid = false
                    };
                },
                "eyr" => {
                    let t = v.parse::<u16>(); 
                    match t {
                        Ok(x) => if x < 2020 || x > 2030 {
                            valid = false;
                        },
                        Err(_) => valid = false
                    };
                },
                "hgt" => {
                    if v.contains("in") {
                        let t = v.replace("in", "").parse::<u8>();
                        match t {
                            Ok(x) => if x < 59 || x > 76 {
                                valid = false;
                            }
                            Err(_) => valid = false
                        };
                    } else if v.contains("cm") {
                        let t = v.replace("cm", "").parse::<u8>();
                        match t {
                            Ok(x) => if x < 150 || x > 193 {
                                valid = false;
                            }
                            Err(_) => valid = false
                        };
                    } else {
                        valid = false;
                    }
                },
                "hcl" => {
                    match v.get(..1) {
                        Some(c) => {
                            if c != "#" {
                                valid = false;
                            } else {
                                let cs: Vec<char> = v.replace("#", "").chars().collect();
                                let re = Regex::new("[0-9a-f]").unwrap();
                                cs.iter().for_each(|c| if !re.is_match(&(c.to_string())) { valid = false});
                            }
                        },
                        _ => valid = false
                    }
                },
                "ecl" => {
                    match v {
                        "amb" => (),
                        "blu" => (),
                        "brn" => (),
                        "gry" => (),
                        "grn" => (),
                        "hzl" => (),
                        "oth" => (),
                        _ => valid = false
                    };
                },
                "pid" => {
                    if v.len() != 9 {
                        valid = false;
                    } else {
                        let n = v.parse::<u64>();
                        match n {
                            Ok(n) => (),
                            Err(_) => valid = false
                        }
                    }
                },
                "cid" => (),
                _ => valid = false
            }
            if !valid {
                break;
            }
        } 
        if valid {
            soln_2 += 1;
        }
    }

    println!("Part 2 soln: {}", soln_2);
}