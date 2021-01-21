// v0.2.0: output A T G C counts
// v0.3.0: add options
// to make dynamic smaller app: cargo rustc --release -- -C prefer-dynamic
use std::io::{self, BufRead, BufReader};
// use std::env;
use std::collections::HashMap;
use clap::App;

fn main() {
    // parse options
    let matches = App::new("Calc_mapping_space")
        .version("0.3.0")
        .author("Junli Zhang <zhjl86@gmail.com>")
        .about("Calculate mapping space and %GC from MAPS output parsed_Kronos_mpileup.txt.gz")
        .arg("-m, --max_missing=[NUMBER] 'maxium number of mising libs'")
        .arg("-c, --min_cov=[NUMBER]              'minimum coverage at positions'")
        .get_matches();
    // get the values
    // Gets a value for config if supplied by user, or defaults to "default.conf"
    // let config = matches.value_of("config").unwrap_or("default.conf");
    let min_cov: u32 = matches.value_of("min_cov").unwrap_or("1").parse().expect("Please give a number of minimum coverage"); // minimum coverage at a position
    println!("minimum coverage is {}", min_cov);
    let max_missing: usize = matches.value_of("max_missing").unwrap_or("4").parse().expect("Please give a number of maximum missing libs");
    println!("Maximum missing libs allowed is {}", max_missing);

    // let args: Vec<String> = env::args().collect();
    // let min_cov: u32 = args[1].parse().expect("Please give a number of minimum coverage"); // minimum coverage at a position
    // println!("minimum coverage is {}", min_cov);

    let input = io::stdin();
    let mut reader = BufReader::new(input.lock());
    let mut first_line = String::new();
    let _ = reader.read_line(&mut first_line);
    let ncol = first_line.split("\t").count();
    let nsample = (ncol - 3) / 4;
    println!("First line has {} columns and {} samples.", ncol, nsample);
    // let max_missing = 4;
    let min_lib_count = nsample - max_missing;
    let mut good_lines = 0;
    let mut map = HashMap::new(); // count A, T, G, C

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
    let mut total = 0;
    let mut gc_count = 0;
    // let nts = ["A".to_string(), "T".to_string(), "G".to_string(), "C".to_string(), "a".to_string(), "t".to_string(), "g".to_string(), "c".to_string()];
    // let gc = ["G".to_string(), "C".to_string(), "g".to_string(), "c".to_string()];
    let nts = "ATGCatgc";
    let gc = "GCgc";
    for (key, value) in &map {
        println!("{}\t{}", &key, value);
        if nts.contains(key) {total += value;}
        if gc.contains(key) {gc_count += value;}
    }
    println!("GC% is {:.2}%", (gc_count as f64 / total as f64) * 100.0);
}


