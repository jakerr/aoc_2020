#[derive(Debug)]
pub struct Password {
    rule: (u32, u32, char),
    value: String,
}

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<Password> {
    input
        .lines()
        .map(|l| {
            let parts: Vec<&str> = l.trim().split(' ').collect();
            let range: Vec<&str> = parts[0].split('-').collect();
            let c = parts[1].chars().next().unwrap();
            Password {
                rule: (range[0].parse().unwrap(), range[1].parse().unwrap(), c),
                value: parts[2].into(),
            }
        })
        .collect()
}

#[aoc(day2, part1)]
pub fn count_valid(input: &[Password]) -> u32 {
    let mut valid_count = 0u32;
    for p in input {
        let mut valid = false;
        let mut count = 0;
        let (min, max, target) = p.rule;
        for c in p.value.chars() {
            if c == target {
                count += 1;
                if count >= min {
                    valid = true;
                }
                if count > max {
                    valid = false;
                    break;
                }
            }
        }
        if valid {
            valid_count += 1;
        } else {
            println!("Invalid password {:?}", p);
        }
    }
    valid_count
}

#[aoc(day2, part2)]
pub fn count_valid2(input: &[Password]) -> u32 {
    let mut valid_count = 0u32;
    for p in input {
        let (p0, p1, target) = p.rule;
        let p0 = (p0 - 1) as usize;
        let p1 = (p1 - 1) as usize;

        if p.value.len() <= p0 || p.value.len() <= p1 {
            println!("Short password {:?}", p);
            continue;
        }
        let c0 = p.value.chars().nth(p0).unwrap();
        let c1 = p.value.chars().nth(p1).unwrap();
        if (c0 == target) ^ (c1 == target) {
            valid_count += 1;
        } else {
            println!("Invalid password {:?}", p);
        }
    }
    valid_count
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn generator() {
        let input = r#"1-3 a: abcde
            1-3 b: cdefg
            2-9 c: ccccccccc"#;
        let v = input_generator(input);
        let p0 = &v[0];
        assert_eq!(p0.rule, (1, 3, 'a'));
        assert_eq!(p0.value, "abcde");
        let p1 = &v[1];
        assert_eq!(p1.rule, (1, 3, 'b'));
        assert_eq!(p1.value, "cdefg");
        let p2 = &v[2];
        assert_eq!(p2.rule, (2, 9, 'c'));
        assert_eq!(p2.value, "ccccccccc");
    }

    #[test]
    fn part1() {}
}
