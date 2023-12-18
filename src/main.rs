use peroxide::fuga::*;
use suneung2024::{
    score::Record,
    history::History,
};

fn main() {
    let record = Record::read_parquet("dongwoo");
    let df = record.to_dataframe();
    df.print();

    let history_2023 = History::load_2023();

    let record_2023 = history_2023.eval_all(&record);
    let dg = record_2023.to_dataframe();
    dg.print();
}
