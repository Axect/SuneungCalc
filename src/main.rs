#[macro_use]
extern crate prettytable;
use prettytable::Table;
use peroxide::fuga::*;
use suneung2024::{
    score::{Record, University::*},
    history::History,
};

macro_rules! add_univ_score {
    ($table:expr, $record:expr, $univ: ident, $year: expr) => {
        let score = $record.calc_with_university($univ, $year);
        $table.add_row(row![c->$univ.name(), c->format!("{:.2}", score)]);
    }
}

fn main() {
    let year = std::env::args()
        .nth(1)
        .and_then(|x| x.parse().ok())
        .unwrap_or(2024);

    let record_2024 = Record::read_parquet("dongwoo");
    let df_2024 = record_2024.to_dataframe();
    df_2024.print();

    let record_2023 = History::load_2023().eval_all(&record_2024);
    let df_2023 = record_2023.to_dataframe();
    df_2023.print();

    let record_2022 = History::load_2022().eval_all(&record_2024);
    let df_2022 = record_2022.to_dataframe();
    df_2022.print();

    let record = match year {
        2024 => record_2024,
        2023 => record_2023,
        2022 => record_2022,
        _ => panic!(),
    };

    let mut table = Table::new();
    add_univ_score!(table, record, KYUNGHEE,        year);
    add_univ_score!(table, record, DONGGUK,         year);
    add_univ_score!(table, record, KOOKMIN,         year);
    add_univ_score!(table, record, CATHOLIC,        year);
    add_univ_score!(table, record, SEOULSCITECH,    year);
    add_univ_score!(table, record, SOONGSIL,        year);
    add_univ_score!(table, record, AJU,             year);
    add_univ_score!(table, record, INHA,            year);
    add_univ_score!(table, record, SEJONG,          year);
    add_univ_score!(table, record, ERICA,           year);
    add_univ_score!(table, record, KWANGWOON,       year);

    table.printstd();
}
