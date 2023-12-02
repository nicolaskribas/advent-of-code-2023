use aho_corasick::AhoCorasick;

fn main() {
    let input = include_str!("../../inputs/day1.txt");
    println!("solution part one: {}", part1(input));
    println!("solution part two: {}", part2(input));
}

fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|l| {
            l.chars()
                .filter(|c| c.is_numeric())
                .collect::<Vec<_>>()
                .into_iter()
        })
        .map(|mut n| {
            let first = n.next().unwrap();
            (first, n.last().unwrap_or(first))
        })
        .map(|(first, last)| (first.to_digit(10).unwrap(), last.to_digit(10).unwrap()))
        .map(|(first, last)| first * 10 + last)
        .sum()
}

fn part2(input: &str) -> u32 {
    let patterns = &[
        "1", "2", "3", "4", "5", "6", "7", "8", "9", "one", "two", "three", "four", "five", "six",
        "seven", "eight", "nine",
    ];
    let ac = AhoCorasick::new(patterns).unwrap();

    let parse = |s| match s {
        "1" | "one" => 1,
        "2" | "two" => 2,
        "3" | "three" => 3,
        "4" | "four" => 4,
        "5" | "five" => 5,
        "6" | "six" => 6,
        "7" | "seven" => 7,
        "8" | "eight" => 8,
        "9" | "nine" => 9,
        _ => 0,
    };

    input
        .lines()
        .map(|l| ac.find_overlapping_iter(l))
        .map(|mut n| {
            let first = n.next().unwrap();
            (
                patterns[first.pattern()],
                patterns[n.last().unwrap_or(first).pattern()],
            )
        })
        .map(|(first, last)| (parse(first), parse(last)))
        .map(|(first, last)| first * 10 + last)
        .sum()
}

#[test]
fn test_part1() {
    let input = "\
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

    assert_eq!(part1(input), 142);
}

#[test]
fn test_part2() {
    let input = "\
two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

    assert_eq!(part2(input), 281);
}
