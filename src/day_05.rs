use crate::Solution;
use std::collections::HashSet;

pub struct Day05;

fn get_rules(lines: &Vec<String>) -> HashSet<(u32, u32)> {
    lines
    .iter()
    .take_while(|l| !l.is_empty())
    .map(|l| {
        let mut columns = l.split('|').map(|num| num.parse::<u32>().unwrap());
        (columns.next().unwrap(), columns.next().unwrap())
    })
    .collect()
}

fn get_updates(lines: &Vec<String>) -> Vec<Vec<u32>> {
    lines
    .iter()
    .filter(|l| l.contains(','))
    .map(|l| l.split(',')
        .map(|num| num.parse::<u32>().unwrap())
        .collect::<Vec<u32>>())
    .collect()
}

fn gsort(rules: &HashSet<(u32, u32)>, update: &Vec<u32>) -> Vec<u32> {
    let mut svec = update.clone();
    let mut ix = 0;
    while ix < update.len() - 1 {
        let pair = (svec[ix], svec[ix + 1]);
        if !rules.contains(&pair) {
            svec.swap(ix, ix + 1);
            if ix > 0 {
                ix -= 1;
            }
        } else {
            ix += 1;
        }
    }
    svec
}

fn solve_parts(rules: &HashSet<(u32, u32)>, updates: &Vec<Vec<u32>>) -> (u32, u32) {
    let mut result = (0, 0);
    for update in updates {
        let mut update_valid = true;
        for ix in 0..update.len() - 1 {
            let pair = (update[ix], update[ix + 1]);
            if !rules.contains(&pair) {
                update_valid = false;
                let sorted_update = gsort(rules, update);
                result.1 += sorted_update[sorted_update.len()/2];
                break;
            }
        }
        if update_valid {
            result.0 += update[update.len()/2];
        } 
    }
    result
}

impl Solution for Day05 {
    type Item = u32;

    fn day(&self) -> u8 {
        return 5;
    }

    fn combine_part1_and_part2(&self) -> bool {
        true
    }

    fn solve(&self, input: &Vec<String>) -> Result<(Self::Item, Self::Item), String> {
        let rules = get_rules(&input);
        let updates = get_updates(&input);
        let result = solve_parts(&rules, &updates);
        Ok((result.0, result.1))
    }

}  

