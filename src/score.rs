use std::hash::Hash;
use std::collections::HashMap;
use peroxide::fuga::*;

#[derive(Debug, Copy, Clone)]
pub struct Score {
    standard_score: f64,
    percentile: f64,
    rank: usize,
}

impl Score {
    pub fn standard_score(&self) -> f64 {
        self.standard_score
    }

    pub fn percentile(&self) -> f64 {
        self.percentile
    }

    pub fn rank(&self) -> usize {
        self.rank
    }
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum Subject {
    Korean,
    Math,
    English,
    Chemistry,
    EarthScience,
}

#[derive(Debug, Clone)]
pub struct Record {
    name: String,
    scores: HashMap<Subject, Score>,
}

impl Record {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            scores: HashMap::new(),
        }
    }

    pub fn record(&mut self, subject: Subject, standard_score: f64, percentile: f64, rank: usize) {
        self.scores.insert(subject, Score { standard_score, percentile, rank });
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn korean(&self) -> Score {
        *self.scores.get(&Subject::Korean).unwrap()
    }

    pub fn math(&self) -> Score {
        *self.scores.get(&Subject::Math).unwrap()
    }

    pub fn english(&self) -> Score {
        *self.scores.get(&Subject::English).unwrap()
    }

    pub fn chemistry(&self) -> Score {
        *self.scores.get(&Subject::Chemistry).unwrap()
    }

    pub fn earth_science(&self) -> Score {
        *self.scores.get(&Subject::EarthScience).unwrap()
    }

    pub fn standard_score(&self, subject: Subject) -> f64 {
        self.scores.get(&subject).unwrap().standard_score
    }

    pub fn percentile(&self, subject: Subject) -> f64 {
        self.scores.get(&subject).unwrap().percentile
    }

    pub fn rank(&self, subject: Subject) -> usize {
        self.scores.get(&subject).unwrap().rank
    }

    pub fn to_dataframe(&self) -> DataFrame {
        let mut df = DataFrame::new(vec![]);
        df.push("Korean", Series::new(vec![
            self.korean().standard_score(),
            self.korean().percentile(),
            self.korean().rank() as f64,
        ]));
        df.push("Math", Series::new(vec![
            self.math().standard_score(),
            self.math().percentile(),
            self.math().rank() as f64,
        ]));
        df.push("English", Series::new(vec![
            0f64,
            0f64,
            self.english().rank() as f64,
        ]));
        df.push("Chemistry", Series::new(vec![
            self.chemistry().standard_score(),
            self.chemistry().percentile(),
            self.chemistry().rank() as f64,
        ]));
        df.push("EarthScience", Series::new(vec![
            self.earth_science().standard_score(),
            self.earth_science().percentile(),
            self.earth_science().rank() as f64,
        ]));

        df
    }

    pub fn write_parquet(&self) {
        let df = self.to_dataframe();
        df.write_parquet(&format!("{}.parquet", self.name()), CompressionOptions::Uncompressed).unwrap();
    }

    pub fn read_parquet(name: &str) -> Self {
        let df = DataFrame::read_parquet(&format!("{}.parquet", name)).unwrap();
        let korean: Vec<f64> = df["Korean"].to_vec();
        let math: Vec<f64> = df["Math"].to_vec();
        let english: Vec<f64> = df["English"].to_vec();
        let chemistry: Vec<f64> = df["Chemistry"].to_vec();
        let earth_science: Vec<f64> = df["EarthScience"].to_vec();

        let mut record = Record::new(name);

        record.record(Subject::Korean, korean[0], korean[1], korean[2] as usize);
        record.record(Subject::Math, math[0], math[1], math[2] as usize);
        record.record(Subject::English, 0f64, 0f64, english[2] as usize);
        record.record(Subject::Chemistry, chemistry[0], chemistry[1], chemistry[2] as usize);
        record.record(Subject::EarthScience, earth_science[0], earth_science[1], earth_science[2] as usize);

        record
    }
}
