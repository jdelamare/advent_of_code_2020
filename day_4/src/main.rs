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
        let mut seen: (bool, bool, bool, bool, bool, bool, bool) = (false, false, false, false, false, false, false);
        for field in passport {
            let kv: Vec<&str> = field.split(':').collect();
            let k = kv[0];
            let v = kv[1];
            match k {
                "byr" => {
                    let t = v.parse::<u16>(); // may need to catch exceptions here
                    match t {
                        Ok(x) => if x < 1920 || x > 2002 {
                            valid = false;
                        },
                        Err(_) => valid = false
                    };
                    seen.0 = true;
                },
                "iyr" => {
                    let t = v.parse::<u16>(); 
                    match t {
                        Ok(x) => if x < 2010 || x > 2020 {
                            valid = false;
                        },
                        Err(_) => valid = false
                    };
                    seen.1 = true;
                },
                "eyr" => {
                    let t = v.parse::<u16>(); 
                    match t {
                        Ok(x) => if x < 2020 || x > 2030 {
                            valid = false;
                        },
                        Err(_) => valid = false
                    };
                    seen.2 = true;
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
                    seen.3 = true;
                },
                "hcl" => {
                    match v.get(..1) {
                        Some(c) => {
                            if c != "#" {
                                valid = false;
                            } else {
                                let cs: Vec<char> = v.replace("#", "").chars().collect();
                                if cs.len() != 6 { 
                                    valid = false; 
                                }
                                else {
                                    let re = Regex::new("[0-9a-f]").unwrap();
                                    cs.iter().for_each(|c| if !re.is_match(&(c.to_string())) { valid = false});
                                }
                            }
                        },
                        _ => valid = false
                    }
                    seen.4 = true;
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
                    seen.5 = true;
                },
                "pid" => {
                    if v.len() != 9 {
                        valid = false;
                    } else {
                        let n = v.parse::<u64>();
                        match n {
                            Ok(_) => (),
                            Err(_) => valid = false
                        }
                    }
                    seen.6 = true;
                },
                "cid" => (),
                _ => valid = false
            }
            if !valid {
                break;
            }
        } 
        if valid && seen == (true, true, true, true, true, true, true) {
            soln_2 += 1;
        }
    }

    println!("Part 2 soln: {}", soln_2);
}