use peroxide::fuga::*;
use suneung2024::score::{Record, University::*};

fn main() {
    let record = Record::read_parquet("dongwoo");
    let df = record.to_dataframe();
    df.print();

    let kyunghee_2022 = record.calc_with_university(KYUNGHEE, 2022);
    println!("{:.4}", kyunghee_2022);
}
