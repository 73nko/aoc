fn solve(input: &str, window_size: usize) -> usize {
    input
        .chars()
        .collect::<Vec<_>>()
        .windows(window_size)
        .enumerate()
        .find_map(|(i, w)| {
            if are_unique(w) {
                Some(i + window_size)
            } else {
                None
            }
        })
        .unwrap_or(input.len())
}

fn are_unique(chars: &[char]) -> bool {
    let mut chars = chars.to_vec();
    chars.sort_unstable();
    chars.windows(2).all(|w| w[0] != w[1])
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = r#"mjqjpqmgbljsphdztnvjfqwrcgsmlb"#;
    #[test]
    fn test_part1() {
        assert_eq!(solve(INPUT, 4), 7);
    }
    #[test]
    fn test_part2() {
        assert_eq!(solve(INPUT, 14), 19);
    }
}

fn main() {
    let input = include_str!("../input.txt");
    println!("PART 1: {:?}", solve(input, 4));
    println!("PART 2: {:?}", solve(input, 14));
}
