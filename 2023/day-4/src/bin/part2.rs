use std::collections::HashMap;

fn main() {
    let input = include_str!("../input2.txt");
    let answer = part2(input);
    println!("{answer}");
}

fn part2(input: &str) -> u32 {
    input
        .lines()
        .map(|x| x.split_once(':').unwrap().1)
        .map(|x| x.split_once('|').unwrap())
        .enumerate()
        .fold(HashMap::new(), |mut acc, (i, (x, y))| {
            let win: Vec<_> = x.split(' ').filter_map(|x| x.parse::<u32>().ok()).collect();
            let count = y
                .split(' ')
                .filter_map(|x| x.parse::<u32>().ok())
                .filter(|x| win.contains(&x))
                .count();
            let entry = acc.entry(i).or_insert(0);
            *entry += 1;
            let n = *entry;

            (i + 1..i + 1 + count).fold(acc, |mut acc, i| {
                let entry2 = acc.entry(i).or_insert(0);
                *entry2 += 1 * n;
                acc
            })
        })
        .iter()
        .map(|(_, x)| *x)
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_part2() {
        const INPUT: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        assert_eq!(30, part2(INPUT));
    }
}
