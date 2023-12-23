use rstar::{PointDistance, RTree, RTreeObject, AABB};
use std::{collections::HashMap, fs};
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    col: isize,
    row: isize,
}

#[derive(Debug)]
struct Symbol {
    point: Point,
    symbol: char,
}

#[derive(Debug)]
struct Number {
    start_point: Point,
    end_point: Point,
    value: usize,
}

impl Point {
    pub fn new(row: usize, col: usize) -> Point {
        Point {
            col: col as isize,
            row: row as isize,
        }
    }
    pub fn coordinate(&self) -> [isize; 2] {
        [self.col, self.row]
    }
}

impl RTreeObject for Point {
    type Envelope = AABB<[isize; 2]>;

    fn envelope(&self) -> Self::Envelope {
        AABB::from_point([self.col, self.row])
    }
}

impl PointDistance for Point {
    fn distance_2(
        &self,
        point: &<Self::Envelope as rstar::Envelope>::Point,
    ) -> <<Self::Envelope as rstar::Envelope>::Point as rstar::Point>::Scalar {
        let col_dist = self.col - point[0];
        let row_dist = self.row - point[1];
        col_dist * col_dist + row_dist * row_dist
    }
}

impl Symbol {
    pub fn new(row: usize, col: usize, symbol: char) -> Symbol {
        let point = Point::new(row, col);
        Symbol { point, symbol }
    }
}

impl Number {
    pub fn new(row: usize, col: usize, value: usize) -> Number {
        let start_point = Point::new(row, col - value.checked_ilog10().unwrap() as usize);
        let end_point = Point::new(row, col);
        Number {
            start_point,
            end_point,
            value,
        }
    }
}

pub fn advent_of_code_3_1() {
    let (numbers, symbols) = find_numbers_and_simbols();
    let tree = RTree::bulk_load(symbols.iter().map(|s| s.point).collect());
    let sum = numbers
        .iter()
        .filter(|number| {
            has_neigbour(&tree, &number.start_point) || has_neigbour(&tree, &number.end_point)
        })
        .map(|number| number.value)
        .sum::<usize>();
    println!("Advent of code 3_1: {:?}", sum);
}

pub fn advent_of_code_3_2() {
    let (numbers, symbols) = find_numbers_and_simbols();
    let gears: Vec<Point> = symbols
        .iter()
        .filter(|s| s.symbol == '*')
        .map(|s| s.point)
        .collect();
    let tree = RTree::bulk_load(gears.clone());
    let mut gear_counts: HashMap<Point, Vec<usize>> =
        gears.into_iter().map(|s| (s, Vec::new())).collect();
    numbers
        .iter()
        .filter(|number| {
            has_neigbour(&tree, &number.start_point) || has_neigbour(&tree, &number.end_point)
        })
        .for_each(|number| {
            let neighbour = nearest_neighbor(&tree, number);
            gear_counts.get_mut(&neighbour).unwrap().push(number.value);
        });
    let sum: usize = gear_counts
        .into_iter()
        .map(|(_, v)| v)
        .filter(|v| v.len() == 2)
        .map(|v| v.iter().product::<usize>())
        .sum();
    println!("Advent of code 3_2: {:?}", sum);
}

fn has_neigbour(tree: &RTree<Point>, point: &Point) -> bool {
    tree.locate_within_distance(point.coordinate(), 2)
        .next()
        .is_some()
}

fn nearest_neighbor(tree: &RTree<Point>, number: &Number) -> Point {
    let start_neighbor = tree
        .locate_within_distance(number.start_point.coordinate(), 2)
        .next();
    match start_neighbor {
        Some(n) => *n,
        None => *tree
            .locate_within_distance(number.end_point.coordinate(), 2)
            .next()
            .unwrap(),
    }
}

fn find_numbers_and_simbols() -> (Vec<Number>, Vec<Symbol>) {
    let contents = fs::read_to_string("data/day_3.txt").unwrap();
    let mut numbers: Vec<Number> = Vec::new();
    let mut symbols: Vec<Symbol> = Vec::new();
    contents.lines().enumerate().for_each(|(row, line)| {
        let mut is_current_number = false;
        let mut current_value = 0;
        for (col, x) in line.chars().enumerate() {
            match x {
                '.' | ' ' => {
                    reset_values_and_append_number(
                        &mut is_current_number,
                        &mut current_value,
                        &mut numbers,
                        row,
                        col,
                    );
                }
                '0'..='9' => {
                    is_current_number = true;  
                    current_value = current_value * 10 + x.to_digit(10).unwrap() as usize
                }
                _ => {
                    reset_values_and_append_number(
                        &mut is_current_number,
                        &mut current_value,
                        &mut numbers,
                        row,
                        col,
                    );
                    symbols.push(Symbol::new(row, col, x))
                }
            }
        }
        reset_values_and_append_number(
            &mut is_current_number,
            &mut current_value,
            &mut numbers,
            row,
            line.len(),
        );
    });
    (numbers, symbols)
}

fn reset_values_and_append_number(
    is_current_number: &mut bool,
    current_value: &mut usize,
    numbers: &mut Vec<Number>,
    row: usize,
    col: usize,
) {
    if *is_current_number {
        numbers.push(Number::new(row, col - 1, *current_value));
        *is_current_number = false;
        *current_value = 0
    }
}
