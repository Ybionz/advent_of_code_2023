use std::{fs, i64};

#[derive(Debug, Clone, Copy)]
struct Map {
    des_start: i64,
    source_start: i64,
    size: i64,
    jump: i64,
    source_end: i64,
}

impl Map {
    fn new(str: &str) -> Map {
        let nums = str.split(" ").collect::<Vec<_>>();
        let des_start = nums.get(0).unwrap().parse::<i64>().unwrap();
        let source_start = nums.get(1).unwrap().parse::<i64>().unwrap();
        let size = nums.get(2).unwrap().parse::<i64>().unwrap();
        let jump =  des_start - source_start;
        let source_end = source_start + size;
        Map {
            des_start,
            source_start,
            size,
            jump,
            source_end,
        }
    }

    fn translate(self, num: i64) -> i64 {
        self.jump + num
    }

    fn contains(self, num: i64) -> bool {
        self.source_start <= num && num <= self.source_end
    }
}
#[derive(Debug)]
struct MapSet {
    maps: Vec<Map>,
}

impl MapSet {
    fn new(str: &str) -> MapSet {
        let strs = str.split("\n").collect::<Vec<_>>();
        let maps = strs
            .split_at(1)
            .1
            .iter()
            .filter(|s| s.len() > 0)
            .map(|s| Map::new(s))
            .collect::<Vec<Map>>();
        MapSet { maps }
    }
    fn translate(&self, num: i64) -> i64 {
        let t = self.maps.iter().find(|m| m.contains(num));
        match t {
            Some(map) => return map.translate(num),
            None => return num,
        }
    }
    fn translate_vec(&self, nums: Vec<i64>) -> Vec<i64> {
       nums.iter().map(|num| self.translate(*num)).collect::<Vec<i64>>()
    }
}

pub fn advent_of_code_5_1() {
    let contents = fs::read_to_string("data/day_5.txt").unwrap();
    let splits = contents.split("\n\n").collect::<Vec<&str>>();
    let seeds = seeds(&splits.get(0).unwrap());
    let map_sets = splits
        .split_at(1)
        .1
        .iter()
        .map(|s| MapSet::new(s))
        .collect::<Vec<MapSet>>();
    let translated_seeds = map_seeds(seeds, map_sets);
    println!("Advent of Code 5_1: {}", translated_seeds.into_iter().min().unwrap());
}

pub fn advent_of_code_5_2() {}

fn map_seeds(seeds: Vec<i64>, map_sets: Vec<MapSet>) -> Vec<i64>{
    let mut seeds = seeds;
    for map_set in map_sets {
        seeds = map_set.translate_vec(seeds);
    }
    seeds
}

fn seeds(str: &str) -> Vec<i64> {
    str.split(":")
        .collect::<Vec<&str>>()
        .get(1)
        .unwrap()
        .split(" ")
        .flat_map(|s| s.parse::<i64>())
        .collect::<Vec<i64>>()
}
