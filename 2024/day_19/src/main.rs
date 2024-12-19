use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

fn parse_file(filename: &str) -> io::Result<(Vec<String>, Vec<String>)> {
    // Open the file
    let file = File::open(filename)?;
    let mut lines = io::BufReader::new(file).lines();

    // Parse the first line: comma-separated list of strings
    let headers = if let Some(Ok(line)) = lines.next() {
        line.split(',').map(|s| s.trim().to_string()).collect()
    } else {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "File missing headers",
        ));
    };

    // Skip the second line (empty line)
    if lines.next().is_none() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "File missing expected format",
        ));
    }

    // Collect remaining lines as strings
    let values: Vec<String> = lines
        .filter_map(|line| line.ok())
        .map(|s| s.trim().to_string())
        .collect();

    Ok((headers, values))
}

fn has_towel_arrangement(design: &str, patterns: &Vec<String>) -> bool {
    if design.is_empty() {
        return true;
    }

    patterns
        .iter()
        .filter(|pattern| design.starts_with(*pattern))
        .any(|pattern| has_towel_arrangement(&design[pattern.len()..], &patterns))
}

fn num_possible_towel_arrangements(
    design: &str,
    patterns: &Vec<String>,
    memo: &mut HashMap<String, u64>,
) -> u64 {
    if design.is_empty() {
        return 1u64;
    }

    if let Some(&result) = memo.get(design) {
        return result;
    }

    let result = patterns
        .iter()
        .filter(|pattern| design.starts_with(*pattern))
        .map(|pattern| num_possible_towel_arrangements(&design[pattern.len()..], &patterns, memo))
        .sum();

    memo.insert(design.to_string(), result);

    result
}

fn challenge_1(filename: &str) -> u64 {
    let (patterns, designs) = parse_file(filename).unwrap();
    designs
        .iter()
        .map(|design| has_towel_arrangement(&design, &patterns) as u64)
        .sum()
}

fn challenge_2(filename: &str) -> u64 {
    let (patterns, designs) = parse_file(filename).unwrap();
    let mut memo = HashMap::new();
    designs
        .iter()
        .map(|design| num_possible_towel_arrangements(&design, &patterns, &mut memo))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::{challenge_1, challenge_2};

    #[test]
    fn test_challenge_1() {
        assert_eq!(challenge_1("testinput.txt"), 6);
    }

    #[test]
    fn test_challenge_2() {
        assert_eq!(challenge_2("testinput.txt"), 16);
    }
}

fn main() {
    println!("Challenge 1: {}", challenge_1("input.txt"));
    println!("Challenge 2: {}", challenge_2("input.txt"));
}
