pub struct StrU64{
    pub dir: String,
    pub val: u64,
}

impl std::str::FromStr for StrU64 {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let a:Vec<&str> = s.split_whitespace().collect();
        return Ok( StrU64{
            dir: a[0].to_string(),
            val: a[1].trim().parse()?,    
        })
    }
}
#[derive(Clone)]
pub struct Bits {
    pub bits: Vec<char>,
}

impl Bits {
    pub fn to_uint(&self) -> u64 {
        let mut val: u64 = 0; 
        for c in &self.bits {
            if *c == '1' {
                val += 1;
            }
            val = val << 1;
        }
        val = val >> 1;
        return val;
    }
}

impl std::str::FromStr for Bits {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut b: Vec<char> = Vec::new();
        for c in s.chars() {
            b.push(c);
        }
        return Ok(Bits{bits: b});
    }
}

#[derive(Clone)]
pub struct HighLowCounts{
    pub low: u64,
    pub high: u64,
}
