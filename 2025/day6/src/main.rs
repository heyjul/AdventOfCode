fn main() {
    let input = include_str!("../../day6/data/input.txt");
    let result = part1(input);
    println!("Part 1: {}", result);
    let result = part2(input);
    println!("Part 2: {}", result);
}

fn part1(values: &str) -> usize {
    let (row_len, values) = parse(values);
    let len = values.len() / row_len;

    let mut count = 0;
    for i in 0..row_len {
        let mut vals = vec![];
        for j in 0..len {
            match &values[j * row_len + i] {
                Value::Num(x) => {
                    vals.push(*x);
                }
                Value::Str(x) => match x.as_str() {
                    "*" => count += vals.iter().product::<usize>(),
                    "+" => count += vals.iter().sum::<usize>(),
                    _ => panic!("Unknown operator"),
                },
            }
        }
    }

    count
}

fn part2(input: &str) -> usize {
    let lines: Vec<_> = input.lines().map(|x| x.as_bytes()).collect();
    let max_len = lines.iter().map(|x| x.len()).max().unwrap();

    let mut count = 0;
    let mut values: Vec<usize> = vec![];
    let mut v_index = 0;
    let mut next_line = false;
    for i in (0..max_len).rev() {
        for j in 0..lines.len() {
            match lines[j].get(i) {
                Some(x) if x.is_ascii_digit() => {
                    if let Some(val) = values.get_mut(v_index) {
                        *val *= 10;
                        *val += (x - b'0') as usize;
                    } else {
                        values.push((x - b'0') as usize);
                    }

                    next_line = true;
                }
                Some(x) if *x == b'*' => {
                    count += values.iter().product::<usize>();
                    values = vec![];
                    v_index = 0;
                    next_line = false;
                }
                Some(x) if *x == b'+' => {
                    count += values.iter().sum::<usize>();
                    values = vec![];
                    v_index = 0;
                    next_line = false;
                }
                _ => {}
            }
        }

        if next_line {
            v_index += 1;
        }
    }

    count
}

fn parse(input: &str) -> (usize, Vec<Value>) {
    let mut count = 0;
    let values = input
        .lines()
        .map(|line| {
            let values: Vec<_> = line
                .split_ascii_whitespace()
                .map(|val| {
                    val.parse::<usize>()
                        .map_or_else(|_| Value::Str(val.trim().to_string()), |x| Value::Num(x))
                })
                .collect();
            count = values.len();
            values
        })
        .flatten()
        .collect();
    (count, values)
}

#[derive(Debug)]
enum Value {
    Num(usize),
    Str(String),
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1() {
        let input = include_str!("../../day6/data/test.txt");
        let result = super::part1(input);
        assert_eq!(4277556, result);
    }

    #[test]
    fn part2() {
        let input = include_str!("../../day6/data/test.txt");
        let result = super::part2(input);
        assert_eq!(3263827, result);
    }
}
