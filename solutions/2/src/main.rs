use std::fs;

const FILENAME: &str = "data/2/input.txt";

fn part1(lines: &Vec<&str>) {
    let mut depth = 0;
    let mut hor = 0;
    let mut aim = 0;

    for line in lines.iter() {
        let split = line.split(" ").collect::<Vec<&str>>();

        let instruction = split[0];
        let val = split[1].parse::<i32>().expect("Could not parse number");

        let (x, y) = match instruction {
            "forward" => (val, 0),
            "up" => (0, -val),
            "down" => (0, val),
            _ => (0, 0),
        };

        hor += x;
        depth += y;
    }

    println!(
        "part1 result: {} (depth: {}, horizontal: {})",
        depth * hor,
        depth,
        hor
    );
}

fn part2(lines: &Vec<&str>) {
    let mut depth = 0;
    let mut hor = 0;
    let mut aim = 0;

    for line in lines.iter() {
        let split = line.split(" ").collect::<Vec<&str>>();

        let instruction = split[0];
        let val = split[1].parse::<i32>().expect("Could not parse number");

        let (x, y) = match instruction {
            "forward" => (val, 0),
            "up" => (0, -val),
            "down" => (0, val),
            _ => (0, 0),
        };

        aim += y;
        hor += x;
        depth += aim * x;
    }

    println!(
        "part2 result: {} (depth: {}, horizontal: {})",
        depth * hor,
        depth,
        hor
    );
}

fn main() {
    let contents = fs::read_to_string(FILENAME).expect("Couldn't read file");

    let instructions: Vec<&str> = contents.lines().collect();

    part1(&instructions);
    part2(&instructions);
}
