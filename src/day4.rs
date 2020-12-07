use regex::Regex;
use serde::{Deserialize, Serialize};

use crate::record_parser::{RecordFactory, RecordParser};

struct PassportRecordFactory;

impl RecordFactory<Passport> for PassportRecordFactory {
    fn new_record(&self) -> Passport {
        Default::default()
    }

    fn accept_field(&self, record: &mut Passport, field: &str) {
        let values: Vec<&str> = field.split(':').collect();
        let key = values[0];
        let value = Some(values[1].to_string());
        match key {
            "byr" => record.byr = value,
            "iyr" => record.iyr = value,
            "eyr" => record.eyr = value,
            "hgt" => record.hgt = value,
            "hcl" => record.hcl = value,
            "ecl" => record.ecl = value,
            "pid" => record.pid = value,
            "cid" => record.cid = value,
            _ => unreachable!(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Passport {
    byr: Option<String>, // (Birth Year)
    iyr: Option<String>, // (Issue Year)
    eyr: Option<String>, // (Expiration Year)
    hgt: Option<String>, // (Height)
    hcl: Option<String>, // (Hair Color)
    ecl: Option<String>, // (Eye Color)
    pid: Option<String>, // (Passport ID)
    cid: Option<String>, // (Country ID)
}

impl Passport {
    fn valid_date(s: &str, lower: u32, upper: u32) -> bool {
        let re = Regex::new(r"^\d{4}$").unwrap();
        if !re.is_match(s) {
            return false;
        }
        let date: u32 = s.parse().unwrap();
        date >= lower && date <= upper
    }

    fn valid_height(s: &str) -> bool {
        let re = Regex::new(r"^\d+(cm|in)$").unwrap();
        if !re.is_match(s) {
            return false;
        }
        let num: u32 = (&s[..s.len() - 2]).parse().unwrap();
        if s.ends_with("cm") {
            num >= 150 && num <= 193
        } else {
            num >= 59 && num <= 76
        }
    }

    fn valid_hair(s: &str) -> bool {
        let re = Regex::new(r"^\#([0-9a-f]{6})$").unwrap();
        re.is_match(s)
    }

    fn valid_eye(s: &str) -> bool {
        let re = Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap();
        re.is_match(s)
    }

    fn valid_pid(s: &str) -> bool {
        let re = Regex::new(r"^([0-9]{9})$").unwrap();
        re.is_match(s)
    }

    pub fn validate(&self, strict: bool) -> bool {
        match self {
            Passport {
                byr: Some(byr), // Option<String>, // (Birth Year)
                iyr: Some(iyr), // Option<String>, // (Issue Year)
                eyr: Some(eyr), // Option<String>, // (Expiration Year)
                hgt: Some(hgt), // Option<String>, // (Height)
                hcl: Some(hcl), // Option<String>, // (Hair Color)
                ecl: Some(ecl), // Option<String>, // (Eye Color)
                pid: Some(pid), // Option<String>, // (Passport ID)
                cid: _,         // Option<String>, // (Country ID)
            } if !strict
                || (Self::valid_date(byr, 1920, 2002)
                    && Self::valid_date(iyr, 2010, 2020)
                    && Self::valid_date(eyr, 2020, 2030)
                    && Self::valid_height(hgt)
                    && Self::valid_hair(hcl)
                    && Self::valid_eye(ecl)
                    && Self::valid_pid(pid)) =>
            {
                true
            }
            _ => false,
        }
    }
}

#[aoc_generator(day4)]
pub fn generate(input: &str) -> Vec<Passport> {
    let mut parser: RecordParser = Default::default();
    parser.parse(PassportRecordFactory, input)
}

#[aoc(day4, part1)]
pub fn part1(passports: &[Passport]) -> u64 {
    passports.iter().filter(|p| p.validate(false)).count() as u64
}

#[aoc(day4, part2)]
pub fn part2(passports: &[Passport]) -> u64 {
    passports.iter().filter(|p| p.validate(true)).count() as u64
}
