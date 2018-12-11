fn power(x: i32, y: i32, serial: i32) -> i32 {
    let rack_id = x + 10;
    let power_level = ((rack_id * y) + serial) * rack_id;
    (((power_level % 1000) - (power_level % 100)) / 100) - 5
}

fn power3x3(x0: i32, y0: i32, serial: i32) -> i32 {
    let mut acc = 0;
    for x in 0..3 {
        for y in 0..3 {
            acc += power(x0 + x, y0 + y, serial);
        }
    }
    acc
}

fn max_grid(size_x: i32, size_y: i32, serial: i32) -> (i32, i32) {
    let mut best_value = -10000;
    let mut x_v = -1;
    let mut y_v = -1;
    for x in 1..(size_x - 3 + 1) {
        for y in 1..(size_y - 3 + 1) {
            let v = power3x3(x, y, serial);
            if v > best_value {
                best_value = v;
                x_v = x;
                y_v = y;
            }
        }
    }
    (x_v, y_v)
}

fn biggest_square(size: i32, serial: i32) -> (i32, i32, i32) {
    let size2 : i32 = size * size;
    let size3 : i32 = size2 * size;
    // 3-dim array: x,y,d -> value, size = 300*300*300
    let mut values = vec![0; size3 as usize];
    // let index : Fn(i32,i32,i32) -> usize = |x,y,d| (size2 * x + size*y + d) as usize;
    let index2 = |x: i32, y: i32| ((x - 1) + size * (y - 1)) as usize;
    let index = |x: i32, y: i32, d: i32| ((x - 1) + size * (y - 1) + (d - 1) * size2) as usize;

    let mut best_value = -10000;
    let mut x_v = -1;
    let mut y_v = -1;
    let mut d_v = -1;
    for d in 1..(size + 1) {
        for x in 1..(size + 1 - (d - 1)) {
            for y in 1..(size + 1 - (d - 1)) {
                if d == 1 {
                    values[index2(x, y)] = power(x, y, serial);
                } else {
                    // first try: let's add the new borders ?
                    let mut border_x = 0;
                    for i in x..(x+d - 1) {
                        border_x += values[index2(i, y + d - 1)]
                    }
                    let mut border_y = 0;
                    for j in y..(y+d - 1) {
                        border_y += values[index2(x + d - 1, j)]
                    }
                    values[index(x,y,d)] = values[index(x,y,d-1)]
                        + border_x + border_y + values[index2(x+d-1, y+d-1)];
                }
                if values[index(x,y,d)] > best_value {
                    best_value = values[index(x,y,d)];
                    x_v = x;
                    y_v = y;
                    d_v = d;
                }
            }
        }
    }
    (x_v, y_v, d_v)
}

fn main() {
    let serial = 9424;
    let max = max_grid(300, 300, serial);
    println!("max coord = {:?}", max);

    let sol = biggest_square(300, serial);
    println!("best coords = {:?}", sol);
}
