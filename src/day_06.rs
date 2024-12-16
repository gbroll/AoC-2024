use crate::Solution;
pub struct Day06;

#[derive(Debug, Copy, Clone)]
struct PosInfo {
    obstruction: bool,
    visited: [bool; 4]
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    North = 0,
    East = 1, 
    South = 2,
    West = 3
}

impl Direction {
    fn next(&self) -> Direction {
        use Direction::*;
        match self {
            North => East,
            East => South,
            South => West,
            West => North,
        }
    }
}

fn get_map_and_start_pos(lines: &Vec<String>) -> (Vec<Vec<PosInfo>>, Option<(usize, usize)>) {
    let mut start_pos = None;
    let map = lines.iter()
    .enumerate()
    .map(|(x, s)| s.chars()
        .enumerate()
        .map(|(y, c)| {
            let p = PosInfo {
                obstruction: c == '#',
                visited: [c == '^', false, false, false],
            };
            if p.visited.into_iter().any(|d| d) && start_pos.is_none() {
                start_pos = Some((x, y));
            }
            p
        })
    .collect())
    .collect();
    (map, start_pos)
}

fn get_tile_at(x: i32, y: i32, map: &mut Vec<Vec<PosInfo>>) -> Option<& mut PosInfo> {
    if x < 0 || y < 0 {
        return None;
    }
    if let Some(row) = map.get_mut(x as usize) {
        if let Some(value) = row.get_mut(y as usize) {
            return Some(value);
        }
    }
    None
}

fn insert_obstruction(x: usize, y: usize, map: &Vec<Vec<PosInfo>>) -> Option<Vec<Vec<PosInfo>>> {
    let mut map_mod = map.clone();
    if let Some(tile) = get_tile_at(x as i32, y as i32, &mut map_mod) {
        if !tile.obstruction {
            tile.obstruction = true;
            return Some(map_mod);
        }
    }
    None
}

fn walk(map: &mut Vec<Vec<PosInfo>>, start_pos: &(usize, usize)) -> (u32, bool) {
    let mut walk_dir: Direction = Direction::North;  // Start direction
    let mut current_pos = (start_pos.0 as i32, start_pos.1 as i32);    // Start position
    let mut unique_count = 1; // Start posistion already visited
    let mut loop_detected = false;

    loop {
        let (dx, dy): (i32, i32) = match walk_dir {
            Direction::North => (-1, 0),
            Direction::East => (0, 1),
            Direction::South => (1, 0),
            Direction::West => (0, -1)
        };
    
        let next_pos = (current_pos.0 + dx, current_pos.1 + dy);
        if let Some(tile) = get_tile_at(next_pos.0, next_pos.1, map) {
            if tile.obstruction {
                walk_dir = walk_dir.next()
            } else {
                current_pos = next_pos;
                if !(tile.visited.into_iter().any(|d| d)) {
                    unique_count += 1;
                } else if tile.visited[walk_dir as usize] {
                   loop_detected = true;
                   break;
                }
                tile.visited[walk_dir as usize] = true;
            }
        } else {
            break;
        }
        //println!("{:?}", current_pos);
        //println!("{:?}", unique_count);
    }
    (unique_count, loop_detected)
}

impl Solution for Day06 {
    type Item = u32;

    fn day(&self) -> u8 {
        return 6;
    }

    fn part1(&self, lines: &Vec<String>) -> Result<Self::Item, &str> { 
        let (mut map, start_pos) = get_map_and_start_pos(lines);
        let (count, _) = walk(&mut map, &start_pos.unwrap());
        Ok(count)
    }
    
    fn part2(&self, lines: &Vec<String>) -> Result<Self::Item, &str> { 
        let (map, start_pos) = get_map_and_start_pos(lines);
        let mut num_obs_for_loop: u32 = 0;
        for x in 0..map.len() {
            for y in 0..map[0].len() {
                //println!("x: {}, y: {}", x, y);
                if (x,y) != start_pos.unwrap() {
                    if let Some(mut mod_map) = insert_obstruction(x, y, &map) {
                        let loop_detected: bool;
                        (_, loop_detected) = walk(&mut mod_map, &start_pos.unwrap());
                        if loop_detected {
                            num_obs_for_loop += 1;
                        }
                    }
                }
            }
        }
        Ok(num_obs_for_loop)
    }

}