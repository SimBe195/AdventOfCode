use memoize::memoize;
use std::fs::read_to_string;

fn parse_input(filename: &str) -> Vec<u128> {
    read_to_string(filename)
        .expect("Failed to read file")
        .trim_end()
        .split_whitespace()
        .map(|x| x.parse().expect("Invalid number in file"))
        .collect()
}

fn rule_1(stone: u128) -> Option<Vec<u128>> {
    match stone {
        0 => Some(vec![1]),
        _ => None,
    }
}

fn rule_2(stone: u128) -> Option<Vec<u128>> {
    let digit_str = stone.to_string();
    if digit_str.len() % 2 == 1 {
        return None;
    }
    let (left_str, right_str) = digit_str.split_at(digit_str.len() / 2);
    let left_stone = left_str.parse().expect("Invalid digit string");
    let right_stone = right_str.parse().expect("Invalid digit string");
    Some(vec![left_stone, right_stone])
}

fn rule_3(stone: u128) -> Vec<u128> {
    vec![stone * 2024]
}

#[memoize]
fn ruleset(stone: u128) -> Vec<u128> {
    rule_1(stone)
        .or_else(|| rule_2(stone))
        .unwrap_or_else(|| rule_3(stone))
}

#[memoize]
fn num_stones_after_blinking_single(stone: u128, repetitions: u32) -> u128 {
    if repetitions == 0 {
        return 1;
    }

    ruleset(stone)
        .into_iter()
        .map(|x| num_stones_after_blinking_single(x, repetitions - 1))
        .sum()
}

fn num_stones_after_blinking(stones: Vec<u128>, repetitions: u32) -> u128 {
    stones
        .into_iter()
        .map(|stone| num_stones_after_blinking_single(stone, repetitions))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::{num_stones_after_blinking, parse_input};

    #[test]
    fn test_1() {
        assert_eq!(
            num_stones_after_blinking(parse_input("testinput.txt"), 6),
            22,
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            num_stones_after_blinking(parse_input("testinput.txt"), 25),
            55312,
        );
    }
}

fn main() {
    let stones = parse_input("input.txt");
    println!(
        "Challenge 1: {}",
        num_stones_after_blinking(stones.clone(), 25)
    );
    println!(
        "Challenge 2: {}",
        num_stones_after_blinking(stones.clone(), 75)
    );
}
