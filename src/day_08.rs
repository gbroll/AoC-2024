use crate::Solution;
use std::collections::{HashMap, HashSet};

pub struct Day08;


fn get_antennas(lines: &Vec<String>) -> (HashMap<char, Vec<(i32, i32)>>, (i32, i32)) {
    let mut antennas:  HashMap<char, Vec<(i32, i32)>> = HashMap::new();
    lines.iter().enumerate().for_each(|(x, l)| {
        l.chars().enumerate().for_each(|(y, c)| {
            if c != '.' {
                if let Some(vec) = antennas.get_mut(&c) {
                    vec.push((x as i32, y as i32))
                } else {
                    antennas.insert(c, vec![(x as i32, y as i32)]);
                }
            }
        });
    });
   (antennas, (lines.len() as i32, lines[0].len() as i32))
}

fn is_in_grid(x: i32, y: i32, grid_size: &(i32, i32)) -> bool {
    if x < 0 || x >= grid_size.0 {
        return false;
    }
    if y < 0 || y >= grid_size.1 {
        return false;
    }
    true
}


fn count_antinodes(antennas: &HashMap<char, Vec<(i32, i32)>>, grid_size: (i32, i32), repeat_pattern: bool) -> u32 {
    let mut antinodes: HashSet<(i32, i32)> = HashSet::new();
    for (_, locations) in antennas {
        for ix0 in 0..locations.len()-1 {
            for ix1 in ix0 + 1..locations.len() {
                let (x0, y0) = (locations[ix0].0, locations[ix0].1);
                let (x1, y1) = (locations[ix1].0, locations[ix1].1);
                let (dx, dy) = (x1-x0, y1-y0);
                let mut n = 0;
                loop {
                    n += 1;
                    if is_in_grid(x0-n*dx, y0-n*dy, &grid_size) {
                        antinodes.insert((x0-n*dx, y0-dy));
                    } else {
                        break;
                    }
                    if !repeat_pattern {
                        break;
                    }
                }
                loop {
                    if is_in_grid(x1+dx, y1+dy, &grid_size) {
                        antinodes.insert((x1 + dx, y1 + dy));
                    } else {
                        break;
                    }
                    if !repeat_pattern {
                        break;
                    }
                }
            }
        }
    }
    antinodes.len() as u32
}


impl Solution for Day08 {
    type Item = u32;

    fn day(&self) -> u8 {
        return 8;
    }

    fn part1(&self, lines: &Vec<String>) -> Result<Self::Item, &str> { 
        let (antennas, grid_size) = get_antennas(lines);
        for (key, value) in &antennas {
            println!("{}: {:?}", key, value);
        }
        let result = count_antinodes(&antennas, grid_size, false);
        return Ok(result);
    }
    
    fn part2(&self, lines: &Vec<String>) -> Result<Self::Item, &str> { 
        let (antennas, grid_size) = get_antennas(lines);
        let result = count_antinodes(&antennas, grid_size, true);
        return Ok(result);

    }

}
