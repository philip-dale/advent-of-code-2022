use std::collections::HashMap;
use std::error::Error;
use crate::input_reader;
use crate::days::day_factory::Day;
use crate::days::day_factory::types::Point;
pub struct CharAlphaGrid {
    pub cells: Vec<Vec<u32>>,
}

impl std::str::FromStr for CharAlphaGrid {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(CharAlphaGrid {
            cells: {
                let mut v: Vec<Vec<u32>> = Vec::new();
                for l in s.lines() {
                    v.push(Vec::new());
                    let last = v.len()-1;
                    for c in l.chars() {
                        v[last].push(u32::from(c));
                    }
                }
                v
            },
        })
    }
}

struct PathItem {
    pub height: u32,
    pub score: u64,
    pub visited: bool,
}

impl PathItem {
    pub fn new(height: u32) -> Self {
        Self {
            height,
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
    current_point
}

// use dijkstra to find the lowest energy path.
fn scan_path_up(search_path:& mut Vec<Vec<PathItem>>, start: &Point, end: &Point) -> u64{
    // Store points that we have calculated a score for but not visited. We will use this to decide which node to go to next.
    let mut to_visit:HashMap<Point, u64> = HashMap::new();

    let x_max = search_path.len()-1;
    let y_max = search_path[0].len()-1;
    let target = Point{x: end.x, y: end.y};
    let mut current = Point{x:start.x, y:start.y};

    search_path[current.x][current.y].score = 0;
    while !search_path[target.x][target.y].visited {
        // Calculate score to neighbours
        for n in current.get_adjacent_neighbours(x_max, y_max) {
            if search_path[n.x][n.y].height > search_path[current.x][current.y].height + 1 {
                continue;
            }
            if !search_path[n.x][n.y].visited {
                let new_score = search_path[current.x][current.y].score + 1;

                if new_score < search_path[n.x][n.y].score {
                    search_path[n.x][n.y].score  = new_score;
                    to_visit.insert(n.as_point(), new_score);
                }
            }
        }
        search_path[current.x][current.y].visited = true;
        to_visit.remove(&current);

        // find lowest scoring not visited cell
        if !to_visit.is_empty() {
            current = find_next_not_visited(&to_visit);
        } else {
            break;
        }
    }
    search_path[target.x][target.y].score
}

fn scan_path_down_all(search_path:& mut Vec<Vec<PathItem>>, start: &Point) -> u64{
    // Store points that we have calculated a score for but not visited. We will use this to decide which node to go to next.
    let mut to_visit:HashMap<Point, u64> = HashMap::new();

    let x_max = search_path.len()-1;
    let y_max = search_path[0].len()-1;
    let mut current = Point{x:start.x, y:start.y};

    search_path[current.x][current.y].score = 0;
    to_visit.insert(current.as_point(), 0);
    while !to_visit.is_empty() {
        // Calculate score to neighbours
        for n in current.get_adjacent_neighbours(x_max, y_max) {
            if search_path[n.x][n.y].height + 1 < search_path[current.x][current.y].height {
                continue;
            }
            if !search_path[n.x][n.y].visited {
                let new_score = search_path[current.x][current.y].score + 1;

                if new_score < search_path[n.x][n.y].score {
                    search_path[n.x][n.y].score  = new_score;
                    to_visit.insert(n.as_point(), new_score);
                }
            }
        }
        search_path[current.x][current.y].visited = true;
        to_visit.remove(&current);

        // find lowest scoring not visited cell
        if !to_visit.is_empty() {
            current = find_next_not_visited(&to_visit);
        } else {
            break;
        }
    }

    let mut a_points = Vec::new();
    for x in search_path {
        for y in x {
            if y.height == u32::from('a') {
                a_points.push(y.score);
            }
        }
    }
    a_points.sort();
    a_points[0]
}
pub struct Day12{}

impl Day for Day12 {
    fn run1(&self, ipr: input_reader::InputReader) -> Result<String, Box<dyn Error>> {
        let data: CharAlphaGrid = ipr.whole()?;
        let mut search_path: Vec<Vec<PathItem>> = Vec::new();
        let mut start = Point{x:0, y:0};
        let mut end = Point{x:0, y:0};

        for (xpos, x) in data.cells.iter().enumerate() {
            let mut search_line: Vec<PathItem> = Vec::new();
            for (ypos, y) in x.iter().enumerate() {
                if *y == u32::from('S') {
                    start.x = xpos;
                    start.y = ypos;
                    search_line.push(PathItem::new(u32::from('a')));
                } else if *y == u32::from('E') {
                    end.x = xpos;
                    end.y = ypos;
                    search_line.push(PathItem::new(u32::from('z')));
                } else {
                    search_line.push(PathItem::new(*y));
                }
            }
            search_path.push(search_line);
        }
        let score = scan_path_up(& mut search_path, &start, &end);
        Ok(score.to_string())
    }
    
    fn run2(&self, ipr: input_reader::InputReader) -> Result<String, Box<dyn Error>> {
        let data: CharAlphaGrid = ipr.whole()?;
        let mut search_path: Vec<Vec<PathItem>> = Vec::new();
        let mut start = Point{x:0, y:0};

        for (xpos, x) in data.cells.iter().enumerate() {
            let mut search_line: Vec<PathItem> = Vec::new();
            for (ypos, y) in x.iter().enumerate() {
                if *y == u32::from('S') {
                    search_line.push(PathItem::new(u32::from('a')));
                } else if *y == u32::from('E') {
                    start.x = xpos;
                    start.y = ypos;
                    search_line.push(PathItem::new(u32::from('z')));
                } else {
                    search_line.push(PathItem::new(*y));
                }
                
            }

            search_path.push(search_line);
        }
        
        let score = scan_path_down_all(& mut search_path, &start);
        Ok(score.to_string())
    }
}