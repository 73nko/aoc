use anyhow::Result;
use std::str::FromStr;

// Define a struct to represent a range of integers
#[derive(Debug, Clone, Copy)]
struct Range {
    start: i32,
    end: i32,
}

// Implement FromStr trait for Range
impl FromStr for Range {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Split the input string on the '-' character
        let parts = s.split('-').collect::<Vec<&str>>();
        // Parse the start and end of the range from the parts
        let start = parts[0].parse::<i32>()?;
        let end = parts[1].parse::<i32>()?;
        // Return the parsed range
        Ok(Range { start, end })
    }
}

// Implement the contains method for Range
impl Range {
    fn contains(&self, other: Range) -> bool {
        // Check if the other range is contained within this range
        self.start <= other.start && other.end <= self.end
    }
}

// Parse a pair of ranges from a string
fn parse_pair(s: &str) -> Result<(Range, Range), std::num::ParseIntError> {
    // Split the input string on the ',' character
    let parts = s.split(',').collect::<Vec<&str>>();
    // Parse the first and second range from the parts
    let range1 = parts[0].parse::<Range>()?;
    let range2 = parts[1].parse::<Range>()?;
    // Return the parsed ranges
    Ok((range1, range2))
}

// Parse a list of input strings and return a list of parsed ranges
fn parse_pairs(lines: Vec<String>) -> Result<Vec<(Range, Range)>, std::num::ParseIntError> {
    let mut pairs = Vec::new();
    for line in lines {
        // Parse the pair of ranges from the line
        let pair = parse_pair(&line)?;
        // Add the parsed pair to the list of pairs
        pairs.push(pair);
    }
    // Return the list of pairs
    Ok(pairs)
}

// Count the number of pairs where one range fully contains the other
fn count_fully_contained_pairs(pairs: Vec<(Range, Range)>) -> i32 {
    let mut count = 0;
    for pair in pairs {
        // Check if one range fully contains the other
        if pair.0.contains(pair.1) || pair.1.contains(pair.0) {
            // Increment the count if a fully contained pair is found
            count += 1;
        }
    }
    // Return the final count
    count
}

fn count_overlapping_pairs(pairs: Vec<(Range, Range)>) -> i32 {
    let mut count = 0;
    for pair in pairs {
        // Check if the ranges intersect
        let intersection = Range {
            start: pair.0.start.max(pair.1.start),
            end: pair.0.end.min(pair.1.end),
        };
        if intersection.start <= intersection.end {
            // Increment the count if the ranges intersect
            count += 1;
        }
    }
    // Return the final count
    count
}

// Solve the problem
fn solve(lines: Vec<String>) -> i32 {
    // Parse the list of input strings
    let pairs = parse_pairs(lines).unwrap();
    // Count the number of pairs where one range fully contains the other
    count_fully_contained_pairs(pairs)
}

fn solve2(lines: Vec<String>) -> i32 {
    // Parse the list of input strings
    let pairs = parse_pairs(lines).unwrap();
    // Count the number of pairs where one range fully contains the other
    count_overlapping_pairs(pairs)
}

fn part1(input: &str) -> Result<usize> {
    let lines = input.lines().map(|l| l.to_string()).collect();
    Ok(solve(lines) as usize)
}
fn part2(input: &str) -> Result<usize> {
    let lines = input.lines().map(|l| l.to_string()).collect();
    Ok(solve2(lines) as usize)
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &'static str = "";
    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT).unwrap(), 0);
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT).unwrap(), 0);
    }
}

fn main() -> Result<()> {
    let input = include_str!("../input.txt");
    println!("PART 1: {:?}", part1(input)?);
    println!("PART 2: {:?}", part2(input)?);
    Ok(())
}
