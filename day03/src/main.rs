use regex::Regex;

fn calc_sum(script: &Vec<(u32, u32)>) -> u32 {
    script.iter().fold(0, |acc, x| acc + (x.0 * x.1))
}

fn extract_numbers(str: &str) -> (u32, u32) {
    let num_extractor = Regex::new(r"\d+").unwrap();
    let nums: Vec<u32> = num_extractor.find_iter(str).filter_map(|m| m.as_str().parse::<u32>().ok()).collect();
    (nums[0], nums[1])
}

fn part1(lines: &Vec<String>) -> Vec<(u32, u32)> {
    let mul_extractor = Regex::new(r"mul\(\d+,\d+\)").unwrap();
    let mut result = Vec::new();
    for line in lines {
        let captures: Vec<&str> = mul_extractor.find_iter(line).map(|m| m.as_str()).collect();
        for capture in captures {
            result.push(extract_numbers(&capture));
        }
    }
    result
}

fn part2(lines: &Vec<String>) -> Vec<(u32, u32)> {
    let regex = Regex::new(r"mul\(\d+,\d+\)|do\(\)|don't\(\)").unwrap();
    let mut result = Vec::new();
    let mut active = true;

    for line in lines {
        let captures: Vec<&str> = regex.find_iter(line).map(|m| m.as_str()).collect();
        for capture in captures {
            match capture {
                "do()" => active = true,
                "don't()" => active = false,
                &_ => {
                    if active {
                        result.push(extract_numbers(&capture));
                    }
                }
            }
        }
    }
    result
}

fn main() {
    let lines = utils::read_file("input.txt".as_ref()).unwrap();

    let script1 = part1(&lines);
    let script2 = part2(&lines);

    let result1 = calc_sum(&script1);
    let result2 = calc_sum(&script2);

    println!("Part 1: {}", result1);
    println!("Part 2: {}", result2);
}
