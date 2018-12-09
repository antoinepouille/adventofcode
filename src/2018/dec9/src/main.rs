use std::collections::LinkedList;

fn v1() {
    let nb_players = 441;
    // let last_marble_worth = 25;
    let last_marble_worth = 7103200;
    let mut scores = vec![0; nb_players];

    let mut circle = Vec::new();
    circle.push(0);
    circle.push(1);
    let mut current = 1;
    let mut player = 0;
    for marble in 2..(last_marble_worth + 1) {
        if marble % 23 != 0 {
            let location = (current + 2) % (circle.len());
            if location > 0 {
                circle.insert(location, marble);
                current = location;
            } else {
                circle.push(marble);
                current = circle.len() - 1;
            }
        } else {
            scores[player] += marble;
            let location = (current + circle.len()- 7) % (circle.len());
            scores[player] += circle[location];
            circle.remove(location);
            current = location;
        }
        player = (player + 1) % nb_players
    }


    //println!("{:?}", circle);
    println!("{}", scores.iter().max().unwrap());
}

fn insert_ll(l: &mut LinkedList<u32>, n: usize, v: u32) {
    let mut rest = l.split_off(n);
    rest.push_front(v);
    l.append(&mut rest);
}

/*
fn nth<V>(l: &LinkedList<V>, n: usize) -> V{
    let mut iter = l.iter();
    for _ in 0..(n-1) {
        iter.next();
    }
    *iter.next().unwrap()
}
*/

fn delete_ll<V>(l: &mut LinkedList<V>, n: usize) -> V{
    let mut rest = l.split_off(n);
    let v = rest.pop_front().unwrap();
    l.append(&mut rest);
    v
}

fn v2() {
    let nb_players = 441;
    let last_marble_worth = 25;
    let last_marble_worth = 7103200;
    let mut scores = vec![0; nb_players];

    let mut circle = LinkedList::new();
    circle.push_back(0);
    circle.push_back(1);
    let mut current = 1;
    let mut player = 0;
    for marble in 2..(last_marble_worth + 1) {
        if marble % 1000 == 0 {
            println!("current={}", marble);
        }
        if marble % 23 != 0 {
            let location = (current + 2) % (circle.len());
            if location > 0 {
                insert_ll(&mut circle, location, marble);
                current = location;
            } else {
                circle.push_back(marble);
                current = circle.len() - 1;
            }
        } else {
            scores[player] += marble;
            let location = (current + circle.len()- 7) % (circle.len());
            let value = delete_ll(&mut circle, location);
            scores[player] += value;
            current = location;
        }
        player = (player + 1) % nb_players
    }


    // println!("{:?}", circle);
    println!("{}", scores.iter().max().unwrap());
}

fn adv_circle<V>(l: &mut LinkedList<V>) {
    let v = l.pop_front().unwrap();
    l.push_back(v);
}

fn rec_circle<V>(l: &mut LinkedList<V>) {
    let v = l.pop_back().unwrap();
    l.push_front(v);
}

fn test_list() {
    let mut l = LinkedList::new();
    l.push_back(0);
    l.push_back(1);
    l.push_back(2);
    l.push_back(3);
    l.push_back(4);
    insert_ll(&mut l, 1, 5);
    delete_ll(&mut l, 3);
    println!("{:?}", l);
}

fn v3() {
    let nb_players = 441;
    // let last_marble_worth = 25;
    // let last_marble_worth = 71032;
    let last_marble_worth = 7103200;
    let mut scores = vec![0u64; nb_players];

    let mut circle = LinkedList::new();
    circle.push_back(0);
    circle.push_back(1);
    // move to second element:
    adv_circle(&mut circle);
    let mut player = 0;
    for marble in 2..(last_marble_worth + 1) {
        if marble % 10000 == 0 {
            println!("current={}, player = {}", marble, player);
        }
        if marble % 23 != 0 {
            adv_circle(&mut circle);
            adv_circle(&mut circle);
            circle.push_front(marble);
        } else {
            scores[player] += marble;
            for _ in 0..7 {
                rec_circle(&mut circle);
            }
            let value = circle.pop_front().unwrap();
            scores[player] += value;
        }

        player = (player + 1);
        player = player % nb_players
    }


    println!("{}", scores.iter().max().unwrap());
}

fn main() {
    // test_list();
    v3()
}
