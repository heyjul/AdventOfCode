use std::collections::HashSet;

fn main() {
    let input = include_str!("../../day2/data/input.txt");
    let result = part1(input);
    println!("Part 1: {}", result);
    let result = part2(input);
    println!("Part 2: {}", result);
}

fn part1(input: &str) -> usize {
    let ranges = parse(input);
    let mut count = 0;
    for range in ranges {
        for val in range.0..=range.1 {
            let id = val.to_string();
            if id.len() % 2 != 0 {
                continue;
            }

            let middle = id.len() / 2;
            if id[0..middle] == id[middle..] {
                count += val;
            }
        }
    }

    count
}

fn part2(input: &str) -> usize {
    let ranges = parse(input);
    let mut count = 0;
    for range in ranges {
        for val in range.0..=range.1 {
            let id = val.to_string();
            let middle = id.len() / 2;
            for i in 1..=middle {
                if id.len() % i != 0 {
                    continue;
                }

                if id.as_bytes().chunks(i).collect::<HashSet<_>>().len() == 1 {
                    count += val;
                    break;
                }
            }
        }
    }

    count
}

fn parse(input: &str) -> Vec<(usize, usize)> {
    input.split(',').map(|range| {
        let (start, end) = range.trim().split_once('-').unwrap();
        (start.parse().unwrap(), end.parse().unwrap())
    }).collect()
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1() {
        let input = include_str!("../../day2/data/test.txt");
        let result = super::part1(input);
        assert_eq!(1227775554, result);
    }

    #[test]
    fn part2() {
        let input = include_str!("../../day2/data/test.txt");
        let result = super::part2(input);
        assert_eq!(4174379265, result);
    }
}
