use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

use std::collections::HashMap;


fn main() -> std::io::Result<()> {
    let file = File::open("input")?;
    let mut buf_reader = BufReader::new(file);
    let mut full_file = String::new();
    buf_reader.read_to_string(&mut full_file)?;

    let mut freq_reached = HashMap::new();
    let mut found_freq = false;
    let mut calibrate_freq = 0;


    let mut freq : i32 = 0;
    let mut first_run = true;

    while !found_freq {
        for l in full_file.lines(){
            freq += l.parse::<i32>().unwrap();
            // println!("{}", freq);

            if !freq_reached.contains_key(&freq) {
                freq_reached.insert(freq, 1);
            } else if !found_freq {
                calibrate_freq = freq;
                *freq_reached.get_mut(&freq).unwrap() += 1;
                found_freq = true;
            }
        }
        if first_run {
            println!("End freq: {}", freq);
            first_run = false;
        }
    }

    println!("Calibrate freq: {}", calibrate_freq);

    Ok(())
}
