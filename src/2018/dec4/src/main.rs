use std::fmt;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::cmp::Ordering;

use std::collections::HashMap;

trait Parsable {
    fn parse(s : &str) -> Self;
}

#[derive(Ord, PartialOrd, PartialEq, Eq)]
struct Date {
    year : u32,
    month: u32,
    day: u32,
    hour: u32,
    minute: u32,
}

impl Date{
    fn new() -> Date {
        Date {
            year: 0,
            month: 0,
            day: 0,
            hour: 0,
            minute: 0,
        }
    }
    fn period_since_local(&self, date : &Date) -> u32 {
        let modulo = 60*24;
        (self.minute - date.minute + 60 * (self.hour - date.hour) % modulo + modulo) % modulo
    }
}

impl fmt::Display for Date {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}-{}-{} {}:{}]", self.year, self.month, self.day, self.hour, self.minute)
    }
}

impl Parsable for Date {
    fn parse(s: &str) -> Date {
        let splitted : Vec<&str> = s.trim()
                        .split(|c| c == ' ' || c == '[' || c == ']')
                        .filter(|s| !s.is_empty())
                        .collect();
        let date : Vec <u32> = splitted[0].split('-')
            .map(|s| s.parse::<u32>().unwrap_or_else(|_| 666)).collect();
        let hour : Vec <u32> = splitted[1].split(':')
            .map(|s| s.parse::<u32>().unwrap_or_else(|_| 666)).collect();
        Date {
            year: date[0],
            month: date[1],
            day: date[2],
            hour: hour[0],
            minute: hour[1],
        }
    }
}

#[derive(Eq, PartialEq, Ord, PartialOrd)]
struct Guard(u32);

impl fmt::Display for Guard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Guard #{}", self.0)
    }
}

impl Parsable for Guard {
    fn parse(s: &str) -> Guard {
        Guard(s.trim()
               .split('#').collect::<Vec<&str>>()[1]
               .split(' ').collect::<Vec<&str>>()[0]
               .parse::<u32>().unwrap_or_else(|_| 666))
    }
}

#[derive(Eq, PartialEq, Ord, PartialOrd)]
enum Event {
    ShiftStart(Guard),
    FallsAsleep,
    WakesUp,
}

impl Parsable for Event {
    fn parse(s: &str) -> Event {
        let s = s.trim();
        if s == "wakes up" {
            Event::WakesUp
        } else if s == "falls asleep" {
            Event::FallsAsleep
        } else {
            Event::ShiftStart(Guard::parse(s))
        }
    }
}

impl fmt::Display for Event {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            | Event::ShiftStart(guard) =>
                write!(f, "{} begins shift", guard),
            | Event::FallsAsleep =>
                write!(f, "falls asleep"),
            | Event::WakesUp =>
                write!(f, "wakes up"),
        }
    }
}

#[derive(Eq)]
struct Entry {
    date: Date,
    event: Event,
}

impl Parsable for Entry {
    fn parse(s: &str) -> Entry {
        let splitted = s.trim().split(']').collect::<Vec<&str>>();
        Entry {
            date: Date::parse(splitted[0]),
            event: Event::parse(splitted[1]),
        }
    }
}

impl fmt::Display for Entry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.date, self.event)
    }
}

impl Ord for Entry {
    fn cmp(&self, other: &Entry) -> Ordering {
        self.date.cmp(&other.date)
    }
}

impl PartialOrd for Entry {
    fn partial_cmp(&self, other: &Entry) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl PartialEq for Entry {
    fn eq(&self, other: &Entry) -> bool {
        self.date == other.date
    }
}

fn sleep_times(entries : &Vec<Entry>) -> u32 {
    let mut sleep_times = HashMap::new();
    let mut current_shift = 0;
    let mut last_sleep_start = &Date::new();
    for e in entries {
        match &e.event {
            | Event::ShiftStart(guard) => current_shift = guard.0,
            | Event::FallsAsleep => last_sleep_start = &e.date,
            | Event::WakesUp => {
                let mut sleep_time = sleep_times.entry(current_shift).or_insert(0);
                *sleep_time += e.date.period_since_local(&last_sleep_start);
                },
        }
    }

    let mut max_sleep = 0;
    let mut worst_guard = 0;
    for (guard, time) in sleep_times.iter() {
        if *time > max_sleep {
            worst_guard = *guard;
            max_sleep = *time;
        }
    }
    worst_guard
}

fn best_minute(entries : &Vec<Entry>, worst_guard: u32) -> (u32, u32) {
    let mut asleep_on = HashMap::new();
    let mut our_guard = false;
    let mut last_sleep_start = 0u32;
    for e in entries {
        match &e.event {
            | Event::ShiftStart(guard) => {
                if guard.0 == worst_guard { our_guard = true } else {our_guard = false}
                },
            | Event::FallsAsleep => {
                if our_guard {
                    last_sleep_start = e.date.minute
                }
                },
            | Event:: WakesUp => {
                if our_guard {
                    for m in last_sleep_start..(e.date.minute) {
                        let nb = asleep_on.entry(m).or_insert(0);
                        *nb += 1;
                    }
                }
            },
        }
    }

    let (&min, &nb) = asleep_on.iter().max_by_key(|(_, nb)| *nb).unwrap_or((&0, &0));
    (min, nb)
}

fn best_minute_2(entries : &Vec<Entry>) -> (u32, u32) {
    let mut asleep_on = HashMap::new();
    let mut current_shift = 0;
    let mut last_sleep_start = 0u32;
    for e in entries {
        match &e.event {
            | Event::ShiftStart(guard) => current_shift = guard.0,
            | Event::FallsAsleep => {
                    last_sleep_start = e.date.minute
                },
            | Event:: WakesUp => {
                    for m in last_sleep_start..(e.date.minute) {
                        let nb = asleep_on.entry((current_shift, m)).or_insert(0);
                        *nb += 1;
                    }
            },
        }
    }

    let (&(guard, min), &_) = asleep_on.iter().max_by_key(|(_, nb)| *nb).unwrap_or((&(0, 0), &0));
    (guard, min)
}

fn main() -> std::io::Result<()> {
    let file = File::open("input")?;
    let mut buf_reader = BufReader::new(file);
    let mut full_file = String::new();
    buf_reader.read_to_string(&mut full_file)?;

    let mut entries = Vec::new();
    for l in full_file.lines() {
        entries.push(Entry::parse(l));
    }

    entries.sort();

    let target_guard = sleep_times(&entries);

    let (m, _) = best_minute(&entries, target_guard);
    println!("1: Guard {}, {}. Sol = {}", target_guard, m, target_guard * m);

    let (g, m) = best_minute_2(&entries);
    println!("2: Guard {}, {}. Sol = {}", g, m, g * m);
    Ok(())
}
