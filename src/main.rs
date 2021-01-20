// v0.2.0: output A T G C counts

use std::io::{self, BufRead, BufReader};
use std::env;
use std::collections::HashMap;

fn main() {
    let args: Vec<String> = env::args().collect();
    let min_cov: u32 = args[1].parse().expect("Please give a number of minimum coverage"); // minimum coverage at a position
    println!("minimum coverage is {}", min_cov);

    let input = io::stdin();
    let mut reader = BufReader::new(input.lock());
    let mut first_line = String::new();
    let _ = reader.read_line(&mut first_line);
    let ncol = first_line.split("\t").count();
    let nsample = (ncol - 3) / 4;
    println!("First line has {} columns and {} samples.", ncol, nsample);
    let max_missing = 4;
    let min_lib_count = nsample - max_missing;
    let mut good_lines = 0;
    let mut map = HashMap::new(); // count A, T, G, C
    // let min_cov = 2;
    // let min_cov_str = min_cov.to_string();

    for line in reader.lines() {
        let mut nmiss = 0;
        let mut ngood = 0;
        // Method 1: direct loop
        let mut n = 0;
        let mut target = 7;
        let mut nt = String::from("A"); // initial value for column 2
        for word in line.unwrap().split("\t") {
            n += 1;
            if n == 3 {
                nt = word.to_string();
                // println!("word {}", word);
                // println!("word.to_string() is {}", word.to_string());
            }
            if n == target {
                target += 4;
                if word == "." {
                    nmiss += 1;
                    if nmiss > max_missing {break}
                } else {
                    let nn: u32 = word.parse().unwrap();
                    if nn >= min_cov {
                        ngood += 1;
                    } else {
                        nmiss += 1;
                        if nmiss > max_missing {break}
                    }
                }
            } else {
                continue;
            }
        }
        if ngood >= min_lib_count {
            good_lines += 1;
            let count = map.entry(nt).or_insert(0);
            *count += 1;
        }
    }
    println!("The input has {} good lines.", good_lines);
    for (key, value) in &map {
        println!("{}\t{}", &key, value);
    }
}


