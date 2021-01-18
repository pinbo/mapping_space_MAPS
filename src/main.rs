use std::io::{self, BufRead, BufReader};
// use std::env;

fn main() {
    // let args: Vec<String> = env::args().collect();
    // let ncol: u32 = args[1].parse().expect("Please type a number!"); // file number of columns
    // println!("Input has {} columns", ncol);

    let input = io::stdin();
    let mut reader = BufReader::new(input.lock());
    let mut first_line = String::new();
    let _ = reader.read_line(&mut first_line);
    let ncol = first_line.split("\t").count();
    let nsample = (ncol - 3) / 4;
    println!("First line has {} columns and {} samples.", ncol, nsample);
    let max_missing = 4;
    let mut good_lines = 0;
    let min_cov = 2;
    // let min_cov_str = min_cov.to_string();

    for line in reader.lines() {
        let mut nmiss = 0;
        let mut ngood = 0;
        // Method 1: direct loop
        let mut n = 0;
        let mut target = 7;
        for word in line.unwrap().split("\t") {
            n += 1;
            if n == target {
                target += 4;
                if word == "." {
                    nmiss += 1;
                    if nmiss > max_missing {break}
                } else {
                    let nn: usize = word.parse().unwrap();
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
        if ngood >= nsample - min_cov  {
            good_lines += 1;
        }
    }
    println!("The input has {} good lines.", good_lines);
}


