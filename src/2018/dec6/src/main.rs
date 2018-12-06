use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

// use std::cmp::Ordering;
// use std::fmt;

use std::str::FromStr;
use std::num::ParseIntError;
use std::iter;

struct Point {
    x: i64,
    y: i64,
}

impl FromStr for Point {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let splitted : Vec<&str> = s.trim().split(", ").collect();
        let x = splitted[0].parse::<i64>()?;
        let y = splitted[1].parse::<i64>()?;
        Ok(Point {x,y})
    }
}

fn distance_manhattan(a: &Point, b: &Point) -> i64 {
    (b.x - a.x).abs() + (b.y - a.y).abs()
}

fn main() -> std::io::Result<()> {
    let file = File::open("input")?;
    let mut buf_reader = BufReader::new(file);
    let mut full_file = String::new();
    buf_reader.read_to_string(&mut full_file)?;

    let mut points = Vec::new();

    for l in full_file.lines() {
        points.push(Point::from_str(l).unwrap());
    }

    let (mut x_min, mut x_max, mut y_min, mut y_max) = (points[0].x, points[0].x, points[0].y, points[0].y);

    for p in &points {
        if p.x < x_min {
            x_min = p.x
        }
        if p.y < y_min {
            y_min = p.y
        }
        if p.x > x_max {
            x_max = p.x
        }
        if p.y > y_max {
            y_max = p.y
        }
    }

    // so that we loop on the correct intervals
    x_max += 1;
    y_max += 1;

    println!("x_max={}, y_max={}", x_max, y_max);

    let mut count_points = vec![0; points.len()];

    let init_value = (None, x_max + y_max);
    let size = ((x_max + 1) * (y_max + 1) + 1) as usize;
    let mut field : Vec<(Option<i64>, i64)> = iter::repeat(init_value).take(size).collect() ;
    let field_index = |x, y| (x + x_max * y) as usize;
    for x in x_min..x_max {
        for y in y_min..y_max {
            let mut min_distance = x_max + y_max;
            let mut best_point = None;
            for (i, p) in points.iter().enumerate() {
                let distance = distance_manhattan(&p, &Point{x, y});
                if distance < min_distance {
                    best_point = Some(i as i64);
                    min_distance = distance
                } else if distance == min_distance {
                    best_point = None;
                }
            }
            field[field_index(x, y)] = (best_point, min_distance);
            if best_point.is_some() {
                count_points[best_point.unwrap() as usize] += 1
            }
        }
    }

    for (i, size) in count_points.iter().enumerate() {
        println!("Point {}, area {}", i, size);
    }

    // removing infinite domain points: they are on the edge
    for x in x_min..x_max {
        for y in [y_min, y_max-1].iter() {
            let opt = field[field_index(x,*y)].0;
            if opt.is_some() {
                count_points[ opt.unwrap() as usize] = 0
            }
        }
    }

    for y in y_min..y_max {
        for x in [x_min, x_max-1].iter() {
            let opt = field[field_index(*x,y)].0;
            if opt.is_some() {
                count_points[ opt.unwrap() as usize] = 0
            }
        }
    }

    let mut area_max = 0;
    let mut p = 0;
    for (i, size) in count_points.iter().enumerate() {
        if *size > area_max {
            area_max = *size;
            p = i;
        }
    }
    println!("Point {}, area {}", p, area_max);

    let mut nb_points_in_region = 0;
    let limit = 10000;
    for x in x_min..x_max {
        for y in y_min..y_max {
            let mut sum_distances = 0;
            for p in &points {
                sum_distances += distance_manhattan(&p, &Point{x, y})
            }
            if sum_distances < limit {
                nb_points_in_region +=1
            }
        }
    }

    println!("Region size: {}", nb_points_in_region);

    Ok(())
}
