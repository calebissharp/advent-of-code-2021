use std::fs;

const FILENAME: &str = "data/1/input.txt";

fn part1(measurements: &Vec<i32>) {
    let mut count = 0;

    for (i, measurement) in measurements.iter().enumerate() {
        if i == 0 {
            continue;
        };

        let last_measurement = &measurements[i - 1];

        if measurement > last_measurement {
            count += 1;
        }
    }

    println!("p1 count: {}", count)
}

fn part2(measurements: &Vec<i32>) {
    let mut iter = measurements.windows(3).peekable();
    let mut count = 0;

    while let Some(window) = iter.next() {
        if let Some(next_window) = iter.peek() {
            let sum = window.iter().sum::<i32>();
            let next_sum = next_window.iter().sum::<i32>();

            if next_sum > sum {
                count += 1;
            }
        }
    }

    println!("p2 count: {}", count);
}

fn main() {
    let contents = fs::read_to_string(FILENAME).expect("Couldn't read file");

    let measurements: Vec<i32> = contents
        .lines()
        .map(|line| line.parse::<i32>().unwrap())
        .collect();

    part1(&measurements);
    part2(&measurements);
}
