fn main() {
    let input = include_str!("../../day7/data/input.txt");
    let result = part1(input);
    println!("Part 1: {}", result);
    let result = part2(input);
    println!("Part 2: {}", result);
}

fn part1(values: &str) -> usize {
    let lines: Vec<_> = values.lines().collect();
    let mut beams = vec![0; lines.first().unwrap().len()];
    let mut count = 0;
    for line in lines {
        for (i, x) in line.as_bytes().iter().enumerate() {
            if *x == b'S' {
                beams[i] = 1;
                continue;
            }

            if *x == b'^' && beams[i] == 1 {
                beams[i] = 0;
                if let Some(i) = i.checked_sub(1) {
                    beams[i] = 1;
                }
                if let Some(x) = beams.get_mut(i + 1) {
                    *x = 1;
                }
                count += 1;
            }
        }
    }

    count
}

fn part2(input: &str) -> usize {
    let lines: Vec<_> = input.lines().map(|line| line.as_bytes()).collect();
    for (i, line) in lines.iter().enumerate() {
        for (j, x) in line.iter().enumerate() {
            if *x == b'S' {
                return solve(j, &lines[(i + 1)..]) + 1;
            }
        }
    }
    0
}

fn solve(beam: usize, lines: &[&[u8]]) -> usize {
    let mut count = 0;
    let mut stop = false;
    for (i, line) in lines.iter().enumerate() {
        for (j, x) in line.iter().enumerate() {
            if *x == b'^' && beam == j {
                stop = true;
                count += 1;
                if let Some(j) = j.checked_sub(1) {
                    count += solve(j, &lines[(i + 1)..]);
                }
                if j + 1 < line.len() {
                    count += solve(j + 1, &lines[(i + 1)..]);
                }
            }
        }

        if stop {
            break;
        }
    }

    count
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1() {
        let input = include_str!("../../day7/data/test.txt");
        let result = super::part1(input);
        assert_eq!(21, result);
    }

    #[test]
    fn part2() {
        let input = include_str!("../../day7/data/test.txt");
        let result = super::part2(input);
        assert_eq!(40, result);
    }
}
