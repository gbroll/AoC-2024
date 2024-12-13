use crate::Solution;
pub struct Day04;

#[derive(Debug)]
enum SearchDir {
    Horizontal,
    Vertical,
    DiagonalLowerLeft,
    DiagonalLowerRight
}

impl Solution for Day04 {
    type Item = u32;

    fn day(&self) -> u8 {
        return 4;
    }

    fn part1(&self, lines: &Vec<String>) -> Result<Self::Item, &str> { 
        let grid = to_2d_char_array(lines);
        let search_word = vec!['X', 'M', 'A', 'S'];
        let word_count = search_grid(&grid, search_word);
        Ok(word_count)
    }
    
    fn part2(&self, lines: &Vec<String>) -> Result<Self::Item, &str> { 
        let result: u32 = 1;
        Ok(result)
    }

}

fn to_2d_char_array(lines: &Vec<String>) -> Vec<Vec<char>> {
    lines.into_iter()
    .map(|s| s.chars().collect())
    .collect()
}

fn get_char_at(x: i32, y: i32, grid: &Vec<Vec<char>>) -> Option<char> {
    if x < 0 || y < 0 {
        return None;
    }
    if let Some(row) = grid.get(x as usize) {
        if let Some(&value) = row.get(y as usize) {
            return Some(value);
        }
    }
    None
}

fn get_char_slice_at(x: u32, y: u32, search_dir: &SearchDir, len: usize, grid: &Vec<Vec<char>>) -> Option<Vec<char>> {
    
    let mut slice: Vec<char> = Vec::new();
    let (dx, dy): (i32, i32) = match search_dir {
        SearchDir::Horizontal => (0, 1),
        SearchDir::Vertical => (1, 0),
        SearchDir::DiagonalLowerLeft => (-1, 1),
        SearchDir::DiagonalLowerRight => (1, 1),
    };
        for i in 0..len {
            if let Some(value) = get_char_at(x as i32 + i as i32*dx, y as i32 + i as i32*dy, grid) {
                slice.push(value);
            } else {
                return None
            }
        }
        Some(slice)
}

fn search_grid(grid: &Vec<Vec<char>>, search_word: Vec<char>) -> (u32) {
    let search_dirs = [
        SearchDir::Horizontal,
        SearchDir::Vertical,
        SearchDir::DiagonalLowerLeft,
        SearchDir::DiagonalLowerRight
    ];
    let mut word_count: u32 = 0;
    for x in 0..grid.len() {
        for y in 0..grid.len() {
            for search_dir in &search_dirs {
                if let Some(slice) = get_char_slice_at(x as u32, y as u32, &search_dir, search_word.len(), grid) {
                    //println!("{:?} slice at x: {}, y: {}: {:?}", &search_dir, x, y, slice);
                    if slice == search_word {
                        word_count += 1
                    } else if slice.iter().rev().cloned().collect::<Vec<char>>() == search_word {
                        word_count += 1
                    }
                }
            }
        }
    }
    word_count
}
   

