use std::fs::{File, canonicalize, read_to_string};
use std::io::{BufReader, BufRead, Error};

pub struct InputReader {
    pub filename: String,
    pub directory: String,
    pub stage: String,
}

impl InputReader {
    pub fn vec_1d_newln<T: std::str::FromStr>(&self) -> Result<Vec<T>, Error> where <T as std::str::FromStr>::Err: std::fmt::Debug
    {
        let br = BufReader::new(File::open(self.fullname()?)?);
        let mut v = vec![];
        for line in br.lines() {
            v.push(line?.trim().parse().expect("Type not found"));
        }
        Ok(v)
    }

    pub fn vec_1d_sep<T: std::str::FromStr>(&self, sep: &String) -> Result<Vec<T>, Error> where <T as std::str::FromStr>::Err: std::fmt::Debug
    {
        let s = read_to_string(self.fullname()?).unwrap();
        let mut v = vec![];
        for line in s.split(sep) {
            v.push(line.trim().parse().expect("Type not found"));
        }
        Ok(v)
    }

    pub fn whole<T: std::str::FromStr>(&self) -> Result<T, Error> where <T as std::str::FromStr>::Err: std::fmt::Debug
    {
        Ok(read_to_string(self.fullname()?).unwrap().parse().expect("Error Reading Whole File"))
    }

    pub fn fullname(&self) -> Result<String, Error> {
        Ok(canonicalize(self.directory.clone() + "/" + &self.filename[..])?.into_os_string().into_string().unwrap())
    }

}