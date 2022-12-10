use std::io::{BufRead, BufReader};
use std::{fs, io};

fn get_common_character(list_of_item_lists: Vec<&str>) -> Option<char> {
    let mut list_copy = list_of_item_lists.clone();
    let initial_coll = list_copy.first().unwrap().chars();
    list_copy.remove(0);
    for character in initial_coll {
        if list_copy.iter().all(|st| st.contains(character)) {
            return Some(character);
        }
    }

    None
}

fn convert_item_to_priority(character: char) -> i32 {
    let alphabet = String::from_utf8((b'a'..=b'z').chain(b'A'..=b'Z').collect()).unwrap();
    let place_in_alphabet = alphabet.chars().position(|c| c == character).unwrap() + 1;
    place_in_alphabet.try_into().unwrap()
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
    for line in lines {
        let number_of_items_per_compartment = line.len() / 2;
        let comp_1 = &line[..number_of_items_per_compartment];
        let comp_2 = &line[number_of_items_per_compartment..];
        let item_character = get_common_character(vec![comp_1, comp_2]).unwrap();
        total_result += convert_item_to_priority(item_character)
    }
    return total_result;
}

fn part2(file_path: &str) -> i32 {
    let binding = convert_file_to_lines(file_path);
    let chunk_lines = &binding.chunks(3).into_iter();
    let mut total_result = 0;
    chunk_lines.clone().for_each(|lines| {
        let item_character = get_common_character(vec![&lines[0], &lines[1], &lines[2]]).unwrap();
        total_result += convert_item_to_priority(item_character)
    });
    total_result
}

fn main() {
    println!(
        "The sum of priorities appearing in both cimpartments for each rucksack is {}",
        part1("./inputs/day3.txt")
    );
    println!(
        "The sum of the priorities of the elves badges is {}",
        part2("./inputs/day3.txt")
    );
}

#[test]
fn test_get_duplicates_from_strings() {
    let c: char = 'c';
    assert_eq!(get_common_character(vec!["abc", "cde"]), Some(c))
}

#[test]
fn test_part1() {
    assert_eq!(part1("./inputs/day3-test.txt"), 157);
}

#[test]
fn test_part2() {
    assert_eq!(part2("./inputs/day3-test.txt"), 70);
}
