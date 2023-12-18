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
    let df_2023 = record_2023.to_dataframe();
    df_2023.print();

    let history_2022 = History::load_2022();
    let record_2022 = history_2022.eval_all(&record);
    let df_2022 = record_2022.to_dataframe();
    df_2022.print();
}
