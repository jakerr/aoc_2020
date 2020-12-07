use std::fmt::Debug;

pub trait RecordFactory<T> {
    fn new_record(&self) -> T;
    fn accept_field(&self, record: &mut T, field: &str);
}

pub struct DefaultRecordFactory;

impl RecordFactory<Vec<String>> for DefaultRecordFactory {
    fn new_record(&self) -> Vec<String> {
        vec![]
    }

    fn accept_field(&self, record: &mut Vec<String>, field: &str) {
        record.push(field.into());
    }
}

pub struct RecordParser {
    pub rec_sep: String,
    pub field_sep: String,
}

impl Default for RecordParser {
    fn default() -> Self {
        RecordParser {
            rec_sep: "\n\n".into(),
            field_sep: " ".into(),
        }
    }
}

impl RecordParser {
    fn parse_record_string<T>(&mut self, factory: &impl RecordFactory<T>, record_string: &str) -> T
    where
        T: Debug,
    {
        let field_sep = self.field_sep.clone();
        let mut record = factory.new_record();
        for rline in record_string.lines() {
            if rline.is_empty() {
                continue;
            }
            for field_str in rline.split(&field_sep) {
                factory.accept_field(&mut record, field_str);
            }
        }
        record
    }

    pub fn parse<T>(&mut self, factory: impl RecordFactory<T>, input: &str) -> Vec<T>
    where
        T: Debug,
    {
        let mut records = vec![];
        let record_sep = self.rec_sep.clone();
        let mut record_str = "".to_string();
        for line in input.lines() {
            record_str.push_str(line);
            record_str.push('\n');
            if record_str.ends_with(&record_sep) {
                records.push(self.parse_record_string(&factory, &record_str));
                record_str = "".to_string();
            }
        }
        records.push(self.parse_record_string(&factory, &record_str));
        records
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};
    #[derive(Serialize, Deserialize, Debug)]
    pub struct Party {
        hey: u32,
        we: u32,
        doo: Option<u32>,
        like: Option<u32>,
        to: Option<u32>,
        party: Option<u32>,
    }
    #[test]
    fn record_parser() {
        use indoc::indoc;
        let input = indoc! {"
            hey:22 we:21
            like:12 to:29 party:99
            
            hey:99
            we:182
            doo:88
        "};

        let mut parser: RecordParser = Default::default();
        let out: Vec<Vec<String>> = parser.parse(DefaultRecordFactory, input);
        for party in out {
            println!("{:?}", party);
        }
    }
}
