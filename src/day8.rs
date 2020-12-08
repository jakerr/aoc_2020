use crate::record_parser::{RecordFactory, RecordParser};
use std::collections::HashSet;

struct ProgFactory;

impl RecordFactory<(String, i64)> for ProgFactory {
    fn new_record(&self) -> (String, i64) {
        Default::default()
    }

    fn accept_field(&self, statement: &mut (String, i64), field: &str) {
        let parts: Vec<&str> = field.split(' ').collect();
        let instruction = parts[0];
        let arg = parts[1].parse::<i64>().unwrap();
        statement.0 = instruction.into();
        statement.1 = arg;
    }
}

pub fn eval(program: &[(String, i64)], do_fix: bool) -> i64 {
    let mut tweak_next: usize = 0;
    let mut acc: i64;
    let tweak_idxs: Vec<usize> = program
        .iter()
        .enumerate()
        .filter_map(|(idx, (ins, _))| match ins.as_str() {
            "jmp" => Some(idx),
            "nop" => Some(idx),
            _ => None,
        })
        .collect();
    loop {
        let mut normal_exit = true;
        let mut ip: usize = 0;
        let mut ex = HashSet::new();
        acc = 0;
        loop {
            let i = &program[ip];
            let tweak_now = &tweak_idxs[tweak_next];
            if ex.contains(&ip) {
                normal_exit = false;
                break;
            }
            ex.insert(ip);
            let mut instruction = i.0.as_str();
            if do_fix && ip == *tweak_now {
                instruction = match instruction {
                    "jmp" => "nop",
                    "nop" => "jmp",
                    _ => instruction,
                };
            }
            let mut do_jump = false;
            match instruction {
                "acc" => {
                    acc += i.1;
                }
                "jmp" => {
                    ip = ((ip as i64) + i.1) as usize;
                    do_jump = true;
                }
                _ => (),
            }
            if !do_jump {
                ip += 1 as usize;
            }
            if ip >= program.len() {
                break;
            }
        }
        if normal_exit || !do_fix {
            break;
        }
        tweak_next += 1;
        if tweak_next >= tweak_idxs.len() {
            println!("Couldn't find a tweak allowing normal exit.");
            break;
        }
    }
    acc
}

#[aoc_generator(day8)]
pub fn generate(input: &str) -> Vec<(String, i64)> {
    let mut parser = RecordParser {
        rec_sep: "\n".into(),
        field_sep: "\n".into(),
    };
    parser.parse(ProgFactory, input)
}

#[aoc(day8, part1)]
pub fn part1(program: &[(String, i64)]) -> i64 {
    eval(program, false)
}

#[aoc(day8, part2)]
pub fn part2(program: &[(String, i64)]) -> i64 {
    eval(program, true)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example() {
        assert_eq!(
            eval(
                &[
                    ("nop".into(), 0),
                    ("acc".into(), 1),
                    ("jmp".into(), 4),
                    ("acc".into(), 3),
                    ("jmp".into(), -3),
                    ("acc".into(), -99),
                    ("acc".into(), 1),
                    ("jmp".into(), -4),
                    ("acc".into(), 6),
                ],
                true,
            ),
            8
        )
    }
}
