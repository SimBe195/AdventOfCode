use std::fs::read_to_string;

fn parse_file(filename: &str) -> (Vec<[i32; 5]>, Vec<[i32; 5]>) {
    let mut keys = Vec::new();
    let mut locks = Vec::new();

    let content = read_to_string(filename).expect("Failed to open file");

    let mut current_data: [i32; 5] = [-1, -1, -1, -1, -1];
    let mut in_key = false;
    let mut in_lock = false;

    for line in content.lines() {
        if !in_key && !in_lock {
            current_data = [-1, -1, -1, -1, -1];
        }
        if !in_key && !in_lock && line.trim().chars().all(|c| c == '#') {
            in_lock = true;
        }
        if !in_key && !in_lock && line.trim().chars().all(|c| c == '.') {
            in_key = true;
        }

        if line.trim().is_empty() {
            if in_key {
                keys.push(current_data.clone());
                in_key = false;
            }
            if in_lock {
                locks.push(current_data.clone());
                in_lock = false;
            }
            continue;
        }

        let mut next_row: [i32; 5] = [0, 0, 0, 0, 0];

        for i in 0..5 {
            if line
                .chars()
                .nth(i)
                .expect("Line with too few characters in input")
                == '#'
            {
                next_row[i] = 1;
            }
        }

        for i in 0..5 {
            current_data[i] += next_row[i];
        }
    }

    (keys, locks)
}

fn check_fit(lock: &[i32; 5], key: &[i32; 5]) -> bool {
    lock.iter().zip(key.iter()).all(|(x, y)| x + y < 6)
}

fn count_fitting_pairs(locks: &Vec<[i32; 5]>, keys: &Vec<[i32; 5]>) -> u32 {
    locks
        .iter()
        .map(|l| keys.iter().map(|k| check_fit(l, k) as u32).sum::<u32>())
        .sum()
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn test_parsing() {
        let (keys, locks) = parse_file("testinput.txt");
        assert!(keys.contains(&[5, 0, 2, 1, 3]));
        assert!(keys.contains(&[4, 3, 4, 0, 2]));
        assert!(keys.contains(&[3, 0, 2, 0, 1]));
        assert!(locks.contains(&[0, 5, 3, 4, 3]));
        assert!(locks.contains(&[1, 2, 0, 5, 3]));
    }

    #[test]
    fn test_check_fit() {
        assert!(!check_fit(&[0, 5, 3, 4, 3], &[5, 0, 2, 1, 3]));
        assert!(!check_fit(&[0, 5, 3, 4, 3], &[4, 3, 4, 0, 2]));
        assert!(check_fit(&[0, 5, 3, 4, 3], &[3, 0, 2, 0, 1]));
        assert!(!check_fit(&[1, 2, 0, 5, 3], &[5, 0, 2, 1, 3]));
        assert!(check_fit(&[1, 2, 0, 5, 3], &[4, 3, 4, 0, 2]));
        assert!(check_fit(&[1, 2, 0, 5, 3], &[3, 0, 2, 0, 1]));
    }

    #[test]
    fn test_fit_count() {
        let (keys, locks) = parse_file("testinput.txt");
        assert_eq!(count_fitting_pairs(&locks, &keys), 3);
    }
}

fn main() {
    let (keys, locks) = parse_file("input.txt");
    println!("Challenge 1: {}", count_fitting_pairs(&locks, &keys));
}
