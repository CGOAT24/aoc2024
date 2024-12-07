use std::time::Instant;
use regex::Regex;

fn calc_sum(script: &Vec<(u32, u32)>) -> u32 {
    script.iter().fold(0, |acc, x| acc + (x.0 * x.1))
}

fn extract_numbers(str: &str) -> (u32, u32) {
    let regex = Regex::new(r"\d+").unwrap();
    let nums: Vec<u32> = regex.find_iter(str).filter_map(|m| m.as_str().parse::<u32>().ok()).collect();
    (nums[0], nums[1])
}

fn part1(lines: &Vec<String>) -> Vec<(u32, u32)> {
    let regex = Regex::new(r"mul\(\d+,\d+\)").unwrap();
    lines
        .iter()
        .flat_map(|line|
            regex
                .find_iter(line)
                .map(|m| m.as_str())
                .map(|capture| extract_numbers(&capture)
                )
        ).collect()
}

fn part2(lines: &Vec<String>) -> Vec<(u32, u32)> {
    let regex = Regex::new(r"mul\(\d+,\d+\)|do\(\)|don't\(\)").unwrap();
    let mut active = true;

    lines.iter().flat_map(|line|
        regex.find_iter(line).filter_map(|m| {
            match m.as_str() {
                "do()" => {
                    active = true;
                    None
                },
                "don't()" => {
                    active = false;
                    None
                },
                _ if active => Some(extract_numbers(m.as_str())),
                _ => None
            }
        }).collect::<Vec<_>>()
    ).collect()
}

fn main() {
    let now = Instant::now();
    let lines = utils::read_file("input.txt".as_ref()).unwrap();

    let script1 = part1(&lines);
    let script2 = part2(&lines);

    let result1 = calc_sum(&script1);
    let result2 = calc_sum(&script2);

    println!("Part 1: {}", result1);
    println!("Part 2: {}", result2);
    println!("Time elapsed: {}ms", now.elapsed().as_millis());
}
