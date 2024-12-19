#[macro_use]
extern crate prettytable;
use dialoguer::{theme::ColorfulTheme, Input, Select};
use peroxide::fuga::*;
use prettytable::Table;
use suneung_calc::{
    history::History,
    score::{Record, Subject, University::*},
};

macro_rules! add_univ_score {
    ($table:expr, $record:expr, $univ: ident, $year: expr) => {
        let score = $record.calc_with_university($univ, $year);
        $table.add_row(row![c->$univ.name(), c->format!("{:.2}", score)]);
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Check "data" directory exists. If not, create it.
    if !std::path::Path::new("data").exists() {
        std::fs::create_dir("data").unwrap();
    }

    // Make options for choosing subdiretory or creating new one
    let mut options = vec![];
    for entry in std::fs::read_dir("data")? {
        let path = entry?.path();
        if path.is_dir() {
            if let Some(name) = path.file_name() {
                if let Some(name) = name.to_str() {
                    options.push(name.to_string());
                }
            }
        }
    }
    options.push("Create new record".to_string());

    // Choose subdirectory or create new one
    let theme = ColorfulTheme::default();
    let record = loop {
        let idx = Select::with_theme(&theme)
            .with_prompt("Choose record")
            .items(&options)
            .default(0)
            .interact()?;

        // If create new directory, input name and scores
        if idx == options.len() - 1 {
            let name: String = Input::with_theme(&theme)
                .with_prompt("Input student name")
                .interact()?;
            let subjects = vec![
                Subject::Korean,
                Subject::Math,
                Subject::English,
                Subject::Chemistry,
                Subject::EarthScience,
            ];

            let mut record = Record::new(name.as_str());
            for subject in subjects {
                let standard_score = Input::with_theme(&theme)
                    .with_prompt(format!("Input {} standard score", subject.name()))
                    .interact()?;
                let percentile = Input::with_theme(&theme)
                    .with_prompt(format!("Input {} percentile", subject.name()))
                    .interact()?;
                let grade = Input::with_theme(&theme)
                    .with_prompt(format!("Input {} grade", subject.name()))
                    .interact()?;
                record.record(subject, standard_score, percentile, grade);
            }

            record.write_parquet()?;
        } else {
            let record_name = options[idx].clone();
            let record = Record::read_parquet(record_name.as_str());
            break record;
        }
    };

    // Choose year
    let year = Select::with_theme(&theme)
        .with_prompt("Choose year")
        .default(0)
        .items(&[2025, 2024, 2023, 2022])
        .interact()?;

    let df = match year {
        2025 => record.to_dataframe(),
        _ => {
            let history = History::load(year)?;
            history.eval_all(&record).to_dataframe()
        }
    };

    let mut table = Table::new();
    add_univ_score!(table, df, KYUNGHEE, year);
    add_univ_score!(table, df, DONGGUK, year);
    add_univ_score!(table, df, KOOKMIN, year);
    add_univ_score!(table, df, CATHOLIC, year);
    add_univ_score!(table, df, SEOULSCITECH, year);
    add_univ_score!(table, df, SOONGSIL, year);
    add_univ_score!(table, df, AJU, year);
    add_univ_score!(table, df, INHA, year);
    add_univ_score!(table, df, SEJONG, year);
    add_univ_score!(table, df, ERICA, year);
    add_univ_score!(table, df, KWANGWOON, year);

    table.printstd();

    Ok(())
}
