use anyhow::Result;
use std::collections::HashSet;

fn part1(input: &str) -> Result<usize> {
    Ok(input
        .lines()
        .map(|l| l.trim())
        .map(get_common_in_line)
        .sum())
}
fn part2(input: &str) -> Result<usize> {
    Ok(input
        .lines()
        .map(|l| l.trim())
        .collect::<Vec<_>>()
        .chunks(3)
        .map(get_common_in_chunk)
        .sum())
}

fn get_common_in_line(line: &str) -> usize {
    let parts = line.trim().split_at(line.len() / 2);
    get_char_value(find_common_item(vec![parts.0, parts.1]))
}

fn get_common_in_chunk(chunk: &[&str]) -> usize {
    let parts = chunk.iter().map(|l| l.trim()).collect();
    get_char_value(find_common_item(parts))
}

fn find_common_item(groups: Vec<&str>) -> char {
    let mut common_items = groups[0].chars().collect::<HashSet<_>>();

    groups.iter().for_each(|group| {
        common_items = common_items
            .intersection(&group.chars().collect::<HashSet<_>>())
            .copied()
            .collect()
    });

    *common_items.iter().next().unwrap()
}

fn get_char_value(ch: char) -> usize {
    if ch.is_lowercase() {
        ch as usize - ('a' as usize) + 1
    } else {
        ch as usize - ('A' as usize) + 27
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = r#"vJrwpWtwJgWrhcsFMMfFFhFp
    jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
    PmmdzqPrVvPwwTWBwg
    wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
    ttgJtRGJQctTZtZT
    CrZsJsPPZsGzwwsLwLmpwMDw"#;
    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT).unwrap(), 157);
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT).unwrap(), 70);
    }
}

fn main() -> Result<()> {
    let input = include_str!("../input.txt");
    println!("PART 1: {:?}", part1(input)?);
    println!("PART 2: {:?}", part2(input)?);
    Ok(())
}
