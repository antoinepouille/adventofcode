use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

use std::cmp::min;

use std::str::FromStr;

#[derive(Debug)]
struct Entry {
    step: char,
    before: char,
}

impl FromStr for Entry {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let splitted : Vec<&str> = s.trim()
            .trim_start_matches("Step ")
            .trim_end_matches(" can begin.")
            .split(" ").collect();
        let step = splitted[0].chars().next().unwrap();
        let before = splitted[splitted.len() - 1].chars().next().unwrap();
        Ok(Entry {step, before})
    }
}

fn insert_in_vec(vec: &mut Vec<char>, value: char) {
    vec.push(value);
    vec.sort();
}

fn q1(entries: &Vec<Entry>) {
    let index_of_letter = |c: char| (c as usize) - ('A' as usize);
    let letter_of_index = |i: usize| (i + ('A' as usize)) as u8 as char;
    let mut dependancies_nb = [0; 26];
    let mut links = Vec::<Vec<char>>::new();
    for _ in 0..26 {
        links.push(Vec::new());
    }

    for entry in entries {
        dependancies_nb[index_of_letter(entry.before)] += 1;
        insert_in_vec(&mut links[index_of_letter(entry.step)], entry.before);
    }

    let mut ready = Vec::<char>::new();

    for (i, n) in dependancies_nb.iter().enumerate() {
        if *n == 0 {
            insert_in_vec(&mut ready, letter_of_index(i));
        }
    }


    let mut order = String::new();
    while !ready.is_empty() {
        let task = ready.remove(0);
        assert!(dependancies_nb[index_of_letter(task)] == 0);
        order.push(task);
        for &child in &links[index_of_letter(task)] {
            // println!("child of {} is {}", task, child);
            dependancies_nb[index_of_letter(child)] -= 1;
            if dependancies_nb[index_of_letter(child)] == 0 {
                insert_in_vec(&mut ready, child);
                println!("{:?}", ready);
            }
        }
    }

    println!("order = {}", order);
}

fn q2(entries: &Vec<Entry>) {
    let index_of_letter = |c: char| (c as usize) - ('A' as usize);
    let letter_of_index = |i: usize| (i + ('A' as usize)) as u8 as char;
    let mut dependancies_nb = [0; 26];
    let mut links = Vec::<Vec<char>>::new();
    for _ in 0..26 {
        links.push(Vec::new());
    }

    for entry in entries {
        dependancies_nb[index_of_letter(entry.before)] += 1;
        insert_in_vec(&mut links[index_of_letter(entry.step)], entry.before);
    }

    let mut ready = Vec::<char>::new();

    for (i, n) in dependancies_nb.iter().enumerate() {
        if *n == 0 {
            insert_in_vec(&mut ready, letter_of_index(i));
        }
    }

    let length_work = |c : char| 60 + (c as u32) - ('A' as u32) + 1;

    let worker_nb = 5;
    let mut current_doing : Vec<Option<char>> = vec![None; worker_nb as usize];
    let mut end_time : Vec<u32> = vec![0; worker_nb as usize];

    let mut time = 0;

    let mut working = true;
    while !ready.is_empty() || working {
        // wait for someone to be ready for a new task
        let mut workers_ready = Vec::new();
        let &min_time = (&end_time).iter().min().unwrap();
        for (i, t) in (&end_time).iter().enumerate() {
            if *t == min_time {
                workers_ready.push(i);
            }
        }
        time = min_time;
        for &i in &workers_ready {
            // End task
            match current_doing[i] {
                | Some(task) => {
                    for &child in &links[index_of_letter(task)] {
                        dependancies_nb[index_of_letter(child)] -= 1;
                        if dependancies_nb[index_of_letter(child)] == 0 {
                            insert_in_vec(&mut ready, child);
                            println!("{:?}", ready);
                        }
                    }
                },
                | None => ()
            }
        }

        for &i in &workers_ready {
            // get new task, and give it
            if !ready.is_empty() {
                let task = ready.remove(0);
                assert!(dependancies_nb[index_of_letter(task)] == 0);
                current_doing[i] = Some(task);
                end_time[i] = time + length_work(task);
            } else {
                current_doing[i] = None
            }
        }

        // fix waiting time for non-working people
        let mut next_time = 999999999u32;
        working = false;
        for (i, opt) in current_doing.iter().enumerate() {
            match opt {
                | Some(_)=> {
                    next_time = min(next_time, end_time[i]);
                    working = true;
                },
                | None => (),
            }
        }
        for (i, opt) in current_doing.iter().enumerate() {
            match opt {
                | Some(_)=> (),
                | None => {
                    end_time[i] = next_time;
                },
            }
        }
    }

    println!("Time = {}", time);
}


fn main() -> std::io::Result<()> {
    let file = File::open("input")?;
    let mut buf_reader = BufReader::new(file);
    let mut full_file = String::new();
    buf_reader.read_to_string(&mut full_file)?;

    let mut entries = Vec::new();
    for l in full_file.lines() {
        entries.push(Entry::from_str(l).unwrap());
    }

    q1(&entries);
    q2(&entries);

    Ok(())
}
