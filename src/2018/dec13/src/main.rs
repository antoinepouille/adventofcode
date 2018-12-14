use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

use std::cmp;
use std::fmt;
use std::cmp::Ordering;
use std::process;

#[derive(Clone)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

fn apply_turn(dir: &Direction, turn: &Direction) -> Direction {
    match turn {
    | Direction::Up => {
        match dir {
            | Direction::Up => Direction::Up,
            | Direction::Right => Direction::Right,
            | Direction::Left => Direction::Left,
            | Direction::Down => Direction::Down,
        }
    },
    | Direction::Right => {
        match dir {
            | Direction::Up => Direction::Right,
            | Direction::Right => Direction::Down,
            | Direction::Left => Direction::Up,
            | Direction::Down => Direction::Left,
        }
    },
    | Direction::Left => {
        match dir {
            | Direction::Up => Direction::Left,
            | Direction::Right => Direction::Up,
            | Direction::Left => Direction::Down,
            | Direction::Down => Direction::Right,
        }
    },
    | Direction::Down => panic!(),
    }
}

#[derive(Clone)]
enum TrackSegment {
    Empty,
    Vertical,
    Horizontal,
    Crossing,
    CurveUpRight, // defining the two types of slashes here: '/'
    CurveUpLeft, // '\'
}

struct Cart {
    x: usize,
    y: usize,
    facing: Direction,
    choice_turn: Direction,
    destroyed: bool,
}

impl Cart {
    fn advance(self: &mut Self) {
        match self.facing {
            | Direction::Up    => self.y -=1,
            | Direction::Right => self.x += 1,
            | Direction::Down  => self.y += 1,
            | Direction::Left  => self.x -= 1,
        }
    }

    fn turn(self: &mut Self) {
        self.facing = apply_turn(&self.facing, &self.choice_turn);
        self.choice_turn = match self.choice_turn {
            | Direction::Up => Direction::Right,
            | Direction::Right => Direction::Left,
            | Direction::Left => Direction::Up,
            | Direction::Down => panic!(),
        };
    }
}

struct Track {
    segments: Vec<TrackSegment>,
    segments_x: usize,
    segments_y: usize,
    carts: Vec<Cart>,
    step: u32,
}

impl fmt::Display for Track {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = String::new();
        'y: for y in 0..self.segments_y {
            'x: for x in 0..self.segments_x {
                for i in 0..self.carts.len() {
                    if x == self.carts[i].x && y == self.carts[i].y {
                        s.push(match self.carts[i].facing {
                            | Direction::Up => '^',
                            | Direction::Right => '>',
                            | Direction::Left => '<',
                            | Direction::Down => 'v',
                        });
                        continue 'x
                    }
                }
                s.push(match self.segment(x, y) {
                    | TrackSegment::Empty => ' ',
                    | TrackSegment::Horizontal => '-',
                    | TrackSegment::Vertical => '|',
                    | TrackSegment::Crossing => '+',
                    | TrackSegment::CurveUpRight => '/',
                    |  TrackSegment::CurveUpLeft => '\\',
                });
            }
            s += "\n"
        }
        write!(f, "{}", s)
    }

}

impl Track {
    fn segment(self: &Self, x: usize, y: usize) -> &TrackSegment {
        &self.segments[x + self.segments_x * y]
    }

    fn segment_mut(self: &mut Self, x: usize, y: usize) -> &mut TrackSegment {
        &mut self.segments[x + self.segments_x * y]
    }

