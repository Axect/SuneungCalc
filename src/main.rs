#[macro_use]
extern crate prettytable;
use prettytable::Table;
use peroxide::fuga::*;
use suneung2024::score::{Record, University::*};

macro_rules! add_univ_score {
    ($table:expr, $record:expr, $univ: ident, $year: expr) => {
        let score = $record.calc_with_university($univ, $year);
        $table.add_row(row![c->$univ.name(), c->format!("{:.1}", score)]);
    }
}

fn main() {
    let record = Record::read_parquet("dongwoo");
    let df = record.to_dataframe();
    df.print();

    let mut table = Table::new();
    add_univ_score!(table, record, KYUNGHEE, 2022);
    add_univ_score!(table, record, DONGGUK, 2022);
    add_univ_score!(table, record, SEOULSCITECH, 2022);
    add_univ_score!(table, record, KWANGWOON, 2022);

    table.printstd();
}
