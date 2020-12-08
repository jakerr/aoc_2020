use crate::record_parser::{RecordFactory, RecordParser};
use std::collections::{HashMap, HashSet};

struct RuleBookFactory;

impl RecordFactory<BagRuleBook> for RuleBookFactory {
    fn new_record(&self) -> BagRuleBook {
        Default::default()
    }

    fn accept_field(&self, rules: &mut BagRuleBook, field: &str) {
        let rule_text: Vec<&str> = field.split(" contain ").collect();
        let container = rule_text[0].trim_end_matches(" bags");
        let contained = rule_text[1].split(", ");
        for bag in contained {
            let count_and_bag: Vec<&str> = bag.split(' ').collect();
            if let Ok(bag_count) = count_and_bag[0].parse() {
                let bag_name = count_and_bag[1..3].join(" ");
                let bag_rule = rules.update(container);
                bag_rule.contains.push((bag_name.clone(), bag_count));
                let inner_bag = rules.update(&bag_name);
                inner_bag.contained_by.push(container.to_string());
            }
        }
    }
}

#[derive(Debug, Default)]
pub struct BagRules {
    contains: Vec<(String, u8)>,
    contained_by: Vec<String>,
}

#[derive(Debug, Default)]
pub struct BagRuleBook {
    rules: HashMap<String, BagRules>,
}

impl BagRuleBook {
    pub fn update(&mut self, rule: &str) -> &mut BagRules {
        self.rules.entry(rule.into()).or_default()
    }

    pub fn collect_unique_outer_bags(&self, bag_name: &str, seen: &mut HashSet<String>) {
        for outer_bag in &self.rules.get(bag_name).unwrap().contained_by {
            seen.insert(outer_bag.clone());
            self.collect_unique_outer_bags(outer_bag, seen);
        }
    }

    pub fn count_outer_bags(&self, bag_name: &str) -> u64 {
        let mut seen = HashSet::new();
        self.collect_unique_outer_bags(bag_name, &mut seen);
        seen.len() as u64
    }
    pub fn count_inner_bags(&self, bag_name: &str) -> u64 {
        let mut total = 0u64;
        for (inner_bag, count) in &self.rules.get(bag_name).unwrap().contains {
            let c = *count as u64;
            total += c + c * self.count_inner_bags(inner_bag);
        }
        total
    }
}

#[aoc_generator(day7)]
pub fn generate(input: &str) -> Vec<BagRuleBook> {
    let mut parser = RecordParser {
        // This is actually any string that doesn't appear in the input to mean
        // take the entire input as a single record.
        rec_sep: "<EOF>".into(),
        field_sep: "\n".into(),
    };
    parser.parse(RuleBookFactory, input)
}

#[aoc(day7, part1)]
pub fn part1(rules: &[BagRuleBook]) -> u64 {
    if let Some(rule_book) = rules.first() {
        return rule_book.count_outer_bags("shiny gold");
    }
    unreachable!();
}

#[aoc(day7, part2)]
pub fn part2(rules: &[BagRuleBook]) -> u64 {
    if let Some(rule_book) = rules.first() {
        return rule_book.count_inner_bags("shiny gold");
    }
    unreachable!();
}
