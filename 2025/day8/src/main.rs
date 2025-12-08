use std::collections::HashMap;

use itertools::Itertools;

fn main() {
    let input = include_str!("../../day8/data/input.txt");
    let result = part1(input, 1000);
    println!("Part 1: {}", result);
    let result = part2(input);
    println!("Part 2: {}", result);
}

fn part1(input: &str, pairs: usize) -> usize {
    let values = parse(input);
    let mut coords: Vec<_> = values.iter().combinations(2).collect();
    coords.sort_unstable_by(|a, b| a[0].distance(a[1]).total_cmp(&b[0].distance(b[1])));

    let mut map: HashMap<&Coord, usize> = HashMap::new();
    let mut circuits = vec![vec![]; pairs];
    let mut i = 0;
    for x in coords.into_iter().take(pairs) {
        match (map.get(x[0]), map.get(x[1])) {
            (None, None) => {
                circuits[i] = vec![x[0], x[1]];
                map.insert(x[0], i);
                map.insert(x[1], i);
            }
            (None, Some(i)) => {
                circuits[*i].push(x[0]);
                map.insert(x[0], *i);
            }
            (Some(i), None) => {
                circuits[*i].push(x[1]);
                map.insert(x[1], *i);
            }
            (Some(i), Some(j)) => {
                if i == j {
                    continue;
                }

                let i = *i;
                let j = *j;
                let mut copy = circuits[j].clone();
                circuits[i].append(&mut copy);
                circuits[j].clear();
                for (_, v) in map.iter_mut() {
                    if v == &j {
                        *v = i;
                    }
                }
            }
        }

        i += 1;
    }

    let mut values: Vec<_> = circuits.into_iter().map(|x| x.len()).collect();
    values.sort_unstable_by(|a, b| b.cmp(&a));
    values.into_iter().take(3).product()
}

fn part2(input: &str) -> usize {
    let values = parse(input);
    let mut coords: Vec<_> = values.iter().combinations(2).collect();
    coords.sort_unstable_by(|a, b| a[0].distance(a[1]).total_cmp(&b[0].distance(b[1])));

    let mut map: HashMap<&Coord, usize> = HashMap::new();
    let mut circuits = vec![vec![]; coords.len()];
    for (i, x) in coords.iter().enumerate() {
        match (map.get(x[0]), map.get(x[1])) {
            (None, None) => {
                circuits[i] = vec![x[0], x[1]];
                map.insert(x[0], i);
                map.insert(x[1], i);
            }
            (None, Some(i)) => {
                circuits[*i].push(x[0]);
                map.insert(x[0], *i);
            }
            (Some(i), None) => {
                circuits[*i].push(x[1]);
                map.insert(x[1], *i);
            }
            (Some(i), Some(j)) => {
                if i == j {
                    continue;
                }

                let i = *i;
                let j = *j;
                let mut copy = circuits[j].clone();
                circuits[i].append(&mut copy);
                circuits[j].clear();
                for (_, v) in map.iter_mut() {
                    if v == &j {
                        *v = i;
                    }
                }
            }
        }

        if coords.iter().all(|x| map.contains_key(x[0]) && map.contains_key(x[1])) {
            return (x[0].0 * x[1].0) as usize;
        }
    }

    unreachable!()
}

fn parse(input: &str) -> Vec<Coord> {
    input
        .lines()
        .map(|line| {
            let values: Vec<_> = line.split(',').collect();
            Coord(
                values[0].parse().unwrap(),
                values[1].parse().unwrap(),
                values[2].parse().unwrap(),
            )
        })
        .collect()
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Coord(isize, isize, isize);

impl Coord {
    pub fn distance(&self, other: &Self) -> f32 {
        (((self.0 - other.0).pow(2) + (self.1 - other.1).pow(2) + (self.2 - other.2).pow(2)) as f32)
            .sqrt()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1() {
        let input = include_str!("../../day8/data/test.txt");
        let result = super::part1(input, 10);
        assert_eq!(40, result);
    }

    #[test]
    fn part2() {
        let input = include_str!("../../day8/data/test.txt");
        let result = super::part2(input);
        assert_eq!(25272, result);
    }
}
