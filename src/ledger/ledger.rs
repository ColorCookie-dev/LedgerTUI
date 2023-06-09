use std::collections::HashMap;

use itertools::Itertools;

pub use super::Record;

#[derive(Debug)]
pub struct Ledger {
    records: Vec<Record>,
    // totals: HashMap<String, i32>,
}

impl Ledger {
    pub fn from_path<'a>(path: impl std::convert::AsRef<std::path::Path>)
        -> anyhow::Result<Self> {
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

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn save_to_path(&self, path: impl AsRef<std::path::Path>)
        -> anyhow::Result<()> {
        let mut writer = csv::Writer::from_path(path.as_ref())?;
        for entry in self.entries() {
            writer.serialize(entry)?;
        }

        Ok(())
    }

    pub fn add_entry(&mut self, name: &str, amount: i32) {
        self.records.push(Record::new(name, amount));
    }

    pub fn entries(&self) -> Vec<Record> {
        self.records.clone()
    }

    /*
    pub fn totals(&self) -> &HashMap<String, i32> {
        &self.totals
    }
    */

    pub fn totals(&self) -> Vec<(String, i32)> {
        let mut totals = HashMap::<String, i32>::new();
        self.records.iter().for_each(
        |record| {
            totals.entry(record.recipient().to_string())
                .and_modify(|amount| *amount += record.amount())
                .or_insert(record.amount());
        });
        totals
            .iter()
            .map(|(name, amt)| (name.to_owned(), amt.to_owned()))
            .collect_vec()
    }

    pub fn total(&mut self, name: impl AsRef<str>) -> i32 {
        self.records.iter()
            .filter(|rec| rec.recipient() == name.as_ref())
            .fold(0, |total, rec| total + rec.amount())
    }
}

impl PartialEq for Ledger {
    fn eq(&self, other: &Self) -> bool {
        self.records == other.records
    }
}

#[cfg(test)]
mod test {
    use super::Ledger;

    #[test]
    fn test_ledger_write_path()
        -> anyhow::Result<()> {
        let path = std::path::Path::new("ledger.csv");
        let mut ledger = Ledger::new();
        ledger.add_entry("Alice", 1000);
        ledger.add_entry("Bob", -100);
        ledger.save_to_path(path)?;
        let new_ledger = Ledger::from_path(path)?;
        assert_eq!(ledger, new_ledger);
        Ok(())
    }

    #[test]
    #[ignore]
    fn fill_with_random_data() -> anyhow::Result<()> {
        use rand::{Rng, thread_rng};
        let ledger_path = "ledger.csv";
        let popular_names = "popular-names.txt";
        let mut ledger = Ledger::new();

        let name_rows = std::fs::read_to_string(popular_names)
            .expect(&format!("Failed to read {} file", popular_names));
        let names = name_rows.lines().map(|line| line.
                                          split_once('\t')
                                          .unwrap_or(("", "")).0
                                          ).collect::<Vec<&str>>();

        let no_of_entries = 100;
        // Fill with random values
        let total_no_of_names = names.len();
        (1..=no_of_entries)
            .map(|_| thread_rng().gen_range(0..total_no_of_names))
            .filter_map(|idx| names.get(idx))
            .cloned()
            .map(|name| (name, thread_rng().gen_range(-10000..=10000)))
            .for_each(|(name, amt)| ledger.add_entry(name, amt));

        ledger.save_to_path(ledger_path)?;
        let new_ledger = Ledger::from_path(ledger_path)?;
        assert_eq!(ledger, new_ledger);
        Ok(())
    }
}
