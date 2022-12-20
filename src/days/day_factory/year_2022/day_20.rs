use crate::days::day_factory::Day;
use crate::input_reader;
use std::error::Error;

struct LinkNode {
    val: i64,
    index: usize,
    left: usize,
    right: usize,
}

impl LinkNode {
    pub fn new(val: i64, index: usize, max: usize) -> Self {
        Self {
            val,
            index,
            right: (index + 1) % max,
            left: {
                let pos = (index as i64 - 1) % max as i64;
                if pos < 0 {
                    (max as i64 + pos) as usize
                } else {
                    pos as usize
                }
            },
        }
    }
}
struct LinkNodes {
    nodes: Vec<LinkNode>,
    zero_index: usize,
}

impl LinkNodes {
    fn remove(&mut self, n: usize) {
        let r = self.nodes[n].right;
        let l = self.nodes[n].left;
        self.nodes[r].left = self.nodes[n].left;
        self.nodes[l].right = self.nodes[n].right;
    }

    fn move_node_right_of(&mut self, n_source: usize, d_index: usize) {
        self.remove(n_source);

        let index = self.nodes[d_index].index;
        let right = self.nodes[d_index].right;

        self.nodes[right].left = self.nodes[n_source].index;
        self.nodes[n_source].right = right;
        self.nodes[index].right = self.nodes[n_source].index;
        self.nodes[n_source].left = index;
    }

    pub fn mix(&mut self) {
        for ni in 0..self.nodes.len() {
            let mut move_to_index = ni;

            if self.nodes[move_to_index].val.abs() % (self.nodes.len() as i64 - 1) == 0 {
                continue;
            }

            match self.nodes[move_to_index].val {
                val if val > 0 => {
                    for _i in 0..self.nodes[move_to_index].val % (self.nodes.len() as i64 - 1) {
                        move_to_index = self.nodes[move_to_index].right;
                    }
                }
                _ => {
                    // Note plus one so we can use move_node_right_of()
                    for _i in
                        0..(self.nodes[move_to_index].val.abs() + 1) % (self.nodes.len() as i64 - 1)
                    {
                        move_to_index = self.nodes[move_to_index].left;
                    }
                }
            }
            self.move_node_right_of(ni, move_to_index);
        }
    }

    pub fn multiply(&mut self, val: i64) {
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
        let mut zero_index: usize = 0;
        Ok(Self {
            nodes: {
                let mut n = Vec::new();

                for l in &ls {
                    if l == &"0" {
                        zero_index = n.len();
                    }
                    n.push(LinkNode::new(l.parse()?, n.len(), ls.len()));
                }
                n
            },
            zero_index: { zero_index },
        })
    }
}

pub struct Day20 {}

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