    fn step(self: &mut Self) {
        self.step += 1;
        self.carts.sort_by(|a, b| {
            if a.y < b.y {
                Ordering::Less
            } else if a.y == b.y && a.x < a.y {
                Ordering::Less
            } else if a.y == b.y && a.x == a.y {
                Ordering::Equal
            } else {
                Ordering::Greater
            }
        });
        for i in 0..self.carts.len() {
            let (x,y) = (self.carts[i].x, self.carts[i].y);
            match self.segment(x, y) {
                | TrackSegment::Vertical => (),
                | TrackSegment::Horizontal => (),
                | TrackSegment::Crossing  => {
                    self.carts[i].turn()
                },
                | TrackSegment::CurveUpRight => { // '/'
                    self.carts[i].facing = match self.carts[i].facing {
                        | Direction::Up => Direction::Right,
                        | Direction::Right => Direction::Up,
                        | Direction::Left => Direction::Down,
                        | Direction::Down => Direction::Left,
                    }
                },
                | TrackSegment::CurveUpLeft => { // '\'
                    self.carts[i].facing = match self.carts[i].facing {
                        | Direction::Up => Direction::Left,
                        | Direction::Right => Direction::Down,
                        | Direction::Left => Direction::Up,
                        | Direction::Down => Direction::Right,
                    }
                },
                | TrackSegment::Empty => (),
            }
            self.carts[i].advance();
            for j in 0..self.carts.len() {
                if i != j && self.carts[i].x == self.carts[j].x && self.carts[i].y == self.carts[j].y {
                    if !self.carts[i].destroyed && !self.carts[i].destroyed {
                        println!("Collision! step={}, x,y = {},{}", self.step, self.carts[i].x, self.carts[i].y);
                        self.carts[i].destroyed = true;
                        self.carts[j].destroyed = true;
                    }
                    // if self.carts.len() == 1 + carts_to_remove.len() { // if we need to check out the last cart
                    //     for k in 0..self.carts.len() {
                    //         let mut present = false;
                    //         for l in 0..carts_to_remove.len() {
                    //             if carts_to_remove[l] == self.carts[k].id {
                    //                 present = true;
                    //             }
                    //         }
                    //         if !present {
                    //             println!("That's the end! step = {}, last cart = {},{}", self.step, self.carts[k].x, self.carts[k].y);
                    //             process::exit(0);
                    //         }
                    //     }
                    // }
                }
            }
        }

        let mut i = 0;
        while i != self.carts.len() {
            if self.carts[i].destroyed {
                self.carts.remove(i);
            } else {
                i += 1;
            }
        }

        if self.carts.len() == 1 {
            println!("That's the end! step = {}, last cart = {},{}", self.step, self.carts[0].x, self.carts[0].y);
            println!("{}", self);
            process::exit(0);
        }
    }

    fn from_file(f: String) -> Self {
        let mut segments_x = 0;
        let mut segments_y = 0;
        for (i, l) in f.lines().enumerate() {
            segments_x = cmp::max(segments_x, l.len());
            segments_y = i;
        }
        segments_y += 1;

        let segments = vec![TrackSegment::Empty; (segments_x + 1) * (segments_y + 1) + 1];
        let carts = Vec::new();
        let mut track = Track {
            segments,
            segments_x,
            segments_y: segments_y,
            carts,
            step: 0,
        };

        for (y, l) in f.lines().enumerate() {
            for (x, c) in l.chars().enumerate() {
                *track.segment_mut(x, y) = {
                    match c {
                        | ' ' => TrackSegment::Empty,
                        | '-' => TrackSegment::Horizontal,
                        | '|' => TrackSegment::Vertical,
                        | '+' => TrackSegment::Crossing,
                        | '/' => TrackSegment::CurveUpRight,
                        | '\\' => TrackSegment::CurveUpLeft,
                        | '>' => {
                            track.carts.push(Cart {
                                x, y, choice_turn:Direction::Left, facing: Direction::Right, destroyed: false
                            });
                            TrackSegment::Horizontal
                        },
                        | '<' => {
                            track.carts.push(Cart {
                                x, y, choice_turn:Direction::Left, facing: Direction::Left, destroyed: false
                            });
                            TrackSegment::Horizontal
                        },
                        | '^' => {
                            track.carts.push(Cart {
                                x, y, choice_turn:Direction::Left, facing: Direction::Up, destroyed: false
                            });
                            TrackSegment::Vertical
                        },
                        | 'v' => {
                            track.carts.push(Cart {
                                x, y, choice_turn:Direction::Left, facing: Direction::Down, destroyed: false
                            });
                            TrackSegment::Vertical
                        },
                        | _ => {
                            println!("WARNING: wrong char in file: {}", c);
                            TrackSegment::Empty
                        }
                    }
                };
            }
        }
        track
    }
}

fn main() -> std::io::Result<()> {
    let file = File::open("input")?;
    let mut buf_reader = BufReader::new(file);
    let mut full_file = String::new();
    buf_reader.read_to_string(&mut full_file)?;
    let mut track = Track::from_file(full_file);

    println!("{}", track);

    for _ in 0..50000 {
        track.step();
        // println!("{}", track);
    }

    Ok(())
}
