use std::fs;

#[derive(Debug, Clone, Copy)]
struct Round {
    green: i32,
    red: i32,
    blue: i32,
}

#[derive(Debug)]
struct Game {
    index: i32,
    rounds: Vec<Round>,
}

impl Round {
    fn to_pair(string: &str) -> (i32, &str) {
        let strs: Vec<_> = string.split(" ").collect();
        let value = strs.get(1).unwrap().parse::<i32>().unwrap();
        let color = strs.last().unwrap();
        (value, color)
    }

    fn value_for_color<'a>(pairs: &'a Vec<(i32, &str)>, color: &str) -> i32 {
        *pairs
            .iter()
            .filter(|(_, p_color)| *p_color == color)
            .map(|(value, _)| *value)
            .collect::<Vec<_>>()
            .first()
            .unwrap_or(&0)
    }

    pub fn new(string: &str) -> Round {
        let pairs: Vec<_> = string
            .split(",")
            .map(|value| Round::to_pair(value))
            .collect();
        let green = Round::value_for_color(&pairs, "green");
        let blue = Round::value_for_color(&pairs, "blue");
        let red = Round::value_for_color(&pairs, "red");
        Round { green, red, blue }
    }
}

impl Game {
    pub fn new(string: &str) -> Game {
        let strings: Vec<_> = string.split(":").collect();
        let index = Game::index(strings.first().unwrap());
        let rounds: Vec<_> = strings
            .last()
            .unwrap()
            .split(";")
            .map(|s| Round::new(s))
            .collect();
        Game { index, rounds }
    }

    fn index(string: &str) -> i32 {
        let strings: Vec<_> = string.split(" ").collect();
        strings.last().unwrap().parse::<i32>().unwrap()
    }

    pub fn at_least_reds(&self) -> i32 {
        self.rounds.iter().map(|round| round.red).max().unwrap()
    }
    pub fn at_least_greens(&self) -> i32 {
        self.rounds.iter().map(|round| round.green).max().unwrap()
    }
    pub fn at_least_blues(&self) -> i32 {
        self.rounds.iter().map(|round| round.blue).max().unwrap()
    }

    pub fn power_of_least(&self) -> i32 {
        let green = self.at_least_greens();
        let blue = self.at_least_blues();
        let red = self.at_least_reds();
        green * blue * red
    }
}

pub fn advent_of_code_2_1() {
    let index_sum: i32 = get_games()
        .iter()
        .filter(|game| game.at_least_reds() <= 12)
        .filter(|game| game.at_least_greens() <= 13)
        .filter(|game| game.at_least_blues() <= 14)
        .map(|game| game.index)
        .sum();
    println!("Advent of code 2_1: {}", index_sum);
}

pub fn advent_of_code_2_2() {
    let power_sum: i32 = get_games().iter().map(|game| game.power_of_least()).sum();
    println!("Advent of code 2_2: {}", power_sum);
}

fn get_games() -> Vec<Game> {
    let contents = fs::read_to_string("data/day_2.txt").expect("should be there");
    contents.lines().map(|line| Game::new(line)).collect()
}
