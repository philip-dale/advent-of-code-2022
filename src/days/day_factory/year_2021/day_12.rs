use std::error::Error;
use crate::input_reader;
use crate::days::day_factory::Day;

use std::collections::HashMap;

struct NodeLink {
    a: String,
    b: String,
}

impl std::str::FromStr for NodeLink {
    type Err = std::num::ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let nodes:Vec<&str> = s.split("-").collect();
        Ok(NodeLink {
            a: nodes[0].to_string(),
            b: nodes[1].to_string(),
        })
    }
}

struct NodeInfo {
    pub is_small: bool,
    pub links: Vec<String>,
    pub visited: bool,
    pub visted_once: bool,
}

impl NodeInfo {
    pub fn new(name: String) -> NodeInfo {
        NodeInfo {
            is_small: name.chars().next().unwrap().is_lowercase(),
            links: Vec::new(),
            visited: false,
            visted_once: false,
        }
    }
}

struct CaveSystem {
    nodes: HashMap<String, NodeInfo>,
    twice_visit: String,
}

impl CaveSystem {
    pub fn new(nodes: Vec<NodeLink>) -> CaveSystem {
        let mut cs = CaveSystem { 
            nodes: HashMap::new(),
            twice_visit: String::from(""),
        };

        for n in nodes {
            let nodea = cs.nodes.entry(n.a.to_string()).or_insert(NodeInfo::new(n.a.to_string()));
            nodea.links.push(n.b.to_string());

            let nodeb = cs.nodes.entry(n.b.to_string()).or_insert(NodeInfo::new(n.b));
            nodeb.links.push(n.a);
        }

        return cs;
    }

    fn walk(& mut self, name: & String, path: & mut Vec<String>, paths: & mut usize, visit_twice_mode: bool) {
        let mut current_node = self.nodes.get_mut(name).unwrap();
    
        if current_node.is_small && current_node.visited{
            return;
        }

        path.push(name.to_string());

        if visit_twice_mode && current_node.is_small{
            if (*current_node).visted_once {
                if self.twice_visit.len() == 0 {
                    self.twice_visit = name.to_string();
                    (*current_node).visited = true;
                } else {
                    return;
                }
            } else {
                (*current_node).visted_once = true;
            }
            
        } else {
            (*current_node).visited = true;
        }

        

        for link in &current_node.links.to_vec() {
            
            if link == &String::from("end") {
                // path.push(link.to_string());
                // for p in path.into_iter() {
                //     print!("{},", p);
                // }
                // println!("");
                // path.pop();
                *paths += 1;

            } else {
                self.walk(&link, path, paths, visit_twice_mode);
            }
        }
        path.pop();

        if visit_twice_mode {
            if self.twice_visit == *name {
                self.twice_visit = String::from("");
                self.nodes.get_mut(name).unwrap().visited = false;
            } else {
                self.nodes.get_mut(name).unwrap().visted_once = false;
            }
        } else {
            self.nodes.get_mut(name).unwrap().visited = false;
        }
        
    }
    
    pub fn walk_start(& mut self, visit_twice: bool) -> usize{
    
        let mut path: Vec<String> = vec![String::from("start")];
        let mut paths: usize = 0;
    
        let mut start_node = self.nodes.get_mut(&String::from("start")).unwrap();
        (*start_node).visited = true;

        for link in &start_node.links.to_vec() {
            self.walk(&link, & mut path, & mut paths, visit_twice);
        }
        return paths;
    }
}



pub struct Day12{}

impl Day for Day12 {
    fn run1(&self, ipr: input_reader::InputReader) -> Result<String, Box<dyn Error>> {
        let data: Vec<NodeLink> = ipr.vec_1d_newln()?;
        let mut cs = CaveSystem::new(data);
        let paths = cs.walk_start(false);
        return Ok(paths.to_string());
    }
    
    fn run2(&self, ipr: input_reader::InputReader) -> Result<String, Box<dyn Error>> {
        let data: Vec<NodeLink> = ipr.vec_1d_newln()?;
        let mut cs = CaveSystem::new(data);
        let paths = cs.walk_start(true);
        return Ok(paths.to_string());
    }
}