use crate::Solution;
use regex::Regex;

pub struct Day13;

#[derive(Debug, Copy, Clone)]
pub struct Equation {
    a: i64,
    b: i64,
    loc: i64
}

fn parse(lines: &Vec<String>) -> Vec<(Equation, Equation)> {
    let mut game_equations: Vec<(Equation, Equation)> = Vec::new();
    let re_a = Regex::new(r"Button A: X\+([0-9]+), Y\+([0-9]+)").unwrap();
    let re_b = Regex::new(r"Button B: X\+([0-9]+), Y\+([0-9]+)").unwrap();
    let re_c = Regex::new(r"Prize: X=([0-9]+), Y=([0-9]+)").unwrap();
    let mut a: (i64, i64) = (0, 0);
    let mut b: (i64, i64) = (0, 0);
    let mut loc: (i64, i64) = (0, 0);

    for line in lines {
        for captures in re_a.captures_iter(line){
            a = (
                captures.get(1).unwrap().as_str().parse::<i64>().unwrap(),
                captures.get(2).unwrap().as_str().parse::<i64>().unwrap()
            );
        }
        for captures in re_b.captures_iter(line){
            b = (
                captures.get(1).unwrap().as_str().parse::<i64>().unwrap(),
                captures.get(2).unwrap().as_str().parse::<i64>().unwrap()
            );
        }
        for captures in re_c.captures_iter(line){
            loc = (
                captures.get(1).unwrap().as_str().parse::<i64>().unwrap(),
                captures.get(2).unwrap().as_str().parse::<i64>().unwrap()
            );
        }
        if line.is_empty() {
            game_equations.push((Equation {a: a.0, b: b.0, loc: loc.0}, Equation{a: a.1, b: b.1, loc: loc.1}));
        }
    }
    game_equations.push((Equation {a: a.0, b: b.0, loc: loc.0}, Equation{a: a.1, b: b.1, loc: loc.1}));
    game_equations
}

fn solve(equations: &Vec<(Equation, Equation)>, add_to_loc: i64)-> u64 {
    let mut total_cost: u64 = 0;
    for eqsys in equations {
        let mut eq0 = eqsys.0;
        let mut eq1 = eqsys.1;
    
        // For part 2
        eq0.loc += add_to_loc;
        eq1.loc += add_to_loc;

        let b: i64 = (eq0.loc * eq1.a - eq1.loc * eq0.a) / (eq0.b * eq1.a - eq0.a * eq1.b);
        let a: i64 = (eq0.loc - b * eq0.b) / eq0.a;

        if eq0.a*a + eq0.b*b == eq0.loc && eq1.a*a + eq1.b*b == eq1.loc {
            total_cost += 3*a as u64 + b as u64;
        }
    }
    total_cost
}

impl Solution for Day13 {
    type Item = u64;

    fn day(&self) -> u8 {
        return 2;
    }

    fn part1(&self, lines: &Vec<String>) -> Result<Self::Item, &str> { 
        let game_equations = parse(lines);
        let total_cost = solve(&game_equations, 0);
        Ok(total_cost)
    }
    
    fn part2(&self, lines: &Vec<String>) -> Result<Self::Item, &str> { 
        let game_equations = parse(lines);
        let total_cost = solve(&game_equations, 10000000000000);
        Ok(total_cost)
    }

}
