use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
    vx: i32,
    vy: i32,
}

impl Point {
    fn next(self: &Self) -> Point {
        let x = self.x + self.vx;
        let y = self.y + self.vy;
        let vx = self.vx;
        let vy = self.vy;
        Point {x, y, vx, vy}
    }
}

fn parse_line(l: &str) -> Point {
    let two : Vec <&str> = l.trim_start_matches("position=<").trim_end_matches(">").split("> velocity=<").collect();
    let pos : Vec <&str> = two[0].split(", ").collect();
    let vit : Vec <&str> = two[1].split(", ").collect();
    Point {
        x: pos[0].trim().parse::<i32>().unwrap(),
        y: pos[1].trim().parse::<i32>().unwrap(),
        vx: vit[0].trim().parse::<i32>().unwrap(),
        vy: vit[1].trim().parse::<i32>().unwrap(),
    }
}

struct Field {
    points: Vec<Point>,
    time: u32,
}

impl Field {
    fn of_vec(v: Vec<Point>) -> Self {
        Field {
            points: v,
            time: 0,
        }
    }

    fn step(self: &mut Self) {
        for p in &mut self.points {
            *p = p.next();
        }
        self.time += 1
    }

    fn draw(self: &mut Self) {
        let mut min_x = self.points[0].x;
        let mut max_x = min_x;
        let mut min_y = self.points[0].y;
        let mut max_y = min_y;
        for p in &self.points {
            if p.x < min_x {
                min_x = p.x
            } else if p.x > max_x {
                max_x = p.x
            }
            if p.y < min_y {
                min_y = p.y
            } else if p.y > max_y {
                max_y = p.y
            }
        }

        max_x += 1;
        max_y += 1;

        if (max_x - min_x) < 100 && (max_y - min_y) < 30 {
            let mut pic = vec![false; ((max_x - min_x + 1) * (max_y - min_y +1)) as usize];
            let access = |x, y| (x - min_x + (max_x - min_x + 1) * (y - min_y)) as usize;

            for p in &self.points {
                pic[access(p.x, p.y)] = true;
            }

            println!("{}: {}:{}, {}:{}", self.time, min_x, max_x, min_y, max_y);

            for y in min_y..max_y  {
                let mut s = String::new();
                for x in min_x..max_x {
                    if pic[access(x, y)] {
                        s.push('O');
                    } else {
                        s.push(' ');
                    }
                }
                println!("{}", s);
            }
        }
    }
}

fn main() -> std::io::Result<()> {
    let file = File::open("input")?;
    let mut buf_reader = BufReader::new(file);
    let mut full_file = String::new();
    buf_reader.read_to_string(&mut full_file)?;
    let entries: Vec<Point>  = full_file.lines().map(|l| parse_line(l)).collect();
    let mut field = Field::of_vec(entries);
    for _i in 0..12000 {
        field.step();
        field.draw();
    }


    Ok(())
}
