use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

use std::collections::HashSet;
use std::collections::LinkedList;

fn char_matches(c1 : char, c2 : char) -> bool {
    (c1.is_uppercase() && c2.is_lowercase() && c2.to_ascii_uppercase() == c1)
        || (c2.is_uppercase() && c1.is_lowercase() && c1.to_ascii_uppercase() == c2)
}

fn _reduce1(initial_polymer : String) -> usize {
    let mut current_polymer = initial_polymer;
    let mut was_modified = true;
    while was_modified {
        let mut new_polymer = String::new();
        let mut last_char = ' ';
        was_modified = false;
        for c in current_polymer.chars() {
            if last_char == ' ' {
                last_char = c;
            } else {
                if !char_matches(c, last_char)  {
                    new_polymer.push(last_char);
                    last_char = c;
                } else {
                    last_char = ' ';
                    was_modified = true;
                }
            }
        }
        if last_char != ' ' {
            new_polymer.push(last_char);
        }
        current_polymer = new_polymer;
    }
    current_polymer.len()
}

fn reduce2(initial_polymer : String) -> usize {
    let mut chain = LinkedList::new();
    for c in initial_polymer.chars() {
        let opt = match chain.back() {
            | None => None,
            | Some(c_ch) => Some(*c_ch),
        };

        match opt {
            | None => chain.push_back(c),
            | Some(c_ch) => {
                if char_matches(c, c_ch) {
                    let _ = chain.pop_back();
                } else {
                    chain.push_back(c)
                }
            },
        }
    }
    chain.len()
}

fn main() -> std::io::Result<()> {
    let file = File::open("input")?;
    let mut buf_reader = BufReader::new(file);
    let mut initial_polymer = String::new();
    buf_reader.read_to_string(&mut initial_polymer)?;
    initial_polymer = String::from(initial_polymer.trim());

    let mut chars = HashSet::new();
    for c in initial_polymer.chars() {
        chars.insert(c.to_ascii_lowercase());
    }

    let mut best_length = initial_polymer.len();
    let mut best_char = ' ';
    for c in &chars {
        let current_polymer = initial_polymer.replace(|a : char| a.to_ascii_lowercase() == *c,"");
        let length = reduce2(current_polymer);
        if length < best_length {
            best_length = length;
            best_char = *c;
        }
    }

    println!("{} && len: {}", best_char, best_length);

    Ok(())
}
