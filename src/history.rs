use crate::score::{Record, Subject, Subject::*};
use peroxide::fuga::*;
use std::collections::HashMap;

use crate::suneung_data::*;

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

        let cs = cubic_hermite_spline(&xs, &ys, Quadratic).unwrap();

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
                self.eval(subject, student.percentile(subject)).round(),
                student.percentile(subject),
                student.rank(subject),
            );
        }

        record.record(English, 0f64, 0f64, student.rank(English));

        record
    }

    pub fn load(year: usize) -> Result<Self, String> {
        if !(2022..=2025).contains(&year) {
            return Err(format!("Unsupported year: {}", year));
        }

        let mut history = Self::new(year);

        // 연도별 데이터 매핑
        let (korean, math, chem, earth) = match year {
            2025 => (
                KOREAN_2025.to_vec(),
                MATH_2025.to_vec(),
                CHEM_2025.to_vec(),
                EARSCI_2025.to_vec(),
            ),
            2024 => (
                KOREAN_2024.to_vec(),
                MATH_2024.to_vec(),
                CHEM_2024.to_vec(),
                EARSCI_2024.to_vec(),
            ),
            2023 => (
                KOREAN_2023.to_vec(),
                MATH_2023.to_vec(),
                CHEM_2023.to_vec(),
                EARSCI_2023.to_vec(),
            ),
            2022 => (
                KOREAN_2022.to_vec(),
                MATH_2022.to_vec(),
                CHEM_2022.to_vec(),
                EARSCI_2022.to_vec(),
            ),
            _ => unreachable!(),
        };

        // 각 과목별 데이터 기록
        history.record(
            Korean,
            &korean.iter().map(|&x| x as f64).collect::<Vec<f64>>(),
        );
        history.record(Math, &math.iter().map(|&x| x as f64).collect::<Vec<f64>>());
        history.record(
            Chemistry,
            &chem.iter().map(|&x| x as f64).collect::<Vec<f64>>(),
        );
        history.record(
            EarthScience,
            &earth.iter().map(|&x| x as f64).collect::<Vec<f64>>(),
        );

        Ok(history)
    }
}
