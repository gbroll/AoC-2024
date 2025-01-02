use crate::Solution;
use std::collections::{HashMap, VecDeque};

pub struct Day12;


fn to_2d_char_array(lines: &Vec<String>) -> Vec<Vec<char>> {
    lines.into_iter()
    .map(|s| s.chars().collect())
    .collect()
}

fn get_nb(point: &(usize, usize), offset: &(i32, i32), garden: &Vec<Vec<char>>) -> Option<(usize, usize, char)> {
    let x: i32 = point.0 as i32 + offset.0;
    let y: i32 = point.1 as i32 + offset.1;
    if x < 0 || y < 0 {
        return None;
    }
    if let Some(row) = garden.get(x as usize) {
        if let Some(value) = row.get(y as usize) {
            return Some((x as usize, y as usize, value.clone()));
        }
    }
    None
}

fn fence_garden_regions(garden: &Vec<Vec<char>>) -> u64 {
    let mut regions: HashMap<(usize, usize), char> = HashMap::new();
    let mut total_price: u64 = 0;
    for x in 0..garden.len() {
        for y in 0..garden[0].len() {
            if regions.contains_key(&(x, y)) {
                continue;
            }
            let (area, perimeter) = fence_region_from_point(&(x,y), garden, &mut regions);
            total_price += (area * perimeter) as u64;
        }
    }
    total_price
}

fn fence_region_from_point(point: &(usize, usize), garden: &Vec<Vec<char>>, visited: &mut HashMap<(usize, usize), char>) -> (u32, u32) {
    let mut region: HashMap<(usize, usize), char> = HashMap::new();
    let (mut area, mut perimeter) = (0, 0);
    let offsets = vec![(-1, 0), (0, 1), (1, 0), (0, -1)];
    let mut q: VecDeque<(usize, usize)> = VecDeque::new();
    q.push_back((point.0, point.1));
    area += 1;
    region.insert((point.0, point.1), garden[point.0][point.1]);
    while !q.is_empty() {
        let pt = q.pop_front().unwrap();
        for offset in &offsets {
            if let Some(nb) = get_nb(&pt, offset, garden) {
                if region.contains_key(&(nb.0, nb.1)) {
                    continue;
                }
                if nb.2 == garden[point.0][point.1] {
                    q.push_back((nb.0, nb.1));
                    area += 1;
                    region.insert((nb.0, nb.1), garden[nb.0][nb.1]);
                    println!("{:?}", (nb));
                } else {
                    perimeter += 1;
                }
            } else {
                perimeter += 1;
            }
        }
    }
    println!("Area: {:?}, perimeter: {:?}", area, perimeter);
    visited.extend(region.iter());
    (area, perimeter)
}

impl Solution for Day12 {
    type Item = u64;

    fn day(&self) -> u8 {
        return 1;
    }

    fn part1(&self, lines: &Vec<String>) -> Result<Self::Item, String> { 
        let garden = to_2d_char_array(lines);
        println!("{:?}", garden);
        let total_cost = fence_garden_regions(&garden);
        Ok(total_cost)
    }
    
    fn part2(&self, lines: &Vec<String>) -> Result<Self::Item, String> { 
        Ok(2)
    }

}
