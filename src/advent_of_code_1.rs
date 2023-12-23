use std::fs;

struct NumberIndex {
    index: usize,
    number: u32,
}

impl NumberIndex {
    pub fn new(index: usize, number: u32) -> NumberIndex {
        NumberIndex { index, number }
    }
}

pub fn advent_of_code_1_1() {
    let sum: u32 = find_sum(false);
    println!("Advent of code 1_1: {}", sum);
}

pub fn advent_of_code_1_2() {
    let sum: u32 = find_sum(true);
    println!("Advent of code 1_2: {}", sum);
}

fn find_sum(with_words: bool) -> u32 {
    let contents = get_contents();
    contents
        .lines()
        .map(|line| find_number_for_line(line, with_words))
        .sum()
}

fn get_contents() -> String {
    fs::read_to_string("data/day_1.txt").expect("File should exist")
}

fn find_number_for_line(line: &str, with_words: bool) -> u32 {
    let mut number_indexs = number_index_for_digits(&line);
    if with_words {
        number_indexs.append(&mut number_index_for_words(&line));
    }
    number_indexs.sort_by(|a, b| a.index.cmp(&b.index));
    let first_number = number_indexs.first().map(|ni| ni.number).unwrap_or(0);
    let last_number = number_indexs.last().map(|ni| ni.number).unwrap_or(0);
    first_number * 10 + last_number
}

fn number_index_for_digits(line: &str) -> Vec<NumberIndex> {
    line.chars()
        .enumerate()
        .filter_map(|(i, c)| char_to_number_index(i, c))
        .collect::<Vec<_>>()
}

fn char_to_number_index(i: usize, c: char) -> Option<NumberIndex> {
    let n = c.to_digit(10);
    match n {
        Some(n) => Some(NumberIndex {
            index: i,
            number: n,
        }),
        None => None,
    }
}

fn number_index_for_words(line: &str) -> Vec<NumberIndex> {
    vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ]
    .iter()
    .enumerate()
    .flat_map(|(number, text)| {
        line.match_indices(text)
            .map(|(i, _)| i)
            .map(move |i| NumberIndex::new(i, u32::try_from(number + 1).unwrap()))
    })
    .collect()
}
