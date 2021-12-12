use ndarray::{Array2, Array3, Axis};
use regex;
use std::fs;

const FILENAME: &str = "data/5/data.txt";
const TEST_FILENAME: &str = "data/5/test.txt";

fn part1(lines: &Array3<u32>, use_diagonal: bool) -> u32 {
    let width: usize = lines
        .rows()
        .into_iter()
        .map(|c| c[0])
        .max()
        .unwrap()
        .try_into()
        .unwrap();
    let height: usize = lines
        .rows()
        .into_iter()
        .map(|c| c[1])
        .max()
        .unwrap()
        .try_into()
        .unwrap();

    let mut sea_floor = Array2::<u32>::zeros((width + 1, height + 1));

    for line in lines.axis_iter(Axis(0)) {
        let x1: i32 = line[[0, 0]].try_into().unwrap();
        let y1: i32 = line[[0, 1]].try_into().unwrap();
        let x2: i32 = line[[1, 0]].try_into().unwrap();
        let y2: i32 = line[[1, 1]].try_into().unwrap();

        if !use_diagonal && x1 != x2 && y1 != y2 {
            continue;
        }
        // Vertical line
        if x1 == x2 {
            let y_min = y1.min(y2);
            let y_max = y1.max(y2);

            for y in y_min..(y_max + 1) {
                sea_floor[[x1.try_into().unwrap(), y.try_into().unwrap()]] += 1;
            }
        } else {
            let m = (y2 - y1) / (x2 - x1);
            let b = y1 - m * x1;

            let x_min = x1.min(x2);
            let x_max = x1.max(x2);
            for x in x_min..(x_max + 1) {
                let y = m * x + b;
                sea_floor[[x.try_into().unwrap(), y.try_into().unwrap()]] += 1;
            }
        }
    }

    sea_floor
        .iter()
        .map(|c| if *c > 1 { 1 } else { 0 })
        .sum::<u32>()
}

fn process_file(filename: &str) -> Array3<u32> {
    let contents = fs::read_to_string(filename).expect("Couldn't read file");

    let mut num_rows = 0;
    let mut data: Vec<u32> = Vec::new();

    let re = regex::Regex::new(r",|->").unwrap();
    for line in contents.lines() {
        let row = re
            .split(line)
            .map(|s| s.trim().parse::<u32>().unwrap())
            .collect::<Vec<_>>();
        num_rows += 1;
        data.extend_from_slice(&row);
    }

    Array3::from_shape_vec((num_rows, 2, 2), data).unwrap()
}

fn main() {
    let data = process_file(FILENAME);
    let test_data = process_file(TEST_FILENAME);

    assert_eq!(part1(&test_data, false), 5);
    assert_eq!(part1(&test_data, true), 12);

    let p1_score = part1(&data, false);
    println!("p1 {}", p1_score);

    let p2_score = part1(&data, true);
    println!("p2 {}", p2_score);
}
