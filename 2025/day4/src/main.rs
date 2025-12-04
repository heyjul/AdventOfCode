fn main() {
    let input = include_str!("../../day4/data/input.txt");
    let result = part1(input);
    println!("Part 1: {}", result);
    let result = part2(input);
    println!("Part 2: {}", result);
}

fn part1(input: &str) -> usize {
    let values = parse(input);
    let i_len = values.len();
    let j_len = values[0].len();
    let mut forkliftable = 0;

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
                forkliftable += 1;
            }
        }
    }

    forkliftable
}

fn part2(input: &str) -> usize {
    todo!()
}

fn parse(input: &str) -> Vec<Vec<u8>> {
    input.lines().map(|line| line.as_bytes().to_vec()).collect()
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1() {
        let input = include_str!("../../day4/data/test.txt");
        let result = super::part1(input);
        assert_eq!(13, result);
    }

    //#[test]
    //fn part2() {
    //    let input = include_str!("../../day4/data/test.txt");
    //    let result = super::part2(input);
    //    assert_eq!(3121910778619, result);
    //}
}
