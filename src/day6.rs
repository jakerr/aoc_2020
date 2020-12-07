use std::collections::HashSet;

#[derive(Debug, Default)]
pub struct Group {
    answers: Vec<HashSet<char>>,
}

impl Group {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn append(&mut self, answer: HashSet<char>) {
        self.answers.push(answer);
    }

    // Count of any unique answer.
    pub fn count_unique(&self) -> u16 {
        let mut group = self.answers.first().unwrap().clone();
        for a in &self.answers[1..] {
            group.extend(a);
        }
        group.len() as u16
    }

    // Count of shared answers.
    pub fn count_shared(&self) -> u16 {
        let mut group = self.answers.first().unwrap().clone();
        for a in &self.answers[1..] {
            let g = group.intersection(a);
            group = g.cloned().collect();
        }
        group.len() as u16
    }
}

#[aoc_generator(day6)]
pub fn generate(input: &str) -> Vec<Group> {
    let mut group = Group::new();
    let mut output = vec![];
    for s in input.lines() {
        // Empty line means end of group.
        if s.is_empty() {
            output.push(group);
            group = Group::new();
        } else {
            let mut individual = HashSet::new();
            for c in s.chars() {
                individual.insert(c);
            }
            group.append(individual);
        }
    }
    output.push(group);
    output
}

#[aoc(day6, part1)]
pub fn part1(inputs: &[Group]) -> u16 {
    inputs.iter().map(|i| i.count_unique()).sum()
}

#[aoc(day6, part2)]
pub fn part2(inputs: &[Group]) -> u16 {
    inputs.iter().map(|i| i.count_shared()).sum()
}
