use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

use std::collections::HashSet;

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
        let mut was_modified = true;
        let mut current_polymer = initial_polymer.replace(|a : char| a.to_ascii_lowercase() == *c,"");
        while was_modified {
            let mut new_polymer = String::new();
            let mut last_char = ' ';
            was_modified = false;
            for c in current_polymer.chars() {
                if last_char == ' ' {
                    last_char = c;
                } else {
                    if ! ((c.is_uppercase() && last_char.is_lowercase() && last_char.to_ascii_uppercase() == c)
                        || (last_char.is_uppercase() && c.is_lowercase() && c.to_ascii_uppercase() == last_char)) {
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
        if current_polymer.len() < best_length {
            best_length = current_polymer.len();
            best_char = *c;
        }
    }

    println!("{} && len: {}", best_char, best_length);

    Ok(())
}
