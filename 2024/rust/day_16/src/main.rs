use std::{collections::HashMap, fs, usize};
use itertools::Itertools;

fn get_start_position(map: &Vec<Vec<char>>, pos: char) -> (usize, usize) {
    for x in 0..map.len() {
        for y in 0..map[0].len() {
            if map[x][y] == pos {
                return (x,y);
            }
        }
    }
    return (0,0)
}

fn find_path(
    map: &Vec<Vec<char>>,
    mp: &mut usize,
    mut v: Vec<(usize, usize)>,
    mut graph: HashMap<(usize, usize), usize>,
    parents: &mut HashMap<usize, Vec<(usize, usize)>>,
    pos: (usize, usize),
    current: (usize, usize),
    direction: char,
    points: usize) -> HashMap<(usize, usize), usize> {
    
    // if the score can't be lower than highest score
    // return the graph and look at other path.
    if *mp != 0 {
        if *mp < points {
            return graph;
        }
    }
    // 15, 7
    // Check if position exists in graph
    if graph.contains_key(&pos) {
        // Check the points that has previously reached this pos 
        let current_score = graph.get(&pos).copied().unwrap_or(0);
        // If current score is lower, replace the path.
        if current_score >= points {
            graph.insert(current, points);
        }
        else {
            return graph;
        }
    }
    // If this pos has not been reached before, insert the points value
    else {
        graph.insert(current, points);
    }
    // If the position is the end pos
    if map[pos.0][pos.1] == 'E' {
        *mp = points;
        graph.insert(pos, points);
        v.push(pos);
        parents.entry(points).and_modify(|x| x.append(&mut v)).or_insert(v);
        return graph;
    }
    if v.contains(&pos) {
        return graph;
    }

    v.push(pos);
    
    // Check all direction except the one coming from.
    if direction == '>' {

        let directions = [
            // EAST
            ((pos.0, pos.1+1), '>', 1),
            // NORTH
            ((pos.0-1, pos.1), '^', 1001),
            // SOUTH
            ((pos.0+1, pos.1), 'v', 1001),
        ];
        for (p, d, inc) in directions {
            if map[p.0][p.1] == '.' || map[p.0][p.1] == 'E' {
                let point = points + inc;
                graph = find_path(map,mp,v.clone(),graph,parents,(p.0, p.1),pos,d,point);
            }
        }
    }
    else if direction == 'v' {
        
        let directions = [
            // SOUTH
            ((pos.0+1, pos.1), 'v', 1),
            // EAST
            ((pos.0, pos.1+1), '>', 1001),
            // WEST 
            ((pos.0, pos.1-1), '<', 1001),
        ];
        for (p, d, inc) in directions {
            if map[p.0][p.1] == '.' || map[p.0][p.1] == 'E' {
                let point = points + inc;
                graph = find_path(map,mp,v.clone(),graph,parents,(p.0, p.1),pos,d,point);
            }
        }
    }
    else if direction == '<' {
        let directions = [
            // WEST 
            ((pos.0, pos.1-1), '<', 1),
            // NORTH
            ((pos.0-1, pos.1), '^', 1001),
            // SOUTH
            ((pos.0+1, pos.1), 'v', 1001),
        ];
        for (p, d, inc) in directions {
            if map[p.0][p.1] == '.' || map[p.0][p.1] == 'E' {
                let point = points + inc;
                graph = find_path(map,mp,v.clone(),graph,parents,(p.0, p.1),pos,d,point);
            }
        }
    }
    else if direction == '^' {
        let directions = [
            // NORTH
            ((pos.0-1, pos.1), '^', 1),
            // EAST
            ((pos.0, pos.1+1), '>', 1001),
            // WEST 
            ((pos.0, pos.1-1), '<', 1001),
        ];
        for (p, d, inc) in directions {
            if map[p.0][p.1] == '.' || map[p.0][p.1] == 'E' {
                let point = points + inc;
                graph = find_path(map,mp,v.clone(),graph,parents,(p.0, p.1),pos,d,point);
            }
        }
    }
    return graph;
}

fn remove_deadends(map: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut clean = true;
    let mut map = map.clone();
    for x in 1..map.len()-1 {
        for y in 1..map.len()-1 {
            if map[x][y] == 'S' ||
                map[x][y] == 'E'||
                map[x][y] == '#' {
                continue;
            }
            let mut deadend = 0;
            if map[x-1][y] == '#' { deadend += 1 }
            if map[x][y+1] == '#' { deadend += 1 }
            if map[x+1][y] == '#' { deadend += 1 }
            if map[x][y-1] == '#' { deadend += 1 }
            if deadend >= 3 {
                map[x][y] = '#';
                clean = false;
            }
        }
    }
    if !clean {
        return remove_deadends(map);
    }
    return map;
}

fn main() {
    // read input file to string
    let map: Vec<Vec<char>> = fs::read_to_string("puzzle.txt")
        .expect("Did someone remove the file?")
        .lines()
        .map(|x| x.chars().collect())
        .collect();

    let start_pos: (usize, usize) = get_start_position(&map, 'S');
    let end_pos: (usize, usize) = get_start_position(&map, 'E');
    let mut map = remove_deadends(map);
    let mut graph: HashMap<(usize, usize), usize> = HashMap::new();
    let mut parents: HashMap<usize, Vec<(usize, usize)>> = HashMap::new();
    let mut most_points: usize = 0;
    let visited: Vec<(usize, usize)> = Vec::new();
    graph = find_path(
        &map,
        &mut most_points,
        visited,
        graph,
        &mut parents,
        start_pos,
        start_pos,
        '>',
        0);
    let part_one = graph[&end_pos];
    
    let lowest_score: Vec<&usize> = parents.keys().sorted().collect_vec();
    let mut tiles = Vec::new();
    for points in &parents[&lowest_score[0]] {
        if !tiles.contains(points) {
            tiles.push(*points);
        }
    }
    println!("PART ONE: {}", part_one); // 115500
    println!("PART TWO: {}", tiles.len()); // 679
}
