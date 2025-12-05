fn main() {
    let input = include_str!("../../day5/data/input.txt");
    let result = part1(input);
    println!("Part 1: {}", result);
    let result = part2(input);
    println!("Part 2: {}", result);
}

fn part1(values: &str) -> usize {
    let (ranges, values) = parse(values);
    let mut count = 0;

    for val in values {
        for range in ranges.iter() {
            if val >= range.0 && val <= range.1 {
                count += 1;
                break;
            }
        }
    }

    count
}

fn part2(input: &str) -> usize {
    let (mut ranges, _) = parse(input);

    let mut new_ranges = ranges.clone();
    for (i, range) in ranges.iter_mut().enumerate() {
        for (j, range2) in new_ranges.clone().iter().enumerate() {
            if i == j {
                continue;
            }

            if range.0 >= range2.0 && range.0 <= range2.1 {
                if range.1 <= range2.1 {
                    range.0 = 1;
                    range.1 = 0;
                } else {
                    range.0 = range2.1 + 1;
                }
            }

            new_ranges[i] = range.clone();
        }

    }

    ranges.into_iter().map(|range| (range.1 - range.0 + 1) as usize).sum()
}

fn parse(input: &str) -> (Vec<(isize, isize)>, Vec<isize>) {
    let ranges: Vec<_> = input
        .lines()
        .map_while(|line| {
            line.split_once('-')
                .and_then(|(start, end)| Some((start.parse().unwrap(), end.parse().unwrap())))
        })
        .collect();

    let values = input
        .lines()
        .skip(ranges.len() + 1)
        .map(|line| line.parse().unwrap())
        .collect();
    (ranges, values)
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1() {
        let input = include_str!("../../day5/data/test.txt");
        let result = super::part1(input);
        assert_eq!(3, result);
    }

    #[test]
    fn part2() {
        let input = include_str!("../../day5/data/test.txt");
        let result = super::part2(input);
        assert_eq!(14, result);
    }

    #[test]
    fn part2_2() {
        let input = r#"3-5
10-14
16-20
12-18
1-6"#;
        let result = super::part2(input);
        assert_eq!(17, result);
    }

    #[test]
    fn part2_3() {
        let input = r#"3-5
10-14
16-20
12-18
1-6
5-7"#;
        let result = super::part2(input);
        assert_eq!(18, result);
    }

    #[test]
    fn part2_4() {
        let input = r#"3-5
10-14
16-20
12-18
1-6
1-4
5-7"#;
        let result = super::part2(input);
        assert_eq!(18, result);
    }}
