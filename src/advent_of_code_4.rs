use std::fs;

#[derive(Debug, Clone)]
struct Card {
    index: usize,
    wins: Vec<u32>,
    mine: Vec<u32>,
}

impl Card {
    pub fn new(string: &str) -> Card {
        let first_split = string.split(":").collect::<Vec<&str>>();
        let index = Card::find_index(first_split.get(0).unwrap());
        let (wins, mine) = Card::find_wins_mine(first_split.get(1).unwrap());
        Card { index, wins, mine }
    }

    fn find_index(string: &str) -> usize {
        string.split(" ").last().unwrap().parse::<usize>().unwrap()
    }

    fn find_wins_mine(string: &str) -> (Vec<u32>, Vec<u32>) {
        let split = string.split("|").collect::<Vec<&str>>();
        let wins = Card::to_u32(split.get(0).unwrap());
        let mine = Card::to_u32(split.get(1).unwrap());
        (wins, mine)
    }

    fn to_u32(string: &str) -> Vec<u32> {
        string.split(" ").flat_map(|s| s.parse::<u32>()).collect()
    }

    fn mine_in_wins(&self) -> usize {
        self.mine
            .iter()
            .filter(|m| self.wins.contains(m))
            .collect::<Vec<_>>()
            .len()
    }
}

pub fn advent_of_code_4_1() {
    let cards = cards();
    let sum: usize = cards
        .iter()
        .map(|c| c.mine_in_wins())
        .filter(|n| n > &0)
        .map(|n| 2_usize.pow((n - 1) as u32))
        .sum();
    println!("Advent of code 4_1: {:?}", sum);
}

pub fn advent_of_code_4_2() {
    let cards = cards();
    let mut card_counts = cards
        .into_iter()
        .map(|c| (1_u32, c))
        .collect::<Vec<(u32, Card)>>();
    for i in 0..card_counts.len() {
        let n = card_counts[i].0;
        let points = card_counts[i].1.mine_in_wins();
        if points > 0 {
            for j in 0..points {
                if i + j + 1 >= card_counts.len() {
                    continue;
                }
                card_counts[i + j + 1].0 += n;
            }
        }
    }
    let sum = card_counts.into_iter().map(|(n, c)| n).sum::<u32>();
    println!("Advent of code 4_2: {:?}", sum);
}

fn cards() -> Vec<Card> {
    let contents = fs::read_to_string("data/day_4.txt").unwrap();
    contents
        .lines()
        .map(|line| Card::new(line))
        .collect::<Vec<Card>>()
}
