use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::prelude::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct Record {
    transaction_id: Uuid,
    recipient: String,
    transaction_amount: i32,
    transaction_time: DateTime<Utc>,
}

impl PartialEq for Record {
    fn eq(&self, other: &Self) -> bool {
        self.transaction_id == other.transaction_id
    }
}

impl Record {
    pub fn name(&self) -> &str {
        &self.recipient
    }

    pub fn amount(&self) -> i32 {
        self.transaction_amount
    }

    pub fn new(name: &str, amount: i32) -> Self {
        Self {
            transaction_id: Uuid::new_v4(),
            recipient: name.to_string(),
            transaction_amount: amount,
            transaction_time: Utc::now(),
        }
    }
}

