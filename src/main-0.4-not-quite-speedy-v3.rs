// main
// v0.2.0: output A T G C counts
// v0.3.0: add options
// v.0.4.0: use read_line to see whether speeding up: NOT much
// v.0.4.1: put line processing to a function
// v.0.4.2: read gz file directly
// to make dynamic smaller app: cargo rustc --release -- -C prefer-dynamic
use std::io::{self, BufReader, BufRead};
// use std::env;
use std::collections::HashMap;
use clap::{arg, App};
use flate2::read::GzDecoder;
use std::fs::File;

fn main() {
    // parse options
    let matches = App::new("Calc_mapping_space")
        .version("0.4.2")
        .author("Junli Zhang <zhjl86@gmail.com>")
        .about("Calculate mapping space and %GC from MAPS output parsed_Kronos_mpileup.txt.gz")
        .arg(arg!(-i --input [FILE]              "input file name (default from 'stdin')"))
        .arg(arg!(-m  --max_missing [NUMBER] "maxium number of missing libs"))
        .arg(arg!(-c --min_cov [NUMBER]              "minimum coverage at positions"))
        .get_matches();
    // get the values
    let min_cov: usize = matches.value_of("min_cov").unwrap_or("1").parse().expect("Please give a number of minimum coverage"); // minimum coverage at a position
    println!("minimum coverage is {}", min_cov);
    let max_missing: usize = matches.value_of("max_missing").unwrap_or("4").parse().expect("Please give a number of maximum missing libs");
    println!("Maximum missing libs allowed is {}", max_missing);
    let in_file = matches.value_of("input").unwrap_or("stdin");
    println!("Input file is {}", in_file);

    // let args: Vec<String> = env::args().collect();
    // let min_cov: u32 = args[1].parse().expect("Please give a number of minimum coverage"); // minimum coverage at a position
    // println!("minimum coverage is {}", min_cov);
    // let reader: Box<dyn BufRead> = match input {
    //     None => Box::new(BufReader::new(io::stdin())),
    //     Some(filename) => Box::new(BufReader::new(fs::File::open(filename).unwrap()))
    // };

    let mut reader: Box<dyn BufRead> = if in_file == "stdin" {
        Box::new(BufReader::new(io::stdin()))
    } else if in_file.ends_with(".gz") {
        let f = File::open(in_file).unwrap();
        Box::new(BufReader::new(GzDecoder::new(f)))
    } else {
        let f = File::open(in_file).unwrap();
        Box::new(BufReader::new(f))
    };

    // let mut reader = BufReader::new(input);
    let mut first_line = String::new();
    let _ = reader.read_line(&mut first_line);
    let ncol = first_line.split("\t").count();
    let nsample = (ncol - 3) / 4;
    println!("First line has {} columns and {} samples.", ncol, nsample);
    // let max_missing = 4;
    let min_lib_count = nsample - max_missing;
    let mut good_lines = 0;
    let mut map = HashMap::new(); // count A, T, G, C
    // code is from here: https://dev.to/dandyvica/different-ways-of-reading-files-in-rust-2n30
    let mut line = String::new();
    loop {
        match reader.read_line(&mut line) {
            Ok(bytes_read) => {
                // EOF: save last file address to restart from this address for next run
                if bytes_read == 0 {
                    break;
                }
                // func(&line);
                let (ngood, nt) = process_line(line.trim_end(), max_missing, min_cov);
                if ngood >= min_lib_count {
                    good_lines += 1;
                    let count = map.entry(nt).or_insert(0);
                    *count += 1;
                }
                // do not accumulate data
                line.clear();
            }
            Err(err) => {
                panic!("Error - {}", err );
            }
        };
    } // end of loop

    println!("The input has {} good lines.", good_lines);
    let mut total = 0;
    let mut gc_count = 0;
    let nts = "ATGCatgc";
    let gc = "GCgc";
    for (key, value) in &map {
        println!("{}\t{}", &key, value);
        if nts.contains(key) {total += value;}
        if gc.contains(key) {gc_count += value;}
    }
    println!("GC% is {:.2}%", (gc_count as f64 / total as f64) * 100.0);
}

fn process_line(line: &str, max_missing: usize, min_cov: usize) -> (usize, String) {
    let mut nmiss = 0;
    let mut ngood = 0;
    let mut n = 0;
    let mut target = 7;
    let mut nt = String::from("A"); // initial value for column 2
    for word in line.split("\t") {
        n += 1;
        if n == 3 {
            nt = word.to_string();
        }
        if n == target {
            target += 4;
            if word == "." {
                nmiss += 1;
                if nmiss > max_missing {break;}
            } else {
                let nn: usize = word.parse().unwrap();
                if nn >= min_cov {
                    ngood += 1;
                } else {
                    nmiss += 1;
                    if nmiss > max_missing {break;}
                }
            }
        } else {
            continue;
        }
    }
    return (ngood, nt);
}
