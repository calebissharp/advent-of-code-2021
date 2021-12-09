#![feature(drain_filter)]

use ndarray::Array2;
use std::fs;

const FILENAME: &str = "data/4/input.txt";
const BOARD_SIZE: usize = 5;

struct Board<'a> {
    numbers: &'a Array2<u32>,
    markers: Array2<u32>,
}

impl<'a> Board<'a> {
    fn has_won(&self) -> bool {
        // Check rows
        for row in self.markers.rows() {
            if row.iter().sum::<u32>() == BOARD_SIZE.try_into().unwrap() {
                return true;
            }
        }

        for column in self.markers.columns() {
            if column.iter().sum::<u32>() == BOARD_SIZE.try_into().unwrap() {
                return true;
            }
        }

        false
    }

    fn score(&self, last_instruction: u32) -> u32 {
        let unmarked_numbers = self.numbers - (self.markers.clone() * self.numbers);

        unmarked_numbers.iter().sum::<u32>() * last_instruction
    }

    fn mark_number(&mut self, number: u32) {
        if let Some((index, _)) = self.numbers.indexed_iter().find(|(_i, n)| **n == number) {
            self.markers[index] = 1;
        }
    }
}

fn part1(instructions: &Vec<u32>, board_numbers: &Vec<Array2<u32>>) {
    let mut boards: Vec<Board> = board_numbers
        .iter()
        .map(|numbers| Board {
            numbers,
            markers: Array2::zeros(numbers.raw_dim()),
        })
        .collect();

    for instruction in instructions.iter() {
        for (board_i, board) in boards.iter_mut().enumerate() {
            board.mark_number(*instruction);

            if board.has_won() {
                println!(
                    "p1/winner: {}, score: {}",
                    board_i,
                    board.score(*instruction)
                );
                return;
            }
        }
    }
}

fn part2(instructions: &Vec<u32>, board_numbers: &[Array2<u32>]) {
    let mut boards: Vec<Board> = board_numbers
        .iter()
        .map(|numbers| Board {
            numbers,
            markers: Array2::zeros(numbers.raw_dim()),
        })
        .collect();

    for instruction in instructions.iter() {
        boards.retain(|board| !board.has_won());

        for board in boards.iter_mut() {
            board.mark_number(*instruction);
        }

        if boards.len() == 1 && boards[0].has_won() {
            println!("p2/winner: {}", boards[0].score(*instruction));
        }
    }
}

fn main() {
    let contents = fs::read_to_string(FILENAME).expect("Couldn't read file");

    let mut lines = contents.lines().peekable();

    let instructions: Vec<u32> = lines
        .next()
        .unwrap()
        .split(",")
        .map(|num| num.parse::<u32>().unwrap())
        .collect();

    // Skip the first empty line
    lines.next();

    let mut boards: Vec<Array2<u32>> = vec![];
    while lines.peek().is_some() {
        let rows = lines
            .by_ref()
            .take_while(|x| x.trim() != "")
            .map(|row| {
                row.split_whitespace()
                    .map(|num| num.parse::<u32>().unwrap())
                    .collect::<Vec<u32>>()
            })
            .flatten()
            .collect();

        boards.push(Array2::from_shape_vec((BOARD_SIZE, BOARD_SIZE), rows).unwrap());
    }

    part1(&instructions, &boards);
    part2(&instructions, &boards);
}
