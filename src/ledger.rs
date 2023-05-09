mod record;
mod error;

// use std::collections::HashMap;
use record::Record;
pub use error::Error;
use serde::Serialize;

#[derive(Debug)]
pub struct Ledger {
    records: Vec<Record>,
    // totals: HashMap<String, i32>,
}

impl Ledger {
    pub fn from_path<'a>(path: impl std::convert::AsRef<std::path::Path>)
        -> Result<Self, Error> {
        let mut records = csv::Reader::from_path(path)?;
        let records = records.deserialize::<Record>()
            .flatten()
            .collect::<Vec<Record>>();

        /*
        let totals = records
            .iter()
            .fold(HashMap::<String, i32>::new(),
            |mut totals, Record{id: _, name, credit}| {
                totals.entry(name.into())
                    .and_modify(|total_credits| *total_credits += credit)
                    .or_insert(credit.clone());
                totals
        });
        */

        Ok(Self { records, /* totals, */ })
    }

    pub fn new() -> Self {
        Self { records: Vec::new() }
    }

    pub fn save_to_path(&self, path: impl AsRef<std::path::Path>)
        -> Result<(), std::io::Error> {
        let mut writer = csv::Writer::from_path(path.as_ref())?;
        for entry in self.entries() {
            writer.serialize(entry)?;
        }

        Ok(())
    }

    pub fn add_entry(&mut self, name: &str, amount: i32) {
        self.records.push(Record::new(name, amount));
    }

    pub fn entries(&self) -> impl Iterator<Item = &Record> {
        self.records.iter()
    }

    /*
    pub fn totals(&self) -> &HashMap<String, i32> {
        &self.totals
    }
    */

    pub fn total(&mut self, name: impl AsRef<str>) -> i32 {
        self.records.iter()
            .filter(|rec| rec.name() == name.as_ref())
            .fold(0, |total, rec| total + rec.amount())
    }
}

impl PartialEq for Ledger {
    fn eq(&self, other: &Self) -> bool {
        self.entries().eq(other.entries())
    }
}

#[cfg(test)]
mod test {
    use super::Ledger;

    #[test]
    fn test_ledger_write_path()
        -> Result<(), Box<dyn std::error::Error>> {
        let path = std::path::Path::new("ledger.csv");
        let mut ledger = Ledger::new();
        ledger.add_entry("Alice", 1000);
        ledger.add_entry("Bob", -100);
        ledger.save_to_path(path)?;
        let new_ledger = Ledger::from_path(path)?;
        assert_eq!(ledger, new_ledger);
        Ok(())
    }
}
