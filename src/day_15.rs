use std::usize;

use crate::Solution;

pub struct Day15;

fn parse_input(lines: &Vec<String>) -> (Vec<Vec<char>>, (i32, i32), Vec<(i32, i32)>) {
    let mut warehouse: Vec<Vec<char>> = Vec::new();
    let mut start_position: (i32, i32) = (0, 0);
    let mut moves: Vec<(i32, i32)> = Vec::new();

    for (row, line) in lines.iter().enumerate() {
        if line.starts_with("#") {
           if line.contains("@") {
                warehouse.push(
                    line.chars().enumerate().map(|(col, c)| {
                        if c == '@' {
                            start_position = (row as i32, col as i32);
                            '.'
                        } else {
                            c
                        }
    
                    }).collect()
                )
            } else {
               warehouse.push(line.chars().collect());
            }
        }
        else if !line.is_empty() {
            moves.extend(
            line.chars().map(|c| {
                match c {
                    '^' => (-1, 0),
                    '>' => (0, 1),
                    'v' => (1, 0),
                    '<' => (0, -1),
                    _ => (0, 0)
                }
            })
        )
        }
    }
    (warehouse, start_position, moves)
}

fn do_move(warehouse: &mut Vec<Vec<char>>, pos: &mut (i32, i32), move_dir: (i32, i32)) -> () {

    let mut next_pos = (pos.0 + move_dir.0, pos.1 + move_dir.1);
    let mut next_type = warehouse[next_pos.0 as usize][next_pos.1 as usize];

    if next_type == '.' {
        *pos = next_pos;
        return
    } else if next_type == '#' {
        return
    } else {
        let this_pos = next_pos;
        loop {
            next_pos = (next_pos.0 + move_dir.0, next_pos.1 + move_dir.1);
            next_type = warehouse[next_pos.0 as usize][next_pos.1 as usize];
            
            if next_type == '.' {
                warehouse[next_pos.0 as usize][next_pos.1 as usize] = 'O';
                warehouse[this_pos.0 as usize][this_pos.1 as usize] = '.';
                *pos = this_pos;
                break;
            } else if next_type == '#' {
                return
            }
        }
    }

}

fn sum_box_coordinates(warehouse: Vec<Vec<char>>) -> i32 {
    warehouse
    .iter().enumerate().map(|(row_num, row)| 
        row.iter().enumerate().filter_map(|(col_num, &val)| {
            if val == 'O' {
                Some(row_num as i32 * 100 + col_num as i32)
            } else {
                None
            }
        }).sum::<i32>()
    ).sum()
}

impl Solution for Day15 {
    type Item = i32;

    fn day(&self) -> u8 {
        return 15;
    }

    fn part1(&self, lines: &Vec<String>) -> Result<Self::Item, &str> { 
        let (mut warehouse, start_position, moves) = parse_input(lines);
        let mut pos = start_position;
        for move_dir in moves {
            do_move(&mut warehouse, &mut pos, move_dir);
            println!("{:?}", pos);
        }
        Ok(sum_box_coordinates(warehouse))
    }
    
    fn part2(&self, lines: &Vec<String>) -> Result<Self::Item, &str> { 
        Ok(2)
    }

}
