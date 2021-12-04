use std::fs;

const FILENAME: &str = "data/3/data.txt";
const TEST_FILENAME: &str = "data/3/test.txt";

fn part1(report: &Vec<u32>, cols: usize) {
    let mut gamma = 0;
    for col in 0..cols {
        let mask = 1 << col;
        let nums: Vec<u32> = report
            .clone()
            .into_iter()
            .filter(|row| ((row & mask) >> col) == 1)
            .collect();

        // More than half the bits are 1s
        if nums.len() > (report.len() / 2) {
            // Set result bit
            gamma |= mask;
        }
    }

    // Initialize with a value of 1 for each bit up to `col`
    let mut epsilon: u32 = 2u32.pow(cols as u32) - 1;
    // "Invert" gamma (up to length of `col` in effect)
    epsilon = gamma ^ epsilon;

    println!("part1: {}", epsilon * gamma);
}

fn calculate_oxygen(mut report: Vec<u32>, cols: usize) -> u32 {
    for col in 0..cols {
        let offset = cols - col - 1;
        let mask = 1 << offset;
        let one_count: u32 = report
            .clone()
            .into_iter()
            .map(|row| (row & mask) >> offset)
            .sum();

        let trump_bit = if one_count >= report.len() as u32 - one_count {
            1
        } else {
            0
        };

        report = report
            .into_iter()
            .filter(|row| ((row & mask) >> offset) == trump_bit)
            .collect();

        if report.len() == 1 {
            return report[0];
        }
    }

    assert_eq!(report.len(), 1);

    return report[0];
}

fn calculate_scrubber(mut report: Vec<u32>, cols: usize) -> u32 {
    for col in 0..cols {
        let offset = cols - col - 1;
        let mask = 1 << offset;
        let one_count: u32 = report
            .clone()
            .into_iter()
            .map(|row| ((row & mask) >> offset))
            .sum();

        let trump_bit = if one_count >= report.len() as u32 - one_count {
            0
        } else {
            1
        };

        report = report
            .into_iter()
            .filter(|row| ((row & mask) >> offset) == trump_bit)
            .collect();

        if report.len() == 1 {
            return report[0];
        }
    }

    assert_eq!(report.len(), 1);

    return report[0];
}

fn part2(report: &Vec<u32>, cols: usize) -> u32 {
    let oxygen = calculate_oxygen(report.clone(), cols);
    let scrubber = calculate_scrubber(report.clone(), cols);

    println!("part2: {}", oxygen * scrubber);

    return oxygen * scrubber;
}

fn main() {
    let contents = fs::read_to_string(FILENAME).expect("Couldn't read file");

    let lines: Vec<&str> = contents.lines().collect();

    let cols = lines[0].len();
    let report: Vec<u32> = lines
        .iter()
        .map(|line| u32::from_str_radix(line, 2).unwrap())
        .collect();

    part1(&report, cols);

    // assert_eq!(
    //     part2(
    //         &fs::read_to_string(TEST_FILENAME)
    //             .expect("Couldn't read test file")
    //             .lines()
    //             .collect::<Vec<&str>>()
    //             .iter()
    //             .map(|line| u32::from_str_radix(line, 2).unwrap())
    //             .collect(),
    //         5
    //     ),
    //     230
    // );

    part2(&report, cols);
}
