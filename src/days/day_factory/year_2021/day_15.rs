use std::collections::HashMap;
use std::error::Error;
use crate::input_reader;
use crate::days::day_factory::Day;
use crate::days::day_factory::types::{CharNumGrid, Point};

struct PathItem {
    pub weight: u32,
    pub score: u64,
    pub visited: bool,
}

impl PathItem {
    pub fn new(weight: u32) -> Self {
        return Self {
            weight,
            score: u64::MAX,
            visited: false,
        }
    }
}
fn find_next_not_visited(to_visit:&HashMap<Point, u64>) -> Point {
    let mut current_score = u64::MAX;
    let mut current_point = Point{x:usize::MAX,y:usize::MAX};

    for (p, v) in to_visit {
        if *v < current_score {
            current_score = *v;
            current_point = p.as_point();
        }
    }
    return current_point;
}

// use dijkstra to find the lowest energy path.
fn scan_path(search_path:& mut Vec<Vec<PathItem>>) -> u64{

    // Store points that we have calculated a score for but not visited. We will use this to decide which node to go to next.
    let mut to_visit:HashMap<Point, u64> = HashMap::new();

    let x_max = search_path.len()-1;
    let y_max = search_path[0].len()-1;
    let target = Point{x: search_path.len()-1, y: search_path[0].len()-1};
    let mut current = Point{x:0, y:0};

    search_path[current.x][current.y].score = 0;
    while search_path[target.x][target.y].visited == false {
        // Calculate score to neighbours
        for n in current.get_adjacent_neighbours(x_max, y_max) {
            if !search_path[n.x][n.y].visited {
                let new_score = search_path[current.x][current.y].score + u64::from(search_path[n.x][n.y].weight);

                if new_score < search_path[n.x][n.y].score {
                    search_path[n.x][n.y].score  = new_score;
                    to_visit.insert(n.as_point(), new_score);
                }
            }
        }
        search_path[current.x][current.y].visited = true;
        to_visit.remove(&current);

        // find lowest scoring not visited cell
        if to_visit.len() > 0 {
            current = find_next_not_visited(&to_visit);
        } else {
            break;
        }
    }
    return search_path[target.x][target.y].score;
}

fn expand_grid(input: CharNumGrid) -> CharNumGrid {

    let mut output = CharNumGrid {
        cells: vec![vec![0;input.cells[0].len()*5]; input.cells.len()*5],
    };

    for (x, xv) in output.cells.to_vec().iter().enumerate() {
        for (y, _yv) in xv.iter().enumerate() {
            let x_source = x % input.cells.len();
            let y_source = y % input.cells[0].len();
            let x_block = x / input.cells.len();
            let y_block = y / input.cells.len();

            let mut val = input.cells[x_source][y_source] + u32::try_from(x_block).unwrap() + u32::try_from(y_block).unwrap();
            while val > 9 {
                val -= 9;
            }
            output.cells[x][y] = val;
        }
    }

    return output;
}

pub struct Day15{}

impl Day for Day15 {
    fn run1(&self, ipr: input_reader::InputReader) -> Result<String, Box<dyn Error>> {
        let data: CharNumGrid = ipr.whole()?;
        let mut search_path: Vec<Vec<PathItem>> = Vec::new();
        for x in data.cells {
            let mut search_line: Vec<PathItem> = Vec::new();
            for y in x {
                search_line.push(PathItem::new(y));
            }
            search_path.push(search_line);
        }
        let score = scan_path(& mut search_path);

        // for x in search_path {
        //     for y in x {
        //         print!("{:0>2},", y.score);
        //     }
        //     println!("");
        // }

        return Ok(score.to_string());
    }
    
    fn run2(&self, ipr: input_reader::InputReader) -> Result<String, Box<dyn Error>> {
        let data: CharNumGrid = ipr.whole()?;
        let data = expand_grid(data);
        let mut search_path: Vec<Vec<PathItem>> = Vec::new();
        for x in data.cells {
            let mut search_line: Vec<PathItem> = Vec::new();
            for y in x {
                search_line.push(PathItem::new(y));
            }
            search_path.push(search_line);
        }
        let score = scan_path(& mut search_path);

        // for x in search_path {
        //     for y in x {
        //         print!("{:0>2},", y.score);
        //     }
        //     println!("");
        // }

        return Ok(score.to_string());
    }
}