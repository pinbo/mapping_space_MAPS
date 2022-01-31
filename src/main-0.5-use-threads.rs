// main
// v0.2.0: output A T G C counts
// v0.3.0: add options
// v0.4.0: use read_line to see whether speeding up: NOT much
// v0.4.1: put line processing to a function
// v0.5.0: try threads: slower than not-using, possibly because my line-processing is too fast
// to make dynamic smaller app: cargo rustc --release -- -C prefer-dynamic
use std::io::{self, BufRead, BufReader};
// use std::env;
use std::collections::HashMap;
use clap::{arg, App};
use threadpool::ThreadPool;
use std::sync::mpsc::channel;

fn main() {
    // parse options
    let matches = App::new("Calc_mapping_space")
        .version("0.5.0")
        .author("Junli Zhang <zhjl86@gmail.com>")
        .about("Calculate mapping space and %GC from MAPS output parsed_Kronos_mpileup.txt.gz")
        .arg(arg!(-m  --max_missing [NUMBER] "maxium number of mising libs"))
        .arg(arg!(-c --min_cov [NUMBER]              "minimum coverage at positions"))
        .arg(arg!(-t --nthread [NUMBER]              "number of threads"))
        .get_matches();
    // get the values
    let min_cov: usize = matches.value_of("min_cov").unwrap_or("1").parse().expect("Please give a number of minimum coverage"); // minimum coverage at a position
    println!("minimum coverage is {}", min_cov);
    let max_missing: usize = matches.value_of("max_missing").unwrap_or("4").parse().expect("Please give a number of maximum missing libs");
    println!("Maximum missing libs allowed is {}", max_missing);
    let nthread: usize = matches.value_of("nthread").unwrap_or("2").parse().expect("Please give the number of theads to use");
    println!("Number of threads to use: {}", nthread);

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
    let pool = ThreadPool::new(nthread); // multi-threading
    let (tx, rx) = channel();

    for line in reader.lines() {
        let tx = tx.clone();
        pool.execute(move || {
            let digest = process_line(&line.unwrap(), max_missing, min_cov);
            tx.send(digest).expect("Could not send data!");
        });
    }
    drop(tx);
    for t in rx.iter() {
        let (ngood, nt) = t;
        // println!("{:?} {:?}", sha, path);
        if ngood >= min_lib_count {
            good_lines += 1;
            let count = map.entry(nt).or_insert(0);
            *count += 1;
        }
    }

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
    let mut nt = String::from("Q"); // initial value for column 2
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
