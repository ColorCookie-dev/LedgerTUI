use std::{error::Error, convert::identity, iter::Filter};
use csv::{Reader, DeserializeRecordsIter};
use std::collections::HashMap;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Record {
    id: u32,
    name: String,
    credit: i32,
}

impl PartialEq for Record {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

pub struct Ledger {
    records: Vec<Record>,
    totals: HashMap<String, i32>,
}

impl Ledger {
    pub fn from_path(path: impl AsRef<std::path::Path>)
        -> Result<Self, Box<dyn Error>> {
        let mut records = Reader::from_path(path.as_ref())?;
        let records = records.deserialize::<Record>()
            .flatten()
            .collect::<Vec<Record>>();

        let totals = records
            .iter()
            .fold(HashMap::<String, i32>::new(),
            |mut totals, Record{id: _, name, credit}| {
                totals.entry(name.into())
                    .and_modify(|total_credits| *total_credits += credit)
                    .or_insert(credit.clone());
                totals
        });

        Ok(Self { records, totals, })
    }

    pub fn add_entry(&mut self, name: &str, credit: &str) {
        self.records.push(Record::new(name, credit));
    }

    pub fn totals_iter(&self) -> impl Iterator<Item = (&String, &i32)> {
        self.totals.iter()
    }

    pub fn search_totals<'a>(&'a self, search_term: &'a str)
        -> impl Iterator<Item = (&String, &i32)> + Clone + 'a {
        self.totals.iter().filter(
            move |(name, _total_credits)| name.starts_with(search_term)
        )
    }
}

