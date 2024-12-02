fn part1(input: Vec<Vec<u16>>) -> usize {
    let mut safe_reports_count = 0;
    for row in input {
        if is_report_safe(row) {
            safe_reports_count += 1;
        }
    }
    safe_reports_count
}

fn part2(input: Vec<Vec<u16>>) -> usize {
    let mut safe_reports_count = 0;
    for report in input {
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

fn is_valid_difference(num1: u16, num2: u16) -> bool {
    num1.abs_diff(num2) <= 3 && num1.abs_diff(num2) >= 1
}

fn differs_constantly(nums: Vec<u16>) -> bool {
    let first: i32 = nums[0] as i32;
    let second: i32 = nums[1] as i32;

    for i in 1..nums.len() {
        let num1: i32 = nums[i] as i32;
        let num2: i32 = nums[i - 1] as i32;
        if (num1 - num2 < 0) == (first - second < 0) {
            return false;
        }
    }
    true
}

fn is_report_safe(levels: Vec<u16>) -> bool {
    if differs_constantly(levels.clone()) {
        for i in 1..levels.len() {
            if !is_valid_difference(levels[i], levels[i - 1]) {
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
        let str_nums = line.split_whitespace().collect::<Vec<&str>>();
        let nums: Vec<u16> = str_nums.iter().map(|s| s.parse::<u16>().unwrap()).collect();
        result.push(nums);
    }

    result
}

fn main() {
    let lines = utils::read_file("input.txt".as_ref()).unwrap();
    let nums = parse_input(lines);

    let result1 = part1(nums.clone());
    let result2 = part2(nums);
    println!("Part 1: {}", result1);
    println!("Part 2: {}", result2);
}
