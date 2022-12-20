use std::error::Error;
use crate::input_reader;
use crate::days::day_factory::Day;

#[derive(Clone, PartialEq, Eq)]
struct LinkNode {
    val: i64,
    index: usize,
    left: usize,
    right: usize,
}

impl LinkNode {
    #[allow(dead_code)]
    pub fn print(&self) {
        print!("{{val = {}, index = {}, left = {}, right = {}}}, ", self.val, self.index, self.left, self.right);
    }
}

struct LinkNodes {
    nodes: Vec<LinkNode>,
    zero_index: usize,
}

impl LinkNodes {
    fn remove(& mut self, n: usize) {
        let r = self.nodes[n].right;
        let l = self.nodes[n].left;
        self.nodes[r].left = self.nodes[n].left;
        self.nodes[l].right = self.nodes[n].right;
    }

    pub fn mix(& mut self) {
        for ni in 0..self.nodes.len() {

            let mut move_to = self.nodes[ni].clone();

            match move_to.val {
                0 => continue,
                val if val > 0 => {
                    for _i in 0..move_to.val % (self.nodes.len() as i64 -1){
                        move_to = self.nodes[move_to.right].clone();
                    }
                    if move_to == self.nodes[ni] {
                        continue;
                    }
                    self.remove(ni);

                    self.nodes[move_to.right].left = self.nodes[ni].index;
                    self.nodes[ni].right = move_to.right;
                    self.nodes[move_to.index].right = self.nodes[ni].index;
                    self.nodes[ni].left = move_to.index;
                },
                _ => {
                    for _i in 0..move_to.val.abs() % (self.nodes.len() as i64 -1){
                        move_to = self.nodes[move_to.left].clone();
                    }
                    if move_to == self.nodes[ni] {
                        continue;
                    }

                    self.remove(ni);

                    self.nodes[move_to.left].right = self.nodes[ni].index;
                    self.nodes[ni].left = move_to.left;
                    self.nodes[move_to.index].left = self.nodes[ni].index;
                    self.nodes[ni].right = move_to.index;
                }
            }
        }
    }

    pub fn multiply(& mut self, val: i64) {
        for i in 0..self.nodes.len() {
            self.nodes[i].val *= val;
        }
    }

    pub fn result(&self) -> i64 {
        let mut result = 0;
        let mut current = self.zero_index;

        for _i in 0..3 {
            for _m in 0..1000 {
                current = self.nodes[current].right;
            }
            println!("{} = {}", current, self.nodes[current].val);
            result += self.nodes[current].val;
        }

        result
    }
}

impl std::str::FromStr for LinkNodes {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ls: Vec<&str> = s.lines().collect();
        let mut zero_index:usize = 0;
        Ok(Self { 
            nodes: {
                let mut n = Vec::new();
                
                for l in &ls {
                    if l == &"0" {
                        zero_index = n.len();
                    }

                    n.push(LinkNode{
                        val: l.parse()?,
                        index: n.len(),
                        right: (n.len() + 1) % ls.len(),
                        left: {
                            let pos = (n.len() as i64 - 1) % ls.len() as i64;
                            if pos < 0 {
                                (ls.len() as i64 + pos) as usize
                            } else {
                                pos as usize
                            }

                        },
                    });
                }
                n
            },
            zero_index : {
                zero_index
            },
        })
    }
}

pub struct Day20{}

impl Day for Day20 {
    fn run1(&self, ipr: input_reader::InputReader) -> Result<String, Box<dyn Error>> {
        let mut nodes: LinkNodes = ipr.whole()?;
        nodes.mix();
        Ok(nodes.result().to_string())
    }
    
    fn run2(&self, ipr: input_reader::InputReader) -> Result<String, Box<dyn Error>> {
        let mut nodes: LinkNodes = ipr.whole()?;
        nodes.multiply(811589153);
        for _i in 0..10 {
            nodes.mix();
        }
        Ok(nodes.result().to_string())
    }
}