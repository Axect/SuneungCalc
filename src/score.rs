use std::hash::Hash;
use std::collections::HashMap;
use peroxide::fuga::*;
use paste::paste;
use crate::university_weight::*;

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

    pub fn calc_with_university(&self, university: University, year: usize) -> f64 {
        let weight = UniversityWeight::load(university, year);
        let weight_sum_except_eng = weight.korean + weight.math + weight.science;
        let weight_eng = weight.english;
        let weight_sum = weight_sum_except_eng + weight_eng;

        let korean = self.korean().standard_score() * weight.korean / weight_sum_except_eng;
        let math = self.math().standard_score() * weight.math / weight_sum_except_eng;
        let science_required = weight.science_required();
        let science_cand = match science_required {
            1 => self.chemistry().standard_score().max(self.earth_science().standard_score()) * 2f64,
            2 => self.chemistry().standard_score() + self.earth_science().standard_score(),
            _ => unreachable!()
        };
        let science = science_cand * weight.science / weight_sum_except_eng;

        let total = (korean + math + science) * 3f64;

        let eng_rank = self.english().rank();
        let eng_required_rank = weight.english_required();
        let eng_table = weight.english_table();

        let eng_default_score = eng_table[eng_required_rank];
        let eng_score = eng_table[eng_rank];

        if weight_eng > 0f64 {
            total + (eng_score - eng_default_score) * weight_eng / weight_sum
        } else {
            total + (eng_score - eng_default_score) / 4f64
        }
    }
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum University {
    KYUNGHEE
}

#[derive(Debug, Clone)]
pub struct UniversityWeight {
    korean: f64,
    math: f64,
    english: f64,
    science: f64,
    science_required: usize, // Number of required subjects
    english_required: usize, // Default rank
    english_table: Vec<f64>,
}

macro_rules! make_university_weight {
    ($univ:ident, $year:expr) => {
        {
            paste! {
                let weight = [<$univ _ $year _WEIGHT>].to_vec();
                let korean = weight[0];
                let math = weight[1];
                let english = weight[2];
                let science = weight[3];
                let science_required = [<$univ _ $year _SCI_REQ>];
                let english_required = [<$univ _ $year _ENG_REQ>];
                let english_table = [<$univ _$year _ENG>].to_vec().iter().map(|x| *x as f64).collect::<Vec<f64>>();

                UniversityWeight {
                    korean: korean as f64,
                    math: math as f64,
                    english: english as f64,
                    science: science as f64,
                    science_required,
                    english_required,
                    english_table,
                }
            }
        }
    }
}

impl UniversityWeight {
    pub fn load(univ: University, year: usize) -> Self {
        match (univ, year) {
            (University::KYUNGHEE, 2022) => make_university_weight!(KYUNGHEE, 2022),
            _ => unimplemented!(),
        }
    }

    pub fn korean(&self) -> f64 {
        self.korean
    }

    pub fn math(&self) -> f64 {
        self.math
    }

    pub fn english(&self) -> f64 {
        self.english
    }

    pub fn science(&self) -> f64 {
        self.science
    }

    pub fn science_required(&self) -> usize {
        self.science_required
    }

    pub fn english_required(&self) -> usize {
        self.english_required
    }

    pub fn english_table(&self) -> &Vec<f64> {
        &self.english_table
    }
}
