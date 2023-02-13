mod lib;

use std::io::{self, BufRead};

fn main() {
    let mut cities: Vec<Vec<usize>> = Vec::new();
    let num_cities: usize;
    let num_days: usize;

    // Parse input
    (num_cities, num_days, cities) = parse_input();

    println!("{}", lib::opt_method(num_cities, num_days, cities));
}

fn parse_input() -> (usize, usize, Vec<Vec<usize>>) {
    let mut input_lines: io::Lines<io::StdinLock> = io::stdin().lock().lines();
    let mut line_num: usize = 1;
    let mut city_num: usize = 0;
    let mut n: usize = 0;
    let mut d: usize = 0;
    let mut cities: Vec<Vec<usize>> = Vec::new();

    while let Some(line) = input_lines.next() {
        let line = line.unwrap();

        if line.is_empty() {
            break;
        }

        match line_num {
            // 1st line (n, D)
            1 => (n, d) = parse_nums(&line),
            // Cities
            _ => {
                cities.push(parse_city(d, &line));
                city_num += 1;
            }
        }

        if city_num > n {
            invalid_input("number of cities");
        }

        line_num += 1;
    }

    if city_num < n {
        invalid_input("number of cities");
    }

    (n, d, cities)
}

// Parse first line (n, D)
fn parse_nums(str: &str) -> (usize, usize) {
    let nums: Vec<String> = split_by_space(str);

    if nums.len() != 2 {
        invalid_input("first line length");
    }

    // Check number format
    if !is_valid_number(&nums[0]) || !is_valid_number(&nums[1]) {
        invalid_input("number format");
    }

    let n: usize = nums[0].parse().unwrap();
    let d: usize = nums[1].parse().unwrap();

    (n, d)
}

// Parse city line
fn parse_city(d: usize, city: &str) -> Vec<usize> {
    let city: Vec<String> = split_by_space(city);
    let mut city_res: Vec<usize> = Vec::new();

    if city.len() < d {
        invalid_input("city attraction number");
    }

    let mut total_attrs = 0;
    for elem in city.iter() {
        if !is_valid_number(elem) {
            invalid_input("not a number");
        }
        total_attrs = total_attrs + elem.parse::<usize>().unwrap();
        city_res.push(total_attrs);
    }

    city_res
}

fn split_by_space(str: &str) -> Vec<String> {
    return str.split(' ').map(|s| s.to_string()).collect();
}

fn is_valid_number(num: &str) -> bool {
    matches!(num.parse::<usize>(), Ok(_n))
}

fn invalid_input(message: &'static str) {
    panic!("Invalid input! {}", message);
}
