use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

use std::collections::HashMap;

fn compare_strings(s1 : &str, s2 : &str) -> i32 {
    let mut diffs = 0;
    for (c1, c2) in s1.chars().zip(s2.chars()) {
        if c1 != c2 {
            diffs += 1;
        }
    }
    diffs
}

fn remove_diff_chars(s1 : &str, s2 : &str) -> String {
    let mut s = String::from("");
    for (c1, c2) in s1.chars().zip(s2.chars()) {
        if c1 == c2 {
            s.push(c1);
        }
    }
    s
}

fn main() -> std::io::Result<()> {
    let file = File::open("input")?;
    let mut buf_reader = BufReader::new(file);
    let mut full_file = String::new();
    buf_reader.read_to_string(&mut full_file)?;

    let mut couple_words = 0;
    let mut triple_words = 0;

    for line in full_file.lines(){
        let mut letters = HashMap::new();
        for c in line.chars() {
            let e = letters.entry(c).or_insert(0);
            *e += 1;
        }

        let mut couple_bool = false;
        let mut triple_bool = false;
        for (_, count) in &letters {
            if *count == 2 {
                couple_bool = true;
            } else if *count == 3 {
                triple_bool = true;
            }
        }

        if couple_bool {
            couple_words += 1;
        }
        if triple_bool {
            triple_words +=1;
        }

    }


    println!("checksum : {} * {} = {}", couple_words, triple_words, couple_words * triple_words);

    let mut couple = ("", "");
    let vec = full_file.lines().collect::<Vec<&str>>();
    for i1 in 0..(vec.len()) {
        for i2 in (i1+1)..(vec.len()) {
            if compare_strings(vec[i1], vec[i2]) == 1 {
                couple = (vec[i1], vec[i2]);
            }
        }
    }

    println!("couple : {:?}, word = {}", couple, remove_diff_chars(couple.0, couple.1));

    Ok(())
}
