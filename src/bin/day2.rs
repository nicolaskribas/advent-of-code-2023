use std::cmp;

fn main() {
    let input = include_str!("../../inputs/day2.txt");
    println!("solution part one: {}", part1(input));
    println!("solution part two: {}", part2(input));
}

fn part1(input: &'static str) -> u32 {
    let is_in_limits = |(count, color)| match color {
        "red" => count <= 12,
        "green" => count <= 13,
        "blue" => count <= 14,
        _ => false,
    };

    let is_set_possible = |set: &'static str| {
        set.split(",")
            .map(str::trim)
            .map(|s| s.split_once(" ").unwrap())
            .map(|(count, color)| (count.parse::<u32>().unwrap(), color))
            .map(is_in_limits)
            .all(|b| b)
    };

    input
        .lines()
        .map(|l| {
            let (id, record) = l.split_once(":").unwrap();
            if record.split(";").map(is_set_possible).all(|b| b) {
                id.split_once(" ").unwrap().1.parse::<u32>().unwrap()
            } else {
                0
            }
        })
        .sum()
}

fn part2(input: &'static str) -> u32 {
    let max = |mut acc: (u32, u32, u32), (count, color): (u32, &'static str)| {
        match color {
            "red" => acc.0 = cmp::max(acc.0, count),
            "green" => acc.1 = cmp::max(acc.1, count),
            "blue" => acc.2 = cmp::max(acc.2, count),
            _ => (),
        };
        acc
    };

    let power = |line: &'static str| {
        let (r, g, b) = line
            .split_once(":")
            .unwrap()
            .1
            .split(";")
            .map(|a| a.split(","))
            .flatten()
            .map(str::trim)
            .map(|s| s.split_once(" ").unwrap())
            .map(|(count, color)| (count.parse::<u32>().unwrap(), color))
            .fold((0, 0, 0), max);
        r * g * b
    };

    input.lines().map(power).sum()
}

#[cfg(test)]
const TEST_INPUT: &'static str = "\
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

#[test]
fn test_part1() {
    assert_eq!(part1(TEST_INPUT), 8);
}

#[test]
fn test_part2() {
    assert_eq!(part2(TEST_INPUT), 2286);
}
