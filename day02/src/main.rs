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
    if nums[0] > nums[1] {
        for i in 1..nums.len() {
            if nums[i] > nums[i - 1] {
                return false;
            }
        }
        true
    }
    else {
        for i in 1..nums.len() {
            if nums[i] < nums[i - 1] {
                return false;
            }
        }
        true
    }
}

fn is_report_safe(levels: Vec<u16>) -> bool {
    if differs_constantly(levels.clone()) {
        for i in (1..levels.len()).rev() {
            if !is_valid_difference(levels[i], levels[i - 1]) {
                return false;
            }
        }
        return true;
    }
    false
}

fn to_u16_vec(input: Vec<String>) -> Vec<Vec<u16>> {
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
    let nums = to_u16_vec(lines);

    let result1 = part1(nums.clone());
    let result2 = part2(nums);
    println!("Part 1: {}", result1);
    println!("Part 2: {}", result2);
}
