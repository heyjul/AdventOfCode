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
    let lines: Vec<_> = input.lines().collect();
    let mut beams = vec![0; lines.first().unwrap().len()];
    for line in lines {
        for (i, x) in line.as_bytes().iter().enumerate() {
            if *x == b'S' {
                beams[i] = 1;
                continue;
            }

            let val = beams[i];
            if *x == b'^' && val > 0 {
                if let Some(j) = i.checked_sub(1) {
                    beams[j] += val;
                }
                if let Some(x) = beams.get_mut(i + 1) {
                    *x += val;
                }
                beams[i] = 0;
            }
        }
    }

    beams.iter().sum()
}
//         1
//        1 1
//       1 2 1
//      1 3 3 1
//     1 4 331 1
//    1 5 434 2 1
//   1 154 74 21 1
//1 2 10 11 11 211 1
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
