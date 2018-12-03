use std::fmt;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;


// use std::collections::HashMap;

struct Entry {
    id: u32,
    x: u32,
    y: u32,
    w: u32,
    h: u32,
}

impl fmt::Display for Entry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "#{} @ {},{}: {}x{}", self.id, self.x, self.y, self.w, self.h)
    }
}

fn entry_of_line (l : &str) -> Entry {
    let parsed_ints : Vec<u32> = l
        .trim_start_matches('#')
        .split(|c| c == ' ' || c == ',' || c == 'x' || c == ':')
        .filter(|s| *s != "@" && !s.is_empty())
        .map(|s| s.parse::<u32>().unwrap_or_else(|_| {println!("what the '{}'", s); 7337}))
        .collect();
    Entry {
        id: parsed_ints[0],
        x: parsed_ints[1],
        y: parsed_ints[2],
        w: parsed_ints[3],
        h: parsed_ints[4],
    }
}


fn main() -> std::io::Result<()> {
    let file = File::open("input")?;
    let mut buf_reader = BufReader::new(file);
    let mut full_file = String::new();
    buf_reader.read_to_string(&mut full_file)?;

    const SIZE_FABRIC: usize = 1000;
    const SIZE_VEC: usize = SIZE_FABRIC * SIZE_FABRIC;
    let mut fabric : [u32; SIZE_VEC] = [0; SIZE_VEC];

    let mut entries = Vec::new();
    for l in full_file.lines() {
        entries.push(entry_of_line(l));
    }

    for entry in &entries {
        for i in entry.x..(entry.x+entry.w) {
            for j in entry.y..(entry.y+entry.h) {
                let x = i as usize;
                let y = j as usize;
                fabric[x + y * SIZE_FABRIC] += 1
            }
        }
    }


    let mut count = 0;

    for i in 0..SIZE_VEC {
        if fabric[i] >= 2 {
            count += 1;
        }
    }

    println!("{}", count);

    for entry in &entries {
        let mut not_overlapping = true;
        for i in entry.x..(entry.x+entry.w) {
            for j in entry.y..(entry.y+entry.h) {
                let x = i as usize;
                let y = j as usize;
                if fabric[x + y * SIZE_FABRIC] >= 2 {
                    not_overlapping = false
                }
            }
        }
        if not_overlapping {
            println!("no overlap: {}", entry)
        }
    }


    Ok(())
}
