use std::collections::HashMap;

fn dir_parc(commands: &[&str]) -> HashMap<String, u64> {
    let mut current: Vec<String> = vec![];
    let mut dirs: HashMap<String, u64> = HashMap::new();

    for &command in commands {
        if command.starts_with("$ ls") || command.starts_with("dir") {
            continue;
        }

        if command.starts_with("$ cd") {
            match &command[5..] {
                "/" => {
                    current = vec!["".to_string()];
                }
                ".." => {
                    current.pop();
                }
                other => {
                    current.push(other.to_string());
                }
            }
        } else {
            let (size, _file_name) = command.split_once(' ').unwrap();
            let size = size.parse::<u64>().unwrap();

            for i in 0..current.len() {
                let key = current[0..=i].join("/").to_string();
                dirs.entry(key).and_modify(|a| *a += size).or_insert(size);
            }
        }
    }

    dirs
}

fn part1(dirs: &HashMap<String, u64>) -> u64 {
    dirs.iter()
        .filter_map(|(_, &size)| if size <= 100000 { Some(size) } else { None })
        .sum()
}

fn part2(dirs: &HashMap<String, u64>) -> u64 {
    let tt_space = 70000000;
    let free_space = tt_space - dirs[""];
    let need_to_free = 30000000 - free_space;

    let mut sizes = dirs
        .iter()
        .filter_map(|(_, &size)| {
            if size >= need_to_free {
                Some(size)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    sizes.sort();
    sizes[0]
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     const INPUT: &str = r#""#;
//     #[test]
//     fn test_part1() {
//         assert_eq!(part1(INPUT), 0);
//     }
//     #[test]
//     fn test_part2() {
//         assert_eq!(part2(INPUT), 0);
//     }
// }

fn main() {
    let input = include_str!("../input.txt");
    let dirs = dir_parc(&input.lines().collect::<Vec<_>>());

    println!("PART 1: {:?}", part1(&dirs));
    println!("PART 2: {:?}", part2(&dirs));
}
