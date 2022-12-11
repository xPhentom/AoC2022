use std::io::{BufRead, BufReader};

use std::{fs, io};

fn assignment_pairs_overlap(first_pair: [i32; 2], second_pair: [i32; 2]) -> bool {
    (first_pair[0]..=first_pair[1]).any(|num| (second_pair[0]..=second_pair[1]).contains(&num))
}

fn assignment_pairs_fully_overlap(first_pair: [i32; 2], second_pair: [i32; 2]) -> bool {
    (first_pair[0]..=first_pair[1]).all(|num| (second_pair[0]..=second_pair[1]).contains(&num))
        || (second_pair[0]..=second_pair[1])
            .all(|num| (first_pair[0]..=first_pair[1]).contains(&num))
}

fn convert_file_to_lines(file_path: &str) -> Vec<String> {
    let file = fs::File::open(file_path).expect("Unable to open file");
    let file_reader = BufReader::new(file);
    let lines: Vec<String> = file_reader.lines().filter_map(io::Result::ok).collect();
    lines
}

fn part1(file_path: &str) -> i32 {
    let lines = &convert_file_to_lines(file_path);
    let mut total_result = 0;
    for l in lines {
        let pair_groups = l.split(',');
        let pair_1: Vec<i32> = pair_groups
            .clone()
            .nth(0)
            .unwrap()
            .split('-')
            .map(|x| x.parse::<i32>().unwrap())
            .collect();
        let pair_2: Vec<i32> = pair_groups
            .clone()
            .nth(1)
            .unwrap()
            .split('-')
            .map(|x| x.parse::<i32>().unwrap())
            .collect();
        if assignment_pairs_fully_overlap([pair_1[0], pair_1[1]], [pair_2[0], pair_2[1]]) {
            total_result += 1;
        }
    }
    return total_result;
}

fn part2(file_path: &str) -> i32 {
    let lines = &convert_file_to_lines(file_path);
    let mut total_result = 0;
    for l in lines {
        let pair_groups = l.split(',');
        let pair_1: Vec<i32> = pair_groups
            .clone()
            .nth(0)
            .unwrap()
            .split('-')
            .map(|x| x.parse::<i32>().unwrap())
            .collect();
        let pair_2: Vec<i32> = pair_groups
            .clone()
            .nth(1)
            .unwrap()
            .split('-')
            .map(|x| x.parse::<i32>().unwrap())
            .collect();
        if assignment_pairs_overlap([pair_1[0], pair_1[1]], [pair_2[0], pair_2[1]]) {
            total_result += 1;
        }
    }
    return total_result;
}

fn main() {
    println!(
        "There are {} assignemnt pairs overlapping.",
        part1("./inputs/day4.txt")
    );
    println!(
        "There are {} assignemnt pairs partially overlapping.",
        part2("./inputs/day4.txt")
    );
}

#[test]
fn test_assignments_overlap() {
    assert_eq!(assignment_pairs_overlap([1, 3], [2, 4]), true);
    assert_eq!(assignment_pairs_overlap([1, 3], [3, 4]), true);
    assert_eq!(assignment_pairs_overlap([2, 3], [1, 6]), true);
    assert_eq!(assignment_pairs_overlap([1, 3], [4, 6]), false)
}

#[test]
fn test_assignments_fully_overlap() {
    assert_eq!(assignment_pairs_fully_overlap([1, 3], [2, 4]), false);
    assert_eq!(assignment_pairs_fully_overlap([1, 3], [3, 4]), false);
    assert_eq!(assignment_pairs_fully_overlap([2, 3], [1, 6]), true);
    assert_eq!(assignment_pairs_fully_overlap([1, 3], [4, 6]), false)
}

#[test]
fn test_part1() {
    assert_eq!(part1("./inputs/day4-test.txt"), 2);
}

#[test]
fn test_part2() {
    assert_eq!(part2("./inputs/day4-test.txt"), 4);
}
