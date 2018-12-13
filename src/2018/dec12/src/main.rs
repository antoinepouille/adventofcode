use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::collections::HashMap;

struct State(Vec<bool>);
struct Rule(State, bool);

fn parse_initial_state(l: &str) -> State {
    State(l.trim_start_matches("initial state: ").chars()
        .map(|c| if c == '#' {true} else {false}).collect())
}

fn parse_rule(l: &str) -> Rule {
    let a : Vec<&str> = l.trim().split(" => ").collect();
    Rule(parse_initial_state(a[0]), a[1].chars().next().unwrap() == '#')
}

fn parse_file(file: String) -> (State, Vec<Rule>) {
    let mut it = file.lines();
    let initial_state = parse_initial_state(it.next().unwrap());
    let mut rules = Vec::new();
    it.next();
    while let Some(r) = it.next() {
        rules.push(parse_rule(r));
    }
    // rules.sort_by(|r, s| r.0.cmp(s.0));
    (initial_state, rules)
}

fn string_of_state(v: &State) -> String {
    let mut s = String::new();
    let State(vec) = v;
    for &b in vec {
        if b {
            s += "#";
        } else {
            s += ".";
        }
    }
    s
}

fn string_of_rule(r: &Rule) -> String {
    string_of_state(&r.0) + " => " + (if r.1 {"#"} else {"."})
}

fn _apply_rule_to_slice5(state: Vec<bool>, rules: &Vec<Rule>) -> bool {
    for r in rules {
        let mut equal = true;
        for (i, &b) in state.iter().enumerate() {
            if b != (r.0).0[i as usize] {
                equal = false
            }
        }
        if equal {
            return r.1
        }
    }
    assert!(false);
    false
}

fn _get_slice5(s: &Vec<bool>, i: i32) -> Vec<bool> {
    let mut v = Vec::new();
    for a in (i-2)..(i+3) {
        if a < 0 || a as usize >= s.len() {
            v.push(false);
        } else  {
            v.push(s[a as usize]);
        }
    }
    v
}

fn _run_automata(state: State, rules: Vec<Rule>, n: u64) -> (State, i32) {
    let mut current = state.0.to_vec();
    let mut next = Vec::new();
    let mut offset = 0;
    for gen in 0..n {
        if gen % 1000 == 0 {
            println!("gen={}", gen + 1);
        }
        for i in -1..(current.len() + 1) as i32 {
            next.push(_apply_rule_to_slice5(_get_slice5(&current, i), &rules));
        }
        offset -= 1;
        current = next.to_vec();
        next.truncate(0);
    }
    (State(current), offset)
}

// RuleGraph registers all states (5 chars) in the form of a graph. Links are provided when reading
// a char so that we get into the right next state. Each state as an id so that accesses are fast
struct RuleGraph {
    current_state: usize,
    init_state: usize, // '.....'
    result: Vec<bool>, // get the next value of the center char when index = id
    advance_true: Vec<usize>, // get the next state when reading a '#'
    advance_false: Vec<usize>, // get the next state when reading a '.'
}

impl<'a> RuleGraph {
    fn advance(self: &'a mut Self, c: bool) {
        if c {
            self.current_state = self.advance_true[self.current_state];
        } else {
            self.current_state = self.advance_false[self.current_state];
        }
    }

    fn result(self: &'a Self) -> bool{
        self.result[self.current_state]
    }

    fn reset(self: &'a mut Self) {
        self.current_state = self.init_state
    }

    fn of_rules(rules: &'a Vec<Rule>) -> Self {
        let mut id_count = 0;
        let mut result = Vec::new();
        let mut id_of_string = HashMap::new();
        for r in rules {
            let s = string_of_state(&r.0);
            let _ = id_of_string.insert(s, id_count);
            result.push(r.1);
            id_count += 1;
        }

        let mut advance_true = vec![0; id_count];
        let mut advance_false = vec![0; id_count];
        for r in rules {
            let mut s = string_of_state(&r.0);
            let i = *id_of_string.get(&s).unwrap();
            let _ = s.remove(0);
            s.push('.');
            advance_false[i] = *id_of_string.get(&s).unwrap();
            let _ = s.pop();
            s.push('#');
            advance_true[i] = *id_of_string.get(&s).unwrap();
        }

        let init_state = *id_of_string.get(".....").unwrap();
        let current_state = init_state;

        RuleGraph {
            current_state,
            init_state,
            result,
            advance_true,
            advance_false,
        }
    }
}


fn run_automata2(state: State, rules: Vec<Rule>, n: u64) -> (State, i64) {
    let margin1 = 1000;
    let margin2 = 10000000;
    let size = margin1 + margin2  + state.0.len();
    let mut data1 = vec![false; size];
    let mut data2 = vec![false; size];
    for i in 0..state.0.len() {
        data1[margin1 + i] = state.0[i];
    }

    let mut rule_graph = RuleGraph::of_rules(&rules);

    let mut data_current_is_1 = true;
    let mut start = margin1;
    let mut end = margin1 + state.0.len();
    for gen in 0..n {
        let (current, next) = {
            if data_current_is_1 {
                (&mut data1[start..end],
                &mut data2[(start-1)..(end+1)])
            } else {
                (&mut data2[start..end],
                &mut data1[(start-1)..(end+1)])
            }};
        data_current_is_1 = !data_current_is_1;

        if gen % 10000 == 0 {
            println!("gen={}, len={}, start={}, end={}", gen, end - start, start, end);
        }
        println!("{}",string_of_state(&State(current.to_vec())));

        rule_graph.reset();
        rule_graph.advance(current[0]);
        for i in 1..current.len() {
            rule_graph.advance(current[i]);
            next[i-1] = rule_graph.result();
        }
        for i in 0..3 {
            rule_graph.advance(false);
            next[current.len() + i - 1] = rule_graph.result();
        }

        start -= 1;
        end += 1;

        // cleanup every 100
        if gen % 100 == 0 {
            let mut first: i64 = 0;
            for (i, &b) in next.iter().enumerate() {
                if b {
                    first = i as i64 ;
                    break;
                }
            }
            let mut last = 0;
            for (i, &b) in next.iter().enumerate().rev() {
                if b {
                    last = i;
                    break;
                }
            }
            end = start + last + 2;
            start = ((start as i64) + first - 1) as usize;
        }
    }
    let output = {
        if data_current_is_1 {
            &mut data1[start..end]
        } else {
            &mut data2[start..end]
        }};
    (State(output.to_vec()), (start as i64) - (margin1 as i64))
}

fn main() -> std::io::Result<()> {
    let file = File::open("input")?;
    let mut buf_reader = BufReader::new(file);
    let mut full_file = String::new();
    buf_reader.read_to_string(&mut full_file)?;

    let (state, rules) = parse_file(full_file);
    println!("{}",string_of_state(&state));
    for r in &rules {
        println!("{}", string_of_rule(r));
    }

    // 3230
    // let (end, offset) = _run_automata(state, rules, 20);
    // let (end, offset) = run_automata2(state, rules, 20);

    let first_target = 200;
    let interpolation : u64 = 50000000000;
    let (end, offset) = run_automata2(state, rules, first_target);
    let offset : i64 = offset as i64 + (interpolation as i64 - first_target as i64);

    println!("{}",string_of_state(&end));

    let mut acc : i64 = 0;
    for (i, &b) in end.0.iter().enumerate() {
        if b {
            acc += (i as i64) + offset
        }
    }
    // 4400000000304 ?: yes!
    println!("acc={}", acc);

    Ok(())
}
