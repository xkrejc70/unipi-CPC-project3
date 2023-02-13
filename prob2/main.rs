mod lib;

use std::io::{self, BufRead};

fn main() {
    let mut h: Vec<char> = Vec::new();

    h = parse_input();

    println!("{}", lib::patriotic_selections(h));
}

fn parse_input() -> Vec<char> {
    let mut input_lines: io::Lines<io::StdinLock> = io::stdin().lock().lines();
    let mut line_num: usize = 1;
    let mut num: usize = 0;
    let mut h: Vec<char> = Vec::new();

    while let Some(line) = input_lines.next() {
        let line = line.unwrap();

        if line.is_empty() {
            break;
        }

        match line_num {
            // 1st line (n)
            1 => num = parse_num(&line),
            // Houses
            _ => {
                h = parse_houses(num, &line);
            }
        }

        line_num += 1;
    }

    h
}

// Parse first line (n)
fn parse_num(str: &str) -> usize {
    if str.len() != 1 {
        invalid_input("first line length");
    }

    // Check number format
    if !is_valid_number(str) {
        invalid_input("number format");
    }

    let n: usize = str.parse().unwrap();

    n
}

// Parse houses line
fn parse_houses(num: usize, str: &str) -> Vec<char> {
    let mut h: Vec<char> = Vec::new();

    if str.len() != num {
        invalid_input("houses number");
    }

    for c in str.chars() {
        if !is_valid_light(c) {
            invalid_input("not a valid light");
        }
        h.push(c);
    }

    h
}

fn is_valid_number(num: &str) -> bool {
    matches!(num.parse::<usize>(), Ok(_n))
}

fn is_valid_light(c: char) -> bool {
    matches!(c, 'R' | 'W' | 'G' | 'X')
}

fn invalid_input(message: &'static str) {
    panic!("Invalid input! {}", message);
}
