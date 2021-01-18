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
    // let ff = first_line.split("\t");
    // let vec = ff.collect::<Vec<&str>>();
    // let ncol = vec.len();
    let ncol = first_line.split("\t").count();
    let nsample = (ncol - 3) / 4;
    println!("First line has {} columns and {} samples.", ncol, nsample);
    let max_missing = 4;
    let mut good_lines = 0;

    for line in reader.lines() {
        let mut nmiss = 0;
        // Method 1: direct loop
        let mut n = 0;
        let mut target = 7;
        for word in line.unwrap().split("\t") {
            // println!("word '{}'", word);
            n += 1;
            if n == target {
                target += 4;
                if word == "." {
                    nmiss += 1;
                    if nmiss > max_missing {break}
                }
            } else {
                continue;
            }
        }
        // Method 2: split to string vector: much much slower
        // let vec = line_to_words(line.unwrap());
        // for x in (6..ncol).step_by(4) {
        //     if vec[x] == "." {
        //         nmiss += 1;
        //         if nmiss > max_missing {break}
        //     }
        // }
        // after checking all the fields
        if nmiss <= max_missing {
            good_lines += 1;
        }
    }
    println!("The input has {} good lines.", good_lines);
}

// fn line_to_words(line: String) -> Vec<String> {
//     line.split("\t").map(str::to_string).collect()
// }

