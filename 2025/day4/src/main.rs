fn main() {
    let input = include_str!("../../day4/data/input.txt");
    let mut input = parse(input);
    let result = part1(&input).len();
    println!("Part 1: {}", result);
    let result = part2(&mut input);
    println!("Part 2: {}", result);
}

fn part1(values: &Vec<Vec<u8>>) -> Vec<(usize, usize)> {
    let i_len = values.len();
    let j_len = values[0].len();
    let mut forkliftable = vec![];

    for i in 0..i_len {
        for j in 0..j_len {
            if values[i][j] != b'@' {
                continue;
            }

            let mut count = 0;

            for ii in 0..3 {
                // Up
                if let Some(i) = i.checked_sub(1)
                    && let Some(values) = values.get(i)
                {
                    if let Ok(j) = usize::try_from(j as isize - 1 + ii as isize)
                        && let Some(x) = values.get(j)
                    {
                        if *x == b'@' {
                            count += 1;
                        }
                    }
                }

                // Down
                if let Some(x) = values.get(i + 1) {
                    if let Ok(j) = usize::try_from(j as isize - 1 + ii as isize)
                        && let Some(x) = x.get(j)
                    {
                        if *x == b'@' {
                            count += 1;
                        }
                    }
                }
            }

            // Left
            if let Some(j) = j.checked_sub(1)
                && let Some(x) = values[i].get(j)
            {
                if *x == b'@' {
                    count += 1;
                }
            }

            // Right
            if let Some(x) = values[i].get(j + 1) {
                if *x == b'@' {
                    count += 1;
                }
            }

            if count < 4 {
                forkliftable.push((i, j));
            }
        }
    }

    forkliftable
}

fn part2(input: &mut Vec<Vec<u8>>) -> usize {
    let mut count = 0;

    loop {
        let removable = part1(input);
        if removable.len() == 0 {
            break;
        }

        count += removable.len();
        for rm in removable {
            input[rm.0][rm.1] = b'.';
        }
    }

    count
}

fn parse(input: &str) -> Vec<Vec<u8>> {
    input.lines().map(|line| line.as_bytes().to_vec()).collect()
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1() {
        let input = include_str!("../../day4/data/test.txt");
        let input = super::parse(input);
        let result = super::part1(&input).len();
        assert_eq!(13, result);
    }

    #[test]
    fn part2() {
        let input = include_str!("../../day4/data/test.txt");
        let mut input = super::parse(input);
        let result = super::part2(&mut input);
        assert_eq!(43, result);
    }
}
