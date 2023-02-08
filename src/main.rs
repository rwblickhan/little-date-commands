use chrono::prelude::*;
use clap::Parser;

#[derive(clap::Subcommand)]
enum Command {
    Ago {
        input_date: String,
    },
    Til {
        input_date: String,
    },
    Between {
        input_date1: String,
        input_date2: String,
    },
}

#[derive(clap::Parser)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

fn parse(input_str: &str) -> anyhow::Result<NaiveDate> {
    Ok(NaiveDate::parse_from_str(&input_str, "%m-%d-%Y")
        .or(NaiveDate::parse_from_str(&input_str, "%m/%d/%Y"))?)
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    match args.command {
        Command::Ago { input_date } => {
            let then = parse(&input_date)?;
            let now = Local::now().date_naive();
            let ago = now - then;
            println!("{}", ago.num_days());
        }
        Command::Til { input_date } => {
            let then = parse(&input_date)?;
            let now = Local::now().date_naive();
            let til = then - now;
            println!("{}", til.num_days());
        }
        Command::Between {
            input_date1,
            input_date2,
        } => {
            let date1 = parse(&input_date1)?;
            let date2 = parse(&input_date2)?;
            let between = date2 - date1;
            println!("{}", between.num_days());
        }
    }

    anyhow::Ok(())
}
