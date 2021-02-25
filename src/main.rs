use std::io::prelude::*;
use std::io;
use std::collections::{HashMap, HashSet};


fn main() -> io::Result<()> {
    let input = read_input()?;

    let mut solver = Solver::from_str(&input);
    // println!("{:?}", solver);
    let solution = solver.solve();

    write_output(solution);
    return Ok(());
}

#[derive(Debug)]
struct Solver {
    max_duration: usize,
    bonus_points: usize,
    graph: HashMap<usize, HashSet<String>>,
    streets: HashMap<String, Street>,
    paths: HashSet<Path>,
}


impl Solver {
    fn from_str(input: &str) -> Self {
        let lines = input
            .trim()
            .split('\n')
            .collect::<Vec<_>>();
        
        let header = lines[0].split_whitespace().map(|v| v.parse::<usize>().unwrap()).collect::<Vec<_>>();
        let (d, _, s, v, f) = (header[0], header[1], header[2], header[3], header[4]);

        let mut streets = HashMap::new();
        for k in 1..1 + s {
            let street = Street::from_str(lines[k]);
            streets.insert(street.name.clone(), street);
        } 

        let mut graph = HashMap::new();
        for street in streets.values() {
            graph.entry(street.to).or_insert_with(HashSet::new).insert(street.name.clone());
        }

        let mut paths = HashSet::new();
        for k in (1 + s)..(1 + s + v) {
            let path = Path::from_str(lines[k]);
            paths.insert(path);
        }

        return Self::new(d, f, streets, graph, paths);
    }

    fn new(
        max_duration: usize, 
        bonus_points: usize, 
        streets: HashMap<String, Street>,
        graph: HashMap<usize, HashSet<String>>, 
        paths: HashSet<Path>
    ) -> Self {
        Self { 
            max_duration: max_duration,
            bonus_points: bonus_points,
            streets: streets,
            graph: graph,
            paths: paths,
        }
    }

    fn solve(&mut self) -> Solution {
        let mut solution = Solution::new();


        for path in &self.paths {
            for street in &path.streets {
                self.streets.get_mut(street).unwrap().visits += 1;
            }

        }

        for (intersection_id, streets) in &self.graph {
            let mut streets: Vec<_> = streets
                .iter()
                .clone()
                .filter_map(|name| {
                    let street = self.streets.get(name).unwrap();
                    if street.visits == 0 {
                        return None;
                    }
                    return Some(street);
                })
                .collect();

            if streets.is_empty() {
                continue;
            }

            streets.sort_unstable_by_key(|street| street.visits);

            let incoming: Vec<_> = streets
                .into_iter()
                .enumerate()
                .map(|(i, street)| (street.name.clone(), i + 1))
                .collect();

            let intersection = Intersection::new(*intersection_id, incoming);
            solution.insert(intersection);
        }


        return solution;
    }
}

#[derive(Debug)]
struct Street {
    from: usize,
    to: usize,
    name: String,
    transit: usize,
    visits: usize,
}

impl Street {
    fn from_str(raw: &str) -> Self {
        let values: Vec<_> = raw.trim().split_whitespace().collect();
        let from = values[0].parse::<usize>().unwrap();
        let to = values[1].parse::<usize>().unwrap();
        let name = String::from(values[2]);
        let transit = values[3].parse::<usize>().unwrap();

        return Self::new(from, to, name, transit);
    }

    fn new(from: usize, to: usize, name: String, transit: usize) -> Self {
        Self { from: from, to: to, name: name, transit: transit, visits: 0}
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Path {
    streets: Vec<String>,
}

impl Path {

    fn from_str(raw: &str) -> Self {
        let values: Vec<String> = raw.trim().split_whitespace().skip(1).map(String::from).collect();
        return Self::new(values);
    }

    fn new(streets: Vec<String>) -> Self {
        Self { streets: streets }
    }
}

#[derive(Debug)]
struct Solution {
    intersections: HashSet<Intersection>,
}


impl Solution {
    fn new() -> Self {
        Self { intersections: HashSet::new() }
    }

    fn insert(&mut self, intersection: Intersection) {
        self.intersections.insert(intersection);
    }


    fn to_string(&self) -> String {
        let mut ans = String::new();
        ans.push_str(&self.intersections.len().to_string());
        for intersection in &self.intersections {
            ans.push('\n');
            ans.push_str(&intersection.to_string());
        }        
        return ans;
    }
}



#[derive(Debug, PartialEq, Eq, Hash)]
struct Intersection {
    id: usize,
    incoming: Vec<(String, usize)> 
}

impl Intersection {
    fn new(id: usize, incoming: Vec<(String, usize)>) -> Self {
        Self { id: id, incoming: incoming }
    }

    fn to_string(&self) -> String {
        let mut ans = String::new();
        ans.push_str(&format!("{}\n", self.id));
        ans.push_str(&self.incoming.len().to_string());
        for item in &self.incoming {
            ans.push('\n');
            ans.push_str(&format!("{} {}", item.0, item.1));
        }

        return ans;
    }
}

fn read_input() -> io::Result<String> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    return Ok(buffer);
}

fn write_output(solution: Solution) {
    println!("{}", solution.to_string());
}
