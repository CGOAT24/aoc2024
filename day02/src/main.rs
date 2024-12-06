fn part1(reports: &Vec<Vec<u16>>) -> usize {
    reports
        .iter()
        .filter(|&report| is_report_safe(report))
        .count()
}

fn part2(reports: &Vec<Vec<u16>>) -> usize {
    let mut safe_reports_count = 0;
    for report in reports {
        if is_report_safe(&report) {
            safe_reports_count += 1;
        }
        else {
            if remove_bad_level(&report).is_some() {
                safe_reports_count += 1;
            }
        }
    }
    safe_reports_count
}

fn remove_bad_level(report: &Vec<u16>) -> Option<Vec<u16>> {
    report.iter().enumerate().find_map(|(i, _)| {
        let mut fixed = report.clone();
        fixed.remove(i);
        if is_report_safe(&fixed) {
            Some(fixed)
        }
        else {
            None
        }
    })
}

fn is_valid_difference(level1: u16, level2: u16) -> bool {
    level1.abs_diff(level2) <= 3 && level1.abs_diff(level2) >= 1
}

fn differs_constantly(report: &Vec<u16>) -> bool {
    let first_level = report[0] as i32;
    let second_level = report[1] as i32;

    for i in 1..report.len() {
        let level1 = report[i] as i32;
        let level2 = report[i - 1] as i32;
        if (level1 - level2 < 0) == (first_level - second_level < 0) {
            return false;
        }
    }
    true
}

fn is_report_safe(report: &Vec<u16>) -> bool {
    if differs_constantly(&report) {
        for i in 1..report.len() {
            if !is_valid_difference(report[i], report[i - 1]) {
                return false;
            }
        }
        return true;
    }
    false
}

fn parse_input(input: Vec<String>) -> Vec<Vec<u16>> {
    input
        .iter()
        .map(|line|
            line
                .split_whitespace()
                .map(|s| s.parse::<u16>().unwrap())
                .collect()
        )
        .collect()
}

fn main() {
    let lines = utils::read_file("input.txt".as_ref()).unwrap();
    let reports = parse_input(lines);

    let result1 = part1(&reports);
    let result2 = part2(&reports);

    println!("Part 1: {}", result1);
    println!("Part 2: {}", result2);
}
