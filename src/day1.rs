use crate::timed;

pub fn solve() {
    println!("Day1:");
    timed(solve_part1);
    timed(solve_part2);
}
pub fn solve_part1() {
    let codes = include_str!("res/day1/calibration_codes.txt");
    let mut sum = 0;
    for line in codes.split('\n') {
        if line.is_empty() {continue}
        let mut first_digit = None;
        let mut second_digit = None;
        for char in line.chars() {
            if char.is_ascii_digit() {
                if first_digit.is_none() {
                    first_digit = Some(char as u32 - '0' as u32);
                    second_digit = first_digit;
                } else {
                    second_digit = Some(char as u32 - '0' as u32);
                }
            } 
        }
        sum += first_digit.unwrap() * 10 + second_digit.unwrap();
    }
    println!("\tPart 1: sum of calibration codes: {0}", sum);
}
pub fn solve_part2() {
    let codes = include_str!("res/day1/calibration_codes.txt");
    let mut sum = 0;
    let numbers = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    for line in codes.split('\n') {
        if line.is_empty() {continue}
        let mut first_digit = None;
        let mut second_digit = None;
        for (idx, char) in line.chars().enumerate() {
            let mut current_digit = None;
            if char.is_ascii_digit() {
                current_digit = Some(char as u32 - '0' as u32);
            } else {
                for (n, num_str) in numbers.iter().enumerate() {
                    if line[0..(idx + 1)].ends_with(num_str) {
                        current_digit = Some(n as u32 + 1);
                        break;
                    }
                } 
            }
            if current_digit.is_none() {continue}
            if first_digit.is_none() {
                first_digit = current_digit;
                second_digit = current_digit;
            }
            else {
                second_digit = current_digit;
            }
        }
        sum += first_digit.unwrap() * 10 + second_digit.unwrap();
    }
    println!("\tPart 2: sum of calibration codes including number names: {0}", sum);
}
