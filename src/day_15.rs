use std::usize;
use std::collections::{VecDeque, HashSet};
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

fn resize_warehouse(warehouse: &Vec<Vec<char>>, start_position: (i32, i32)) -> (Vec<Vec<char>>, (i32, i32)) {
    let mut resized_warehouse: Vec<Vec<char>> = Vec::new();
    for row in warehouse {
        let mut new_row: Vec<char> = Vec::new();
        for char in row {
            if *char == 'O' {
                new_row.push('[');
                new_row.push(']');
            } else {
                new_row.push(*char);
                new_row.push(*char);
            }
        }
        resized_warehouse.push(new_row);
    }
    (resized_warehouse, (start_position.0, start_position.1 * 2))
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

fn get_next_pos(pos: &(i32, i32), move_dir: &(i32, i32)) -> (i32, i32) {
    (pos.0 + move_dir.0, pos.1 + move_dir.1)
}

fn get_prev_pos(pos: &(i32, i32), move_dir: &(i32, i32)) -> (i32, i32) {
    (pos.0 - move_dir.0, pos.1 - move_dir.1)
}

fn add_box(boxes: &mut VecDeque<(i32, i32)>, pos: &(i32, i32), warehouse: &Vec<Vec<char>>) {
    boxes.push_back(*pos);
    if warehouse[pos.0 as usize][pos.1 as usize] == '[' {
        boxes.push_back((pos.0, pos.1 + 1));
    } else if warehouse[pos.0 as usize][pos.1 as usize] == ']' {
        boxes.push_back((pos.0, pos.1 - 1));
    }
}

fn do_move_part2(warehouse: &mut Vec<Vec<char>>, pos: &mut (i32, i32), move_dir: (i32, i32)) -> () {

    let mut next_pos = get_next_pos(pos, &move_dir);
    let mut next_type = warehouse[next_pos.0 as usize][next_pos.1 as usize];
    
    // Move freely or hit wall?
    if next_type == '.' {
        *pos = next_pos;
        return
    } else if next_type == '#' {
        return
    }

    // Horizontal move, rather easy, just toggle the braces
    if move_dir.0 == 0 {
        let this_pos = next_pos;
        loop {
            next_pos = (next_pos.0, next_pos.1 + move_dir.1);
            next_type = warehouse[next_pos.0 as usize][next_pos.1 as usize];
            if next_type == '.' {
                break;
            } else if next_type == '#' {
                return
            }
        }
        // Go back and toggle
        while this_pos.1 != next_pos.1 {
            let prev_pos = get_prev_pos(&next_pos, &move_dir);
            warehouse[next_pos.0 as usize][next_pos.1 as usize] = warehouse[prev_pos.0 as usize][prev_pos.1 as usize];
            next_pos = prev_pos;
        }
        *pos = this_pos;
        warehouse[this_pos.0 as usize][this_pos.1 as usize] = '.';

    } else {
        // Pushing boxes vertically, worse
        let mut boxes: VecDeque<(i32, i32)> = VecDeque::new();
        let mut boxes_to_be_moved: VecDeque<(i32, i32)> = VecDeque::new();
        add_box(&mut boxes, &next_pos, &warehouse);
        while !boxes.is_empty() {
            let box_pos = boxes.pop_front().unwrap();
            boxes_to_be_moved.push_back(box_pos);
            let next_pos = get_next_pos(&box_pos, &move_dir);
            let next_type = warehouse[next_pos.0 as usize][next_pos.1 as usize];
            if next_type == '[' || next_type == ']' {
                add_box(&mut boxes, &next_pos, warehouse);
            } else if next_type == '#' {
                return
            }
        }

        // Move should be OK
        let mut moved_boxes: HashSet<(i32, i32)> = HashSet::new();
        while !boxes_to_be_moved.is_empty() {
            let box_pos = boxes_to_be_moved.pop_back().unwrap();
            if !moved_boxes.contains(&box_pos)
            {
                let target_position = get_next_pos(&box_pos, &move_dir);
                warehouse[target_position.0 as usize][target_position.1 as usize] = warehouse[box_pos.0 as usize][box_pos.1 as usize];
                warehouse[box_pos.0 as usize][box_pos.1 as usize] = '.';
                moved_boxes.insert(box_pos);
            }
        }
        *pos = next_pos;
    }
}

#[allow(dead_code)] 
fn display(warehouse: &Vec<Vec<char>>, pos: &(i32, i32)) -> () {
    for (row_num, row) in warehouse.iter().enumerate() {
        for (col, val) in row.iter().enumerate() {
            if row_num as i32 == pos.0 && col as i32 == pos.1 {
                print!("{}", '@');
            } else {
                print!("{}", val);
            }
        }
        println!("");
    }
    println!("");
}

fn sum_box_coordinates(warehouse: Vec<Vec<char>>, box_char: char) -> i32 {
    warehouse
    .iter().enumerate().map(|(row_num, row)| 
        row.iter().enumerate().filter_map(|(col_num, &val)| {
            if val == box_char {
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
            //display(&warehouse, &pos);
        }
        Ok(sum_box_coordinates(warehouse, 'O'))
    }
    
    fn part2(&self, lines: &Vec<String>) -> Result<Self::Item, &str> { 
        let (warehouse, start_position, moves) = parse_input(lines);
        let (mut warehouse, start_position) = resize_warehouse(&warehouse, start_position);
        let mut pos = start_position;
        //display(&warehouse, &pos);
        for move_dir in moves {
            do_move_part2(&mut warehouse, &mut pos, move_dir);
            //display(&warehouse, &pos);
        }
        Ok(sum_box_coordinates(warehouse, '['))
    }
}
