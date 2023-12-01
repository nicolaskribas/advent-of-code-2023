use std::io::{self, BufRead};

fn main() {
    let sum: u32 = io::stdin().lock().lines()
        .map(|l| l.unwrap().chars().filter(|c| c.is_numeric()).collect::<Vec<_>>().into_iter())
        .map(|mut n| {
                let first = n.next().unwrap();
                (first, n.last().unwrap_or(first))
        })
        .map(|(first, last)| (first.to_digit(10).unwrap(), last.to_digit(10).unwrap()))
        .map(|(first, last)| first * 10  + last)
        .sum();

    println!("sum {sum}");
}
