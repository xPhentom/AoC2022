use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn calculate_calories_per_elf(file_path: &str) -> Vec<i32> {
    let file = File::open(file_path).expect("Could not open file");
    let buf_reader = BufReader::new(file);
    let mut calorie_list: Vec<i32> = vec![0];

    for line in buf_reader.lines() {
        let calorie: String = line.unwrap();
        if calorie.is_empty() {
            calorie_list.push(0)
        } else {
            let list_length = calorie_list.len() - 1;
            calorie_list[list_length] += calorie.parse::<i32>().unwrap();
        }
    }
    calorie_list
}

fn elf_with_highest_calorie(file_path: &str) -> i32 {
    return *calculate_calories_per_elf(file_path).iter().max().unwrap();
}

fn sum_top_3_elves_with_highest_calories(file_path: &str) -> i32 {
    let mut list_of_calories: Vec<i32> = calculate_calories_per_elf(file_path);
    list_of_calories.sort();
    list_of_calories.reverse();
    return list_of_calories.iter().take(3).sum();
}

fn main() {
    println!(
        "The elf with the highest calorie has the calorie intake of {} kcal",
        elf_with_highest_calorie("./inputs/day1.txt")
    );
    println!(
        "The top 3 elves with the highest calorie combined have the calorie intake of {} kcal",
        sum_top_3_elves_with_highest_calories("./inputs/day1.txt")
    )
}

#[test]
fn it_gets_the_highest_calories() {
    assert_eq!(elf_with_highest_calorie("./inputs/day1-test.txt"), 24000)
}

#[test]
fn it_gets_the_top_3_highest_calories() {
    assert_eq!(
        sum_top_3_elves_with_highest_calories("./inputs/day1-test.txt"),
        45000
    )
}
