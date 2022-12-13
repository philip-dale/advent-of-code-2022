use std::error::Error;
use std::cmp::Ordering;
use crate::input_reader;
use crate::days::day_factory::Day;
use crate::days::day_factory::types::DOUBLE_NEW_LINE;

#[derive(Debug,PartialEq, PartialOrd, Clone)]
enum CodeMap {
    Val(u32),
    Array(Vec<CodeMap>),
}

impl CodeMap {

    pub fn to_codemap(&self) -> Self {
        match self {
            Self::Val(v) => Self::Val(*v),
            Self::Array(v) => Self::Array(v.to_vec()),
        }
    }

    pub fn is_val(&self) -> bool{
        matches!(self, Self::Val(_v))
    }

    pub fn get_val(&self) -> Option<u32> {
        match self {
            Self::Val(v) => Some(*v),
            _ => None,
        }
    }

    pub fn get_vec(&self) -> Option<&Vec<Self>> {
        match self {
            Self::Array(v) => Some(v),
            _ => None,
        }
    }

    fn compare(& self, r:&CodeMap) -> Ordering {
        if self.is_val() && r.is_val() {
            return self.get_val().unwrap().cmp(&r.get_val().unwrap());
        }
    
        if !self.is_val() && !r.is_val() {
            let mut l_pos = 0;
            let mut r_pos = 0;
            while l_pos < self.get_vec().unwrap().len() && r_pos < r.get_vec().unwrap().len() {
                let dif = self.get_vec().unwrap()[l_pos].compare(&r.get_vec().unwrap()[r_pos]);
                if dif.is_ne(){
                    return dif;
                }
                l_pos += 1;
                r_pos += 1;
            }
            return self.get_vec().unwrap().len().cmp(&r.get_vec().unwrap().len());
        }
    
        if self.is_val() {
            let v = CodeMap::Array(vec![CodeMap::Val(self.get_val().unwrap())]);
            return v.compare(r);
        }
    
        let v = CodeMap::Array(vec![CodeMap::Val(r.get_val().unwrap())]);
        self.compare(&v)

    }

    fn parse_string(s: &[char]) -> CodeMap {
        let mut v: Vec<CodeMap> = Vec::new();
        if s.is_empty() {
            return CodeMap::Array(v);
        }

        let mut pos = 0;

        while pos < s.len() {
            if s[pos] == '[' {
                let mut brace_count = 1;
                let mut array_end_index = pos;
                while brace_count > 0 {
                    array_end_index += 1;
                    match s[array_end_index] {
                        '[' => brace_count += 1,
                        ']' => brace_count -= 1,
                        _ => (),
                    }
                }
                v.push(CodeMap::parse_string(&s[pos+1..array_end_index]));
                pos = array_end_index + 1;
            } else if s[pos] == ',' {
                pos += 1;
            } else {
                let mut val_end_index = pos;
                while val_end_index < s.len() && s[val_end_index] != ',' {
                    val_end_index += 1;
                }
                v.push(CodeMap::Val(String::from_iter(&s[pos..val_end_index]).parse().unwrap()));
                pos = val_end_index + 1;
            }
        }

        CodeMap::Array(v)
    }

}

impl std::str::FromStr for CodeMap {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let chars: Vec<char> = s.chars().collect();
        Ok(CodeMap::parse_string(&chars[1..chars.len()-1]))
    }
}
struct CodeVec {
    v: Vec<CodeMap>,
}

impl std::str::FromStr for CodeVec {
    type Err = std::num::ParseIntError;

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
            }
        })
    }
}
pub struct Day13{}

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
        let v2: CodeMap = "[[2]]".parse()?;
        let v6: CodeMap = "[[6]]".parse()?;

        codes.v.push(v2.to_codemap());
        codes.v.push(v6.to_codemap());

        codes.v.sort_by(|a, b| a.compare(b));

        let pos2 = codes.v.iter().position(|a| a.compare(&v2).is_eq()).unwrap() + 1;
        let pos6 = codes.v.iter().position(|a| a.compare(&v6).is_eq()).unwrap() + 1;

        Ok((pos2*pos6).to_string())
    }
}