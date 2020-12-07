use itertools::Itertools;
use std::collections::HashSet;

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<u32> {
    input
        .lines()
        .map(|l| {
            let num = l.trim().parse().unwrap();
            num
        })
        .collect()
}

#[aoc(day1, part1)]
pub fn adds_up(input: &[u32]) -> u32 {
    // The set of 'a' for which '2020 - a' was already seen.
    let mut seen = HashSet::new();
    for k in input {
        if seen.contains(k) {
            return (2020 - k) * k;
        }
        seen.insert(2020 - k);
    }
    unreachable!();
}

#[aoc(day1, part2, brute)]
pub fn part2_brute(input: &[u32]) -> u32 {
    for k in input.iter().combinations(3) {
        if k[0] + k[1] + k[2] == 2020 {
            return k[0] * k[1] * k[2];
        }
    }
    unreachable!();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn generator() {
        assert_eq!(input_generator("2\n30\n"), vec![2, 30]);
    }

    #[test]
    fn part1() {
        assert_eq!(adds_up(&[2019, 88, 1]), 2019);
        assert_eq!(adds_up(&[20, 88, 1, 2000]), 40_000);
    }
}
