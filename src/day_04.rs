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

    fn part1(&self, lines: &Vec<String>) -> Result<Self::Item, String> { 
        let grid = to_2d_char_array(lines);
        let search_word = vec!['X', 'M', 'A', 'S'];
        let word_count = part1_count(&grid, search_word);
        Ok(word_count)
    }
    
    fn part2(&self, lines: &Vec<String>) -> Result<Self::Item, String> { 
        let grid = to_2d_char_array(lines);
        let search_word = vec!['M', 'A', 'S'];
        let word_count = part2_count(&grid, search_word);
        Ok(word_count)
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

fn get_char_slice_at(x: i32, y: i32, search_dir: &SearchDir, len: usize, grid: &Vec<Vec<char>>) -> Option<Vec<char>> {
    if x < 0 || y < 0 {
        return None;
    }

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

fn part1_count(grid: &Vec<Vec<char>>, search_word: Vec<char>) -> u32 {
    let mut count: u32 = 0;
    let search_dirs = [
        SearchDir::Horizontal,
        SearchDir::Vertical,
        SearchDir::DiagonalLowerLeft,
        SearchDir::DiagonalLowerRight
    ];
    for x in 0..grid.len() {
        for y in 0..grid.len() {
            for search_dir in &search_dirs {
                if let Some(slice) = get_char_slice_at(x as i32, y as i32, &search_dir, search_word.len(), grid) {
                    //println!("{:?} slice at x: {}, y: {}: {:?}", &search_dir, x, y, slice);
                    if match_words(&slice, &search_word) {
                        count += 1
                    }
                }
            }
        }
    }
    count
}
   
fn part2_count(grid: &Vec<Vec<char>>, search_word: Vec<char>) -> u32 {
    let mut count: u32 = 0;
    for x in 0..grid.len() {
        for y in 0..grid.len() {
            if let Some(slice1) = get_char_slice_at(x as i32, y as i32, &SearchDir::DiagonalLowerLeft, search_word.len(), grid) {
                if let Some(slice2) = get_char_slice_at(x as i32-2, y as i32, &SearchDir::DiagonalLowerRight, search_word.len(), grid) {
                    if match_words(&slice1, &search_word) && match_words(&slice2, &search_word) {
                        count += 1;
                    }
                }
        }
    }
}
    count
}

fn match_words(first: &Vec<char>, second: &Vec<char>) -> bool {
    first == second || first == &second.iter().rev().cloned().collect::<Vec<char>>()
}