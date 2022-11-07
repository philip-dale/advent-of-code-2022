use std::fs::{File, canonicalize};
use std::io::{BufReader, BufRead, Error};

pub struct InputReader {
    pub filename: String,
    pub directory: String,
    pub stage: String,
}

impl InputReader {
    pub fn vec_1d<T: std::str::FromStr>(&self) -> Result<Vec<T>, Error> where <T as std::str::FromStr>::Err: std::fmt::Debug
    {
        let br = BufReader::new(File::open(self.fullname()?)?);
        let mut v = vec![];
        for line in br.lines() {
            v.push(line?.trim().parse().expect("u64 not found"));
        }
        return Ok(v);
    }

    pub fn fullname(&self) -> Result<String, Error> {
        return Ok(canonicalize(self.directory.clone() + "/" + &self.filename[..])?.into_os_string().into_string().unwrap());
    }

}