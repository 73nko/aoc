
use anyhow::Result;

fn part1(input: &str) -> Result<usize> {
    Ok(0)
}
fn part2(input: &str) -> Result<usize> {
    Ok(0)
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
    let input = include_str!("./inputs/2021_23_input.txt").trim_end();
    println!("{:?}", part1(input)?);
    // println!("{:?}", part2(input)?);
    Ok(())
}
    