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