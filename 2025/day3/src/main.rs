fn main() {
    let input = include_str!("../../day3/data/input.txt");
    let result = part1(input);
    println!("Part 1: {}", result);
    let result = part2(input);
    println!("Part 2: {}", result);
}

fn part1(input: &str) -> usize {
    input
        .trim()
        .lines()
        .map(|line| {
            let mut ten = 0;
            let mut unit = 0;

            let bytes = line.as_bytes();
            let len = bytes.len();

            for byte in bytes.iter().take(len - 1) {
                let value = byte - b'0';
                if value > ten {
                    ten = value;
                    unit = 0;
                    continue;
                }

                if value > unit {
                    unit = value;
                }
            }

            let value = bytes.last().unwrap() - b'0';
            if value > unit {
                unit = value;
            }

            (ten * 10 + unit) as usize
        })
        .sum()
}

// impl of part 2 could be used for part 1 with MAX_LEN = 2
fn part2(input: &str) -> usize {
    const MAX_LEN: usize = 12;

    input
        .trim()
        .lines()
        .map(|line| {
            let mut values = vec![0; MAX_LEN];

            let bytes = line.as_bytes();
            let len = bytes.len();

            for (pos, byte) in bytes.iter().enumerate() {
                let value = byte - b'0';
                let min_pos = (pos as isize - (len as isize - MAX_LEN as isize)).max(0) as usize;
                for i in min_pos..MAX_LEN {
                    if value > values[i] {
                        values[i] = value;
                        for j in (i + 1)..MAX_LEN {
                            values[j] = 0;
                        }
                        break;
                    }
                }
            }

            values.into_iter().fold(0, |acc, v| acc * 10 + v as usize)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1() {
        let input = include_str!("../../day3/data/test.txt");
        let result = super::part1(input);
        assert_eq!(357, result);
    }

    #[test]
    fn part2() {
        let input = include_str!("../../day3/data/test.txt");
        let result = super::part2(input);
        assert_eq!(3121910778619, result);
    }
}
