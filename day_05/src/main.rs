use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;

type SuccessorMap = HashMap<u32, HashSet<u32>>;
type Ordering = Vec<u32>;

fn get_successors(filename: &str) -> SuccessorMap {
    let content = read_to_string(filename).expect("Failed to read file");
    content
        .lines()
        .filter_map(|line| line.split_once('|'))
        .map(|(left, right)| {
            let left_num: u32 = left.parse().expect("Invalid number in predecessors part");
            let right_num: u32 = right.parse().expect("Invalid number in successor part");
            (left_num, right_num)
        })
        .fold(HashMap::new(), |mut map, (left, right)| {
            map.entry(left).or_default().insert(right);
            map
        })
}

fn get_orderings(filename: &str) -> Vec<Ordering> {
    let content = read_to_string(filename).expect("Failed to read file");

    content
        .lines()
        .filter(|line| !line.is_empty() && !line.contains('|'))
        .map(|line| {
            line.split(',')
                .map(|num| num.parse().expect("Invalid number in ordering"))
                .collect()
        })
        .collect()
}

fn violation(first_num: &u32, second_num: &u32, successors: &SuccessorMap) -> bool {
    successors
        .get(second_num)
        .map_or(false, |second_successor_set| {
            second_successor_set.contains(first_num)
        })
}

fn correct_order(ordering: &Ordering, successors: &SuccessorMap) -> bool {
    ordering.iter().enumerate().all(|(i, first)| {
        ordering[i + 1..]
            .iter()
            .all(|second| !violation(first, second, successors))
    })
}

fn reorder(ordering: &mut Ordering, successors: &SuccessorMap) {
    ordering.sort_by(|first, second| {
        if Some(true)
            == successors
                .get(first)
                .map(|first_succ| first_succ.contains(second))
        {
            return std::cmp::Ordering::Less;
        }
        if Some(true)
            == successors
                .get(second)
                .map(|second_succ| second_succ.contains(first))
        {
            return std::cmp::Ordering::Greater;
        }
        std::cmp::Ordering::Equal
    });
}

fn challenge_1(filename: &str) -> u32 {
    let successors = get_successors(filename);
    let orderings = get_orderings(filename);

    orderings
        .into_iter()
        .filter(|ordering| correct_order(ordering, &successors))
        .map(|ordering| ordering[ordering.len() / 2])
        .sum()
}

fn challenge_2(filename: &str) -> u32 {
    let successors = get_successors(filename);
    let orderings = get_orderings(filename);

    orderings
        .into_iter()
        .filter(|ordering| !correct_order(ordering, &successors))
        .map(|mut ordering| {
            reorder(&mut ordering, &successors);
            ordering
        })
        .map(|ordering| ordering[ordering.len() / 2])
        .sum()
}

#[cfg(test)]
mod tests {
    use super::{challenge_1, challenge_2};

    #[test]
    fn test_1() {
        let result = challenge_1("testinput.txt");
        assert_eq!(result, 143);
    }

    #[test]
    fn test_2() {
        let result = challenge_2("testinput.txt");
        assert_eq!(result, 123);
    }
}

fn main() {
    println!("Challenge 1: {}", challenge_1("input.txt"));
    println!("Challenge 2: {}", challenge_2("input.txt"));
}
