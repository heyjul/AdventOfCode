fn main() {
    let input = include_str!("../../day1/data/part1.txt");
    let count = parse(input);

    println!("{count}");
}

fn parse(input: &str) -> i32 {
    let mut count = 0;
    input.lines().fold(50, |mut acc, line| {
        let mut chars = line.chars();
        let mul = match chars.next().unwrap() {
            'L' => -1,
            'R' => 1,
            _ => unreachable!(),
        };
        let value = chars.collect::<String>().parse::<usize>().unwrap();

        for _ in 0..value {
            acc = (acc + 1 * mul) % 100;
            if acc == 0 {
                count += 1;
            }
            if acc < 0 {
                acc = 99;
            }
        }

        acc
    });

    count
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_part2() {
        let input = include_str!("../../day1/data/test.txt");

        let result = super::parse(input.trim());
        assert_eq!(6, result);
    }
}
