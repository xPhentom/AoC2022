use std::io::{BufRead, BufReader};
use std::{fs, io};

fn part1(my_choice: &str, their_choice: &str) -> i32 {
    let choices = [my_choice, their_choice];
    let result = match choices {
        ["A", "X"] => 1 + 3,
        ["A", "Y"] => 2 + 6,
        ["A", "Z"] => 3 + 0,
        ["B", "X"] => 1 + 0,
        ["B", "Y"] => 2 + 3,
        ["B", "Z"] => 3 + 6,
        ["C", "X"] => 1 + 6,
        ["C", "Y"] => 2 + 0,
        ["C", "Z"] => 3 + 3,
        _ => 0,
    };
    result
}

fn part2(my_choice: &str, their_choice: &str) -> i32 {
    let mut choices = [my_choice, their_choice];
    choices = match choices {
        ["A", "X"] => ["A", "Z"],
        ["A", "Y"] => ["A", "X"],
        ["A", "Z"] => ["A", "Y"],
        ["B", "X"] => ["B", "X"],
        ["B", "Y"] => ["B", "Y"],
        ["B", "Z"] => ["B", "Z"],
        ["C", "X"] => ["C", "Y"],
        ["C", "Y"] => ["C", "Z"],
        ["C", "Z"] => ["C", "X"],
        _ => ["", ""],
    };
    let result = part1(choices[0], choices[1]);
    result
}

fn read_file(file_path: &str, battle_function: &dyn Fn(&str, &str) -> i32) -> i32 {
    let file = fs::File::open(file_path).expect("Unable to open file");
    let file_reader = BufReader::new(file);
    let lines: Vec<String> = file_reader.lines().filter_map(io::Result::ok).collect();
    let mut total_result = 0;
    for line in lines {
        let split_line = line.split(" ").collect::<Vec<&str>>();
        let my_choice: &str = split_line[0];
        let their_choice: &str = split_line[1];
        total_result += battle_function(my_choice, their_choice);
    }
    return total_result;
}

fn main() {
    println!("{}", read_file("./inputs/day2.txt", &part1));
    println!("{}", read_file("./inputs/day2.txt", &part2));
}

#[test]
fn test_points_received_when_draw() {
    assert_eq!(part1("A", "X"), 4);
    assert_eq!(part1("B", "Y"), 5);
    assert_eq!(part1("C", "Z"), 6)
}

#[test]
fn test_points_received_when_won() {
    assert_eq!(part1("A", "Z"), 7);
    assert_eq!(part1("B", "X"), 8);
    assert_eq!(part1("C", "Y"), 9)
}

#[test]
fn test_points_received_when_lost() {
    assert_eq!(part1("A", "Y"), 8);
    assert_eq!(part1("B", "Z"), 9);
    assert_eq!(part1("C", "X"), 7)
}

#[test]
fn test_part_1_with_test_input() {
    assert_eq!(read_file("./inputs/day2-test.txt", &part1), 15);
}

#[test]
fn test_part_2_with_test_input() {
    assert_eq!(read_file("./inputs/day2-test.txt", &part2), 12);
}
