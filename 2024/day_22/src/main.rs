use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
};

fn parse_file(filename: &str) -> Vec<u32> {
    read_to_string(filename)
        .expect("Failed to read file.")
        .lines()
        .map(|x| x.trim().parse().expect("Invalid number in file."))
        .collect()
}

fn prune(secret_num: u32) -> u32 {
    (secret_num << 8) >> 8
}

fn transform_num_step_1(secret_num: u32) -> u32 {
    prune((secret_num << 6) ^ secret_num)
}

fn transform_num_step_2(secret_num: u32) -> u32 {
    prune((secret_num >> 5) ^ secret_num)
}

fn transform_num_step_3(secret_num: u32) -> u32 {
    prune((secret_num << 11) ^ secret_num)
}

fn transform_num(secret_num: u32) -> u32 {
    transform_num_step_3(transform_num_step_2(transform_num_step_1(secret_num)))
}

fn repeated_transform_num(secret_num: u32, num_repetitions: u32) -> u32 {
    let mut transformed = secret_num;
    for _ in 0..num_repetitions {
        transformed = transform_num(transformed)
    }

    transformed
}

fn transform_num_sequence(initial_num: u32, num_repetitions: u32) -> Vec<u32> {
    let mut sequence = vec![initial_num];
    let mut current_num = initial_num;
    for _ in 0..num_repetitions {
        let transform_num = transform_num(current_num);
        sequence.push(transform_num);
        current_num = transform_num;
    }

    sequence
}

fn sum_transformed_nums(secret_nums: Vec<u32>, num_repetitions: u32) -> u64 {
    secret_nums
        .iter()
        .map(|x| repeated_transform_num(*x, num_repetitions) as u64)
        .sum()
}

fn sequence_of_changes(num_sequence: &Vec<u32>) -> Vec<i32> {
    num_sequence
        .iter()
        .zip(num_sequence.iter().skip(1))
        .map(|(a, b)| (*b % 10) as i32 - (*a % 10) as i32)
        .collect()
}

fn get_all_possible_sell_values(
    initial_num: u32,
    num_repetitions: u32,
) -> HashMap<(i32, i32, i32, i32), u32> {
    let transformed_nums = transform_num_sequence(initial_num, num_repetitions);
    let changes = sequence_of_changes(&transformed_nums);
    let mut result = HashMap::new();
    for idx in 3..changes.len() {
        let len_4_change = (
            changes[idx - 3],
            changes[idx - 2],
            changes[idx - 1],
            changes[idx],
        );
        if !result.contains_key(&len_4_change) {
            result.insert(len_4_change, transformed_nums[idx + 1] % 10);
        }
    }

    result
}

fn get_all_len4_change_sequences() -> Vec<(i32, i32, i32, i32)> {
    let mut result = HashSet::new();

    for a in 0..10 {
        for b in 0..10 {
            for c in 0..10 {
                for d in 0..10 {
                    for e in 0..10 {
                        result.insert((b - a, c - b, d - c, e - d));
                    }
                }
            }
        }
    }

    result.into_iter().collect()
}

fn get_total_sell_value_for_changes(
    possible_sell_values: &Vec<HashMap<(i32, i32, i32, i32), u32>>,
    target_changes: (i32, i32, i32, i32),
) -> u32 {
    possible_sell_values
        .iter()
        .map(|x| x.get(&target_changes).unwrap_or(&0))
        .sum()
}

fn get_total_sell_value(initial_nums: Vec<u32>, num_repetitions: u32) -> u32 {
    let len_4_changes = get_all_len4_change_sequences();
    let possible_sell_values: Vec<HashMap<(i32, i32, i32, i32), u32>> = initial_nums
        .iter()
        .map(|x| get_all_possible_sell_values(*x, num_repetitions))
        .collect();

    len_4_changes
        .iter()
        .map(|x| get_total_sell_value_for_changes(&possible_sell_values, *x))
        .max()
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{
        get_all_possible_sell_values, get_total_sell_value, get_total_sell_value_for_changes,
        parse_file, repeated_transform_num, sequence_of_changes, sum_transformed_nums,
        transform_num, transform_num_sequence,
    };

    #[test]
    fn test_transform() {
        let result = transform_num(123);
        assert_eq!(result, 15887950);
    }

    #[test]
    fn test_transform_10() {
        let result = repeated_transform_num(123, 10);
        assert_eq!(result, 5908254);
    }

    #[test]
    fn test_change_sequence() {
        let nums = transform_num_sequence(123, 9);
        let changes = sequence_of_changes(&nums);
        assert_eq!(changes, [-3, 6, -1, -1, 0, 2, -2, 0, -2]);
    }

    #[test]
    fn test_testinput_2000() {
        let result = sum_transformed_nums(parse_file("testinput.txt"), 2000);
        assert_eq!(result, 37327623);
    }

    #[test]
    fn test_total_value_given_change() {
        let initial_nums = vec![1, 2, 3, 2024];

        let possible_sell_values: Vec<HashMap<(i32, i32, i32, i32), u32>> = initial_nums
            .iter()
            .map(|x| get_all_possible_sell_values(*x, 2000))
            .collect();

        assert_eq!(
            get_total_sell_value_for_changes(&possible_sell_values, (-2, 1, -1, 3)),
            23
        );
    }

    #[test]
    fn test_total_value() {
        assert_eq!(get_total_sell_value(vec![1, 2, 3, 2024], 2000), 23);
    }
}

fn main() {
    let initial_nums = parse_file("input.txt");
    println!(
        "Challenge 1: {}",
        sum_transformed_nums(initial_nums.clone(), 2000)
    );
    println!(
        "Challenge 2: {}",
        get_total_sell_value(initial_nums.clone(), 2000)
    );
}
