use crate::Solution;
use regex::Regex;

pub struct Day14;

#[derive(Clone)]
struct Robot {
    position: (i32, i32),
    velocity: (i32, i32)
}

fn get_robot_init_state(lines: &Vec<String>) -> Vec<Robot> {
    let re = Regex::new(r"p=([0-9]+),([0-9]+) v=(-?[0-9]+),(-?[0-9]+)").unwrap();
    let mut robots: Vec<Robot> = Vec::new();
    for line in lines {
        for captures in re.captures_iter(line) {
            robots.push(
                Robot {
                    position: (
                        captures.get(1).unwrap().as_str().parse::<i32>().unwrap(),
                        captures.get(2).unwrap().as_str().parse::<i32>().unwrap()
                    ),
                    velocity: (
                        captures.get(3).unwrap().as_str().parse::<i32>().unwrap(),
                        captures.get(4).unwrap().as_str().parse::<i32>().unwrap()                    
                    )
                }
            )
        }
    }
    robots
}

fn update_robots(robots: &mut Vec<Robot>, grid_size: &(u32, u32), num_seconds: u32) -> () {
    robots
    .iter_mut()
    .for_each(|robot| {
        robot.position = (
            (robot.position.0 + robot.velocity.0 * num_seconds as i32) % grid_size.0 as i32, 
            (robot.position.1 + robot.velocity.1 * num_seconds as i32) % grid_size.1 as i32
        );
        if robot.position.0 < 0 {
            robot.position.0 = grid_size.0 as i32 + robot.position.0;
        }
        if robot.position.1 < 0 {
            robot.position.1 = grid_size.1 as i32 + robot.position.1;
        }
    });
}

fn calc_safety_factor(robots: &Vec<Robot>, grid_size: &(u32, u32)) -> u32 {
    let quadrant_sums = sum_robots_in_quadrants(robots, &grid_size);
    quadrant_sums.0 * quadrant_sums.1 * quadrant_sums.2 * quadrant_sums.3
}

fn sum_robots_in_quadrants(robots: &Vec<Robot>, grid_size: &(u32, u32)) -> (u32, u32, u32, u32) {
    let mut quadrant_sums = (0, 0, 0, 0);
    robots
    .iter().for_each(|robot| {
        
        if robot.position.0 < (grid_size.0 as i32 - 1)/2 {
            if robot.position.1 < (grid_size.1 as i32 -1)/2{
                quadrant_sums.0 += 1;
            } else if robot.position.1 > (grid_size.1 as i32 -1)/2 {
                quadrant_sums.2 += 1;
            }
        } else if robot.position.0 > (grid_size.0 as i32 - 1)/2 {
            if robot.position.1 < (grid_size.1 as i32 -1)/2{
                quadrant_sums.1 += 1;
            } else if robot.position.1 > (grid_size.1 as i32 -1)/2 {
                quadrant_sums.3 += 1;
            }
        }
    });
    quadrant_sums
}

fn display_positions(robots: &Vec<Robot>, grid_size: &(u32, u32)) -> () {
    let mut grid: Vec<Vec<char>> = vec![vec!['.'; grid_size.0 as usize]; grid_size.1 as usize];
    for robot in robots {
        grid[robot.position.1 as usize][robot.position.0 as usize] = '#';
    }
    
    for row in grid.iter() {
        for &val in row.iter() {
            print!("{}", val);
        }
        println!();
    }
    println!("\n");

}

impl Solution for Day14 {
    type Item = u32;

    fn day(&self) -> u8 {
        return 14;
    }

    fn part1(&self, lines: &Vec<String>) -> Result<Self::Item, String> { 
        let mut robots = get_robot_init_state(lines);
        let (width, height): (u32, u32) = (101, 103);
        let num_seconds: u32 = 100;
        update_robots(&mut robots, &(width, height), num_seconds);

        Ok(calc_safety_factor(&robots, &(width, height)))
    }
    
    fn part2(&self, lines: &Vec<String>) -> Result<Self::Item, String> { 
        let mut robots = get_robot_init_state(lines);
        let (width, height): (u32, u32) = (101, 103);

        let mut num_seconds: u32 = 1;
        let mut min_sf = 1000000000;
        let mut probable_solution = 0;
        let mut robot_state: Vec<Robot> = robots.clone();
        while num_seconds < width * height {
            update_robots(&mut robots, &(width, height), 1);
            let sf = calc_safety_factor(&robots, &(width, height));
            if sf < min_sf {
                min_sf = sf;
                probable_solution = num_seconds;
                robot_state = robots.clone();
            }
            num_seconds += 1;
        }
        display_positions(&robot_state, &(width, height));
        Ok(probable_solution)
    }

}
