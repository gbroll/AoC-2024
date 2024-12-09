use crate::Solution;

pub struct Day02;

impl Solution for Day02 {
    type Item = u32;

    fn day(&self) -> u8 {
        return 2;
    }

    fn part1(&self, lines: &Vec<String>) -> Result<Self::Item, &str> { 
        let reports = parse_reports(lines);
        let result = count_safe_reports(&reports);
        return Ok(result.0 as u32);
    }
    
    fn part2(&self, lines: &Vec<String>) -> Result<Self::Item, &str> { 
        let reports = parse_reports(lines);
        let result = count_safe_reports(&reports);
        return Ok(result.0 as u32 + result.1 as u32);
    }

}

fn parse_reports(lines: &Vec<String>) -> Vec<Vec<u8>> {
    let reports: Vec<Vec<u8>> = lines
    .into_iter()
    .filter(|l| !l.trim().is_empty())
    .map(|l| l.split_whitespace()
        .map(|num| num.parse::<u8>().unwrap())
        .collect()
    ).
    collect();
    reports
}

fn count_safe_reports(reports: &Vec<Vec<u8>>) -> (u16, u16) {
    let mut safe_count: u16 = 0;
    let mut almost_safe_count: u16 = 0;
    for report in reports.into_iter() {
        if report_is_safe(report) {
            safe_count += 1;
        } else {
            for (ix, _) in report.into_iter().enumerate() {
                let mut adj_report = report.clone();
                adj_report.remove(ix);
                if report_is_safe(&adj_report) {
                    almost_safe_count += 1;
                    break;
                }
            }
        }
    }
    (safe_count, almost_safe_count)
}

fn report_is_safe(report: &Vec<u8>) -> bool {
    let diffs: &Vec<i32> = &report.windows(2).map(|v| (v[1] as i32 - v[0] as i32)).collect();
    let sign = diffs[0].signum();
    if diffs.iter().all(|&x| x.signum() == sign && x.abs() <= 3 && x.abs() > 0) {
        true 
    } else {
        false
    }
}
