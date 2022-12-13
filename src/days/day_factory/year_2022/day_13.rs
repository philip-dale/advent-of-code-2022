use crate::days::day_factory::types::DOUBLE_NEW_LINE;
use crate::days::day_factory::Day;
use crate::input_reader;
use std::cmp::Ordering;
use std::error::Error;

use serde_json;

trait MsgComp {
    fn compare(&self, r: &Self) -> Ordering;
    fn to_array(&self) -> Self;
}

impl MsgComp for serde_json::Value {
    fn to_array(&self) -> Self {
        Self::Array(vec![Self::from(self.as_u64().unwrap())])
    }

    fn compare(&self, r: &Self) -> Ordering {
        if self.is_number() && r.is_number() {
            return self.as_u64().unwrap().cmp(&r.as_u64().unwrap());
        }

        if !self.is_number() && !r.is_number() {
            let mut l_pos = 0;
            let mut r_pos = 0;
            while l_pos < self.as_array().unwrap().len() && r_pos < r.as_array().unwrap().len() {
                let dif = self.as_array().unwrap()[l_pos].compare(&r.as_array().unwrap()[r_pos]);
                if dif.is_ne() {
                    return dif;
                }
                l_pos += 1;
                r_pos += 1;
            }
            return self
                .as_array()
                .unwrap()
                .len()
                .cmp(&r.as_array().unwrap().len());
        }

        if self.is_number() {
            let v = self.to_array();
            return v.compare(r);
        }

        let v = r.to_array();
        self.compare(&v)
    }
}

#[derive(Debug)]
struct CodeVec {
    v: Vec<serde_json::Value>,
}

impl std::str::FromStr for CodeVec {
    type Err = serde_json::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(CodeVec {
            v: {
                let mut v = Vec::new();
                for l in s.lines().collect::<Vec<&str>>() {
                    if !l.is_empty() {
                        v.push(l.parse()?);
                    }
                }
                v
            },
        })
    }
}
pub struct Day13 {}

impl Day for Day13 {
    fn run1(&self, ipr: input_reader::InputReader) -> Result<String, Box<dyn Error>> {
        let mut val = 0;
        let code_pairs: Vec<String> = ipr.vec_1d_sep(&DOUBLE_NEW_LINE.to_string())?;
        for (i, cp) in code_pairs.iter().enumerate() {
            let code_vec: CodeVec = cp.parse()?;
            if code_vec.v[0].compare(&code_vec.v[1]).is_lt() {
                val += i + 1;
            }
        }
        Ok(val.to_string())
    }

    fn run2(&self, ipr: input_reader::InputReader) -> Result<String, Box<dyn Error>> {
        let mut codes: CodeVec = ipr.whole()?;
        let v2: serde_json::Value = "[[2]]".parse()?;
        let v6: serde_json::Value = "[[6]]".parse()?;

        codes.v.push(v2.clone());
        codes.v.push(v6.clone());

        codes.v.sort_by(|a, b| a.compare(b));

        let pos2 = codes.v.iter().position(|a| a.compare(&v2).is_eq()).unwrap() + 1;
        let pos6 = codes.v.iter().position(|a| a.compare(&v6).is_eq()).unwrap() + 1;

        Ok((pos2 * pos6).to_string())
    }
}
