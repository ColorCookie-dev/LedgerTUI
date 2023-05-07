mod record;
mod error;

use csv::Reader;
// use std::collections::HashMap;
use record::Record;
pub use error::Error;

pub struct Ledger {
    records: Vec<Record>,
    // totals: HashMap<String, i32>,
}

impl Ledger {
    pub fn from_path<'a>(path: impl std::convert::AsRef<std::path::Path>)
        -> Result<Self, Error> {
        let mut records = Reader::from_path(path)?;
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

