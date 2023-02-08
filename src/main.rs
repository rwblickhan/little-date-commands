use chrono::prelude::*;
use std::{error::Error, fmt::Display};

#[derive(Debug)]
struct MissingDateError {}

impl Error for MissingDateError {}

impl Display for MissingDateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Missing date input")
    }
}

fn main() -> anyhow::Result<()> {
    let mut args = std::env::args();
    let name = args.next().unwrap();
    let input_date = args.next().ok_or(MissingDateError {})?;

    let then = NaiveDate::parse_from_str(&input_date, "%m-%d-%Y")
        .or(NaiveDate::parse_from_str(&input_date, "%m/%d/%Y"))?;
    let now = Utc::now().date_naive();

    let ago = now - then;

    if name.contains("til") {
        println!("{}", -ago.num_days());
    } else {
        println!("{}", ago.num_days());
    }

    anyhow::Ok(())
}
