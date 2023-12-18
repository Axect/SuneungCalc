use peroxide::fuga::*;
use crate::score::{Subject::*, Subject, Record};
use std::collections::HashMap;

use crate::suneung_data::{
    KOREAN_2023, MATH_2023, CHEM_2023, EARSCI_2023,
};

#[derive(Debug, Clone)]
pub struct History {
    year: usize,
    score_map: HashMap<Subject, Vec<f64>>,
    cs_map: HashMap<Subject, CubicHermiteSpline>,
}

impl History {
    pub fn new(year: usize) -> Self {
        Self {
            year,
            score_map: HashMap::new(),
            cs_map: HashMap::new(),
        }
    }

    pub fn year(&self) -> usize {
        self.year
    }

    pub fn record(&mut self, subject: Subject, scores: &[f64]) {
        self.score_map.insert(subject, scores.to_vec());

        let mut xs = vec![96f64, 89f64, 77f64, 60f64, 40f64, 23f64, 11f64, 4f64];
        let mut ys = scores.to_vec();

        xs.reverse();
        ys.reverse();

        let cs = cubic_hermite_spline(&xs, &ys, Quadratic);

        self.cs_map.insert(subject, cs);
    }

    pub fn eval(&self, subject: Subject, x: f64) -> f64 {
        self.cs_map.get(&subject).unwrap().eval(x)
    }

    pub fn eval_all(&self, student: &Record) -> Record {
        let mut record = Record::new(student.name());

        for subject in [Korean, Math, Chemistry, EarthScience] {
            record.record(
                subject,
                self.eval(subject, student.percentile(subject)),
                student.percentile(subject),
                student.rank(subject),
            );
        }

        record.record(English, 0f64, 0f64, student.rank(English));

        record
    }

    pub fn load_2023() -> Self {
        let mut history = Self::new(2023);
        history.record(Korean, &KOREAN_2023.iter().map(|x| *x as f64).collect::<Vec<f64>>());
        history.record(Math, &MATH_2023.iter().map(|x| *x as f64).collect::<Vec<f64>>());
        history.record(Chemistry, &CHEM_2023.iter().map(|x| *x as f64).collect::<Vec<f64>>());
        history.record(EarthScience, &EARSCI_2023.iter().map(|x| *x as f64).collect::<Vec<f64>>());

        history
    }
}
