use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Record {
    uuid: Uuid,
    name: String,
    amount: i32,
}

impl PartialEq for Record {
    fn eq(&self, other: &Self) -> bool {
        self.uuid == other.uuid
    }
}

impl Record {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn amount(&self) -> i32 {
        self.amount
    }

    pub fn new(name: &str, amount: i32) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            name: name.to_string(),
            amount,
        }
    }
}

