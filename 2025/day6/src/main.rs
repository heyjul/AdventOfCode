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
    todo!()
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
