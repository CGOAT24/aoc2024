fn part1(reports: Vec<Vec<u16>>) -> usize {
    let mut safe_reports_count = 0;
    for row in reports {
        if is_report_safe(row) {
            safe_reports_count += 1;
        }
    }
    safe_reports_count
}

fn part2(reports: Vec<Vec<u16>>) -> usize {
    let mut safe_reports_count = 0;
    for report in reports {
        if is_report_safe(report.clone()) {
            safe_reports_count += 1;
        }
        else {
            if remove_bad_level(report).is_some() {
                safe_reports_count += 1;
            }
        }
    }
    safe_reports_count
}

fn remove_bad_level(report: Vec<u16>) -> Option<Vec<u16>> {
    for i in 0..report.len() {
        let mut fixed = report.clone();
        fixed.remove(i);
        if is_report_safe(fixed.clone()) {
            return Some(fixed);
        }
    }
    None
}

fn is_valid_difference(level1: u16, level2: u16) -> bool {
    level1.abs_diff(level2) <= 3 && level1.abs_diff(level2) >= 1
}

fn differs_constantly(report: Vec<u16>) -> bool {
    let first_level: i32 = report[0] as i32;
    let second_level: i32 = report[1] as i32;

    for i in 1..report.len() {
        let level1: i32 = report[i] as i32;
        let level2: i32 = report[i - 1] as i32;
        if (level1 - level2 < 0) == (first_level - second_level < 0) {
            return false;
        }
    }
    true
}

fn is_report_safe(report: Vec<u16>) -> bool {
    if differs_constantly(report.clone()) {
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
    let mut result: Vec<Vec<u16>> = Vec::new();

    for line in input {
        let str_report = line.split_whitespace().collect::<Vec<&str>>();
        let report: Vec<u16> = str_report.iter().map(|s| s.parse::<u16>().unwrap()).collect();
        result.push(report);
    }
    result
}

fn main() {
    let lines = utils::read_file("input.txt".as_ref()).unwrap();
    let reports = parse_input(lines);

    let result1 = part1(reports.clone());
    let result2 = part2(reports);
    println!("Part 1: {}", result1);
    println!("Part 2: {}", result2);
}
