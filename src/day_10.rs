use crate::Solution;
use std::collections::{HashSet, VecDeque};

pub struct Day10;

fn to_topmap(lines: &Vec<String>) -> (Vec<Vec<u8>>, Vec<(usize, usize)>) {
    let mut trailheads: Vec<(usize, usize)> = Vec::new();
    let topmap = lines
    .iter().enumerate().map(|(row, l)| 
        l.chars().enumerate().map(|(col, c)| {
            let height = c.to_digit(10).expect("Not a digit") as u8;
            if height == 0 {
                trailheads.push((row, col));
            }
            height
        })  
        .collect()
    )
    .collect();
    (topmap, trailheads)
}

fn get_reachable_pos(topmap: &Vec<Vec<u8>>, pos: &(usize, usize, u8)) -> Vec<(usize, usize, u8)> {
    let mut output: Vec<(usize, usize, u8)> = Vec::new();
    let offsets = vec![(0, -1), (0, 1), (-1, 0), (1,0)];
    for offset in offsets {
        let x: i32 = pos.0 as i32 + offset.0;
        let y: i32 = pos.1 as i32 + offset.1;
        if let Some(height) = get_height_at(x, y, topmap)  {
            if height == pos.2 + 1 {
                output.push((x.try_into().unwrap(), y.try_into().unwrap(), height))
            }
        }
    }
    output
}

fn get_height_at(x: i32, y: i32, topmap: &Vec<Vec<u8>>) -> Option<u8> {
    if x < 0 || y < 0 {
        return None;
    }
    if let Some(row) = topmap.get(x as usize) {
        if let Some(&value) = row.get(y as usize) {
            return Some(value);
        }
    }
    None
}

fn calc_trailhead_score(topmap: &Vec<Vec<u8>>, trailheads: &Vec<(usize, usize)>, dfs: bool) -> u32 {
    let mut total_score = 0;
    for trail in trailheads {
        let mut score = 0;
        let mut pos: VecDeque<(usize, usize, u8)> = VecDeque::new();
        let mut visited: HashSet<(usize, usize)> = HashSet::new();
        pos.push_back((trail.0, trail.1, 0));
        while !pos.is_empty() {
            let current_pos: (usize, usize, u8);
            if dfs {
                current_pos = pos.pop_back().unwrap();
            } else {
                current_pos = pos.pop_front().unwrap();
            }

            visited.insert((current_pos.0, current_pos.1));
            if current_pos.2 == 9 {
                score += 1;
            } else {
                let nbs = get_reachable_pos(&topmap, &current_pos);
                for nb in nbs {
                    if !visited.contains(&(nb.0, nb.1)) {
                        pos.push_back(nb);
                    }
                }
            }
        }
        total_score += score;
    }
    total_score
}

impl Solution for Day10 {
    type Item = u32;

    fn day(&self) -> u8 {
        return 10;
    }

    fn part1(&self, lines: &Vec<String>) -> Result<Self::Item, String> { 
        let (topmap, trailheads) = to_topmap(lines);
        let res = calc_trailhead_score(&topmap, &trailheads, true);
        Ok(res)
    }
    
    fn part2(&self, lines: &Vec<String>) -> Result<Self::Item, String> {
        let (topmap, trailheads) = to_topmap(lines);
        let res = calc_trailhead_score(&topmap, &trailheads, false);
        Ok(res)
    }

}
