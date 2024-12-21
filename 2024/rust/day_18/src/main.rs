use std::{collections::{HashMap, VecDeque}, fs, usize};

fn graph_creation(
    map: &mut Vec<Vec<char>>,
    falling_bytes: &Vec<(usize, usize)>,
    start_bytes: usize,
    stop_bytes: usize) -> HashMap<(usize, usize), VecDeque<(usize,usize)>> {

    // Setting bytes on map.
    for i in start_bytes..stop_bytes {
        let byte = falling_bytes[i];
        map[byte.0+1][byte.1+1] = '#'
    }
    // Creation of the graph
    let mut graph: HashMap<(usize, usize), VecDeque<(usize,usize)>> = HashMap::new();
    for (x, line) in map.clone().into_iter().enumerate() {
        for (y, c) in line.into_iter().enumerate() {
            if c == '.' {
                for pos in vec![(x-1,y), (x,y+1), (x+1,y), (x,y-1)] {
                    if map[pos.0][pos.1] == '.' {
                        graph.entry((x,y))
                            .and_modify(|n: &mut VecDeque<(usize,usize)>| n.push_back(pos))
                            .or_insert(VecDeque::from([pos]));
                    }
                }
            }
        }
    }
    return graph
}

fn bfs(
    start: (usize, usize),
    end: (usize, usize),
    graph: HashMap<(usize, usize), VecDeque<(usize,usize)>>
    ) -> HashMap<(usize, usize), (usize, usize)> {
    let mut visited: Vec<(usize, usize)> = vec![];
    let mut q: VecDeque<(usize, usize)> = VecDeque::from([start]);
    let mut parents: HashMap<(usize, usize), (usize, usize)> = HashMap::new();
    
    while !q.is_empty() {
        // pop first item from queue
        let node = q.pop_front().unwrap();
        // continue if position already is checked
        if !visited.contains(&node) {
            // backtrace path if position is 
            if node == end {
                break;
            } else {
                for neighbor in graph[&node].clone() {
                    if !visited.contains(&neighbor) {
                        q.push_back(neighbor);
                        parents.insert(neighbor, node);
                    }
                }
                visited.push(node)
            }
        }
    }
    return parents
}

fn main() {
    // read input file to string
    let size = 70;
    // let size = 6;
    let start_bytes = 1024;
    // let start_bytes = 12;
    let start = (1,1);
    let end = (size+1,size+1);
    let falling_bytes: Vec<(usize,usize)> = fs::read_to_string("puzzle.txt")
        .expect("Did someone remove the file?")
        .lines()
        .map(|x| 
            (x.split(",").collect::<Vec<&str>>()[1].parse().unwrap(),
            x.split(",").collect::<Vec<&str>>()[0].parse().unwrap()))
        .collect();
    
    // creation of the puzzle map with border
    let mut map: Vec<Vec<char>> = (0..(size+3)).map(
        |x| (0..(size+3)).map(
            |y|
            if x == 0 || x == size+2 {
                '#'
            } else if y == 0 || y == size+2 {
                '#'
            } else {
                '.'
            }
        ).collect()
    ).collect();

    let part_two_map: Vec<Vec<char>> = map.clone();
    let graph: HashMap<(usize, usize), VecDeque<(usize,usize)>> = graph_creation(
        &mut map, &falling_bytes, 0, start_bytes 
    );
    let parents: HashMap<(usize, usize), (usize, usize)> = bfs(start, end, graph);
    let mut path: Vec<(usize, usize)> = Vec::from([end]);
    if parents.contains_key(&end) {
        let mut parent = parents[&end];
        loop {
            path.push(parent);
            if parent == start {
                break;
            } else {
                parent = parents[&parent];
            }
        }
        println!("PART ONE {}", path.len() - 1); // 268
    }

    // Iterate through the rest of the falling bytes..
    for i in start_bytes..falling_bytes.len() {
        // if a falling byte lands on the existing shortest path
        // then check if there is a path to the end position.
        let byte = (falling_bytes[i].0+1, falling_bytes[i].1+1);
        if path.contains(&byte) {
            // create a new graph
            let mut new_map = part_two_map.clone();
            let graph: HashMap<(usize, usize), VecDeque<(usize,usize)>> = graph_creation(
                &mut new_map, &falling_bytes, 0, i + 1
            );
            // run bfs to find the shortest path
            let parents: HashMap<(usize, usize), (usize, usize)> = bfs(start, end, graph);
            // break and print the byte that landed so
            // we can't reach end postion..
            if !parents.contains_key(&end) {
                let stop_byte = (falling_bytes[i].1, falling_bytes[i].0);
                println!("PART TWO: {stop_byte:?}");
                break
            } else {
                path.clear();
                path.push(end);
                new_map[end.0][end.1] = '0';
                let mut parent = parents[&end];
                loop {
                    path.push(parent);
                    new_map[parent.0][parent.1] = '0';
                    if parent == start {
                        break;
                    } else {
                        parent = parents[&parent];
                    }
                }
            }
        }
    }
}
