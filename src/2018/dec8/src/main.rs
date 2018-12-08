use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

struct Node {
    childs: Vec<Node>,
    metadata: Vec<u32>,
}

fn pop_nb(entries: &Vec<u32>, i: &mut usize) -> u32 {
    let v = entries[*i];
    *i += 1;
    v
}

fn parse_node (entries: &Vec<u32>, i: &mut usize) -> Node {
    let nb_childs = pop_nb(entries, i);
    let nb_metadata = pop_nb(entries, i);
    let mut childs = Vec::new();
    for _ in 0..nb_childs {
        childs.push(parse_node(entries, i));
    }
    let mut metadata = Vec::new();
    for _ in 0..nb_metadata {
        metadata.push(pop_nb(entries, i));
    }
    Node {childs, metadata}
}

fn parse_tree(entries: &Vec<u32>) -> Node {
    let mut i = 0;
    parse_node(entries, &mut i)
}

fn sum_metadata(t: &Node) -> u32 {
    let mut n = 0;
    for &met in &t.metadata {
        n += met;
    }
    for child in &t.childs {
        n += sum_metadata(child);
    }
    n
}

fn value_node(t: &Node) -> u32 {
    let mut n = 0;
    if t.childs.len() == 0 {
        for &met in &t.metadata {
            n += met
        }
    } else {
        for &met in &t.metadata {
            let child_index = (met - 1) as usize;
            if child_index < t.childs.len() {
                let child = &t.childs[child_index];
                n += value_node(child);
            }
        }
    }
    n
}

fn main() -> std::io::Result<()> {
    let file = File::open("input")?;
    let mut buf_reader = BufReader::new(file);
    let mut full_file = String::new();
    buf_reader.read_to_string(&mut full_file)?;
    let entries: Vec<u32>  = full_file.trim().split(' ').map(|s| s.parse::<u32>().unwrap()).collect();
    let t = parse_tree(&entries);

    let sum = sum_metadata(&t);
    println!("sum = {}", sum);

    let value = value_node(&t);
    println!("value = {}", value);

    Ok(())
}
