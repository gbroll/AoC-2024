use crate::Solution;
use std::collections::{VecDeque, HashMap, HashSet};

pub struct Day16;

type Pos = (i32, i32);

#[derive(Eq, Hash, PartialEq, Clone, Copy, Debug)]
enum Direction {
    North = 0,
    East = 1, 
    South = 2,
    West = 3
}

#[derive(Debug)]
struct QItem {
    pos: Pos,
    dir: Direction,
    cost: u32,
    path: Vec<Pos>
}

fn get_maze_and_start_pos(lines: &Vec<String>) -> (Vec<Vec<char>>, Pos, Pos) {
    let mut start_pos: Pos = (-1, -1);
    let mut end_pos: Pos = (-1, -1);
    let map = lines.iter()
    .enumerate()
    .map(|(x, s)| s.chars()
        .enumerate()
        .map(|(y, c)| {
            if c == 'S' {
                start_pos = (x as i32, y as i32);
            } else if c == 'E' {
                end_pos = (x as i32, y as i32);
            }
            c
        })
    .collect())
    .collect();
    (map, start_pos, end_pos)
}

fn get_90_directions(dir: &Direction) -> (Direction, Direction) {
    match dir {
        Direction::North => (Direction::West, Direction::East),
        Direction::East => (Direction::North, Direction::South),
        Direction::South => (Direction::East, Direction::West),
        Direction::West => (Direction::South, Direction::North)
    }
}

fn get_nbg_tile(maze: &Vec<Vec<char>>, pos:&Pos, dir: &Direction) -> (Pos, char) {
    let next_pos = match dir {
        Direction::North => (pos.0 - 1, pos.1),
        Direction::East => (pos.0, pos.1 + 1),
        Direction::South => (pos.0 + 1, pos.1),
        Direction::West => (pos.0, pos.1 - 1)
    };
    (next_pos, maze[next_pos.0 as usize][next_pos.1 as usize])
}

impl Solution for Day16 {
    type Item = u32;

    fn day(&self) -> u8 {
        return 16;
    }

    fn part1(&self, lines: &Vec<String>) -> Result<Self::Item, &str> { 
        let (maze, start_pos, end_pos) = get_maze_and_start_pos(lines);
        let start_dir = Direction::East;
        let mut q: VecDeque<QItem> = VecDeque::new();
        let mut visited: HashMap<(Pos, Direction), u32> = HashMap::new();
        let mut lowest_cost: Option<u32> = None;
        let mut tiles_on_best_path: HashSet<Pos> = HashSet::new();

        // First position, direction east, cost zero
        q.push_back(QItem{pos: start_pos, dir: start_dir, cost: 0, path: vec![start_pos]});

        while let Some(next_item) = q.pop_front() {
            visited.insert((next_item.pos, next_item.dir), next_item.cost);

            if let Some(lowest) = lowest_cost {
                if next_item.cost > lowest {
                    continue;
                }
            }

            if next_item.pos == end_pos {
                match lowest_cost {
                    None => {
                        lowest_cost = Some(next_item.cost);
                        tiles_on_best_path = next_item.path.iter().cloned().collect();
                    }
                    Some(ref mut cost) if *cost > next_item.cost => {
                        *cost = next_item.cost;
                        tiles_on_best_path =  next_item.path.iter().cloned().collect();
                    },
                    Some(ref mut cost) if *cost == next_item.cost => {
                        *cost = next_item.cost;
                        next_item.path.iter().for_each(|pos| {
                            tiles_on_best_path.insert(*pos);
                        });
                    },
                    _ => ()
                }
            }
            
            let turn_directions = get_90_directions(&next_item.dir);
            let dirs = vec![next_item.dir, turn_directions.0, turn_directions.1];
            let cost = vec![1, 1001, 1001];

            for (i, dir) in dirs.into_iter().enumerate() {
                let (nbg_pos, ch) = get_nbg_tile(&maze, &next_item.pos, &dir);
                if ch == '#' {
                    continue;
                }
                
                if let Some(&prior_cost) = visited.get(&(nbg_pos, dir)) {
                    // We have been here before but maybe this path is better (has lower cost)
                    if (next_item.cost + cost[i]) < prior_cost {
                        let mut path = next_item.path.clone();
                        path.push(nbg_pos);
                        q.push_back(QItem { pos: nbg_pos, dir, cost: next_item.cost + cost[i], path });
                    }
                }
                else {
                    let mut path = next_item.path.clone();
                    path.push(nbg_pos);
                    q.push_back(QItem { pos: nbg_pos, dir, cost: next_item.cost + cost[i], path });
                }
            }            
        }
        println!("{:?}", tiles_on_best_path.len());
        Ok(lowest_cost.unwrap())

    }
    
    fn part2(&self, lines: &Vec<String>) -> Result<Self::Item, &str> { 
        Ok(2)
    }
}
