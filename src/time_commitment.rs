use std::time::Duration;
//use chrono::prelude::*;
use chrono::{Local, Datelike};
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Deserialize, Serialize)]
pub struct TimeCommitment {
    commitments: [Duration; 7],
}

impl TimeCommitment {
    pub fn for_number_from_sunday(&self, idx: usize) -> Duration {
        self.commitments[idx]
    }

    pub fn for_today(&self) -> Duration {
        self.for_number_from_sunday(Local::now().weekday().number_from_sunday() as usize)
    }
}
