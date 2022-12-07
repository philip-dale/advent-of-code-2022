use std::error::Error;
use crate::input_reader;
use crate::days::day_factory::Day;

pub struct Day07{}

pub struct FileInfo {
    name: String,
    size: usize,
}

impl std::str::FromStr for FileInfo {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let sp: Vec<&str> = s.split_whitespace().collect();
        Ok(Self{
            name: sp[1].to_string(),
            size: sp[0].parse()?,
        })
    }
}

pub struct DirStructure {
    name: String,
    files: Vec<FileInfo>,
    dirs: Vec<DirStructure>,
    size: usize,
}

impl DirStructure {
    pub fn add_file(& mut self, f: FileInfo, path: &[String]) -> bool{
        self.size += f.size;
        if path.len() == 1 && *path.last().unwrap() == self.name {
            self.files.push(f);
            return true
        }
        for ds in 0..self.dirs.len() {
            if self.dirs[ds].name == path[1] {
                self.dirs[ds].add_file(f, &path[1..]);
                return true;
            }
        }
        false
    }

    pub fn add_dir(& mut self, d: &str, path: &[String])  -> bool{
        if path.len() == 1 && *path.last().unwrap() == self.name {
            self.dirs.push(DirStructure{
                name: d.to_string(),
                files: Vec::new(),
                dirs: Vec::new(),
                size: 0,
            });
            return true
        } 
        for ds in 0..self.dirs.len() {
            if self.dirs[ds].name == path[1] {
                self.dirs[ds].add_dir(d, &path[1..]);
                return true;
            }
        }
        false
    }

    pub fn get_dir_size_sum(&self, max: usize) -> usize{
        let mut sum = 0;
        for d in &self.dirs {
            if d.size <= max {
                sum += d.size;
            }
            sum += d.get_dir_size_sum(max);
        }
        sum
    }

    pub fn get_closest_to(&self, target: usize) -> usize {
        let mut found = usize::MAX;
        for d in &self.dirs {
            if d.size > target && d.size < found {
                found = d.size;
            }
            let sub_found = d.get_closest_to(target);
            if sub_found > target && sub_found < found {
                found = sub_found;
            }
        }
        found
    }

    #[allow(dead_code)]
    pub fn print(&self, indent: usize) {
        let indent_text = String::from_utf8(vec![b' '; indent*2]).unwrap();
        
        for f in &self.files {
            println!("{0} - {1} (file, size={2})", indent_text ,f.name, f.size);
        }
        for d in &self.dirs {
            println!("{0} - {1} (dir, size={2})", indent_text , d.name, d.size);
            d.print(indent + 1);
        }
    }
}

impl std::str::FromStr for DirStructure {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let sl: Vec<&str> = s.lines().collect();

        let mut dir_structure = Self {
            name: String::from("/"),
            files: Vec::new(),
            dirs: Vec::new(),
            size: 0,
        };
        
        let mut path :Vec<String> = vec![String::from("/"); 1];

        for l in sl {
            match l.chars().next().unwrap() {
                '$' => {
                    if l.chars().nth(2).unwrap() == 'c' {
                        let ls : Vec<&str> = l.split_whitespace().collect();
                        match ls[2] {
                            ".." => {path.pop();},
                            "/" => {path.resize(1, String::from(""));},
                            _ => {path.push(ls[2].to_string());},
                        }
                    }
                    // ignore ls command
                },
                'd' => {
                    let ls : Vec<&str> = l.split_whitespace().collect();
                    dir_structure.add_dir(ls[1], &path);
                },
                _ => {dir_structure.add_file(l.parse()?, &path);},
            }
        }
        // dir_structure.update_size();
        Ok(dir_structure)
    }   
}

impl Day for Day07 {
    fn run1(&self, ipr: input_reader::InputReader) -> Result<String, Box<dyn Error>> {
        let data: DirStructure = ipr.whole()?;
        Ok(data.get_dir_size_sum(100000).to_string())
    }
    
    fn run2(&self, ipr: input_reader::InputReader) -> Result<String, Box<dyn Error>> {
        let data: DirStructure = ipr.whole()?;
        let del_needed = 30000000 - (70000000 - data.size);
        Ok(data.get_closest_to(del_needed).to_string())
    }
}