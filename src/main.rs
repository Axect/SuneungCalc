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
    let record = Record::read_parquet("dongwoo");
    let df = record.to_dataframe();
    df.print();

    let df_2023 = History::load_2023().eval_all(&record).to_dataframe();
    df_2023.print();

    let df_2022 = History::load_2022().eval_all(&record).to_dataframe();
    df_2022.print();

    let mut table = Table::new();
    add_univ_score!(table, record, KYUNGHEE, 2022);
    add_univ_score!(table, record, DONGGUK, 2022);
    add_univ_score!(table, record, SEOULSCITECH, 2022);
    add_univ_score!(table, record, KOOKMIN, 2022);
    add_univ_score!(table, record, INHA, 2022);
    add_univ_score!(table, record, AJU, 2022);
    add_univ_score!(table, record, ERICA, 2022);
    add_univ_score!(table, record, SOONGSIL, 2022);
    add_univ_score!(table, record, SEJONG, 2022);
    add_univ_score!(table, record, KWANGWOON, 2022);

    table.printstd();
}
