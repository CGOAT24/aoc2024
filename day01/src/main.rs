use std::collections::HashMap;
use utils;

fn part1(vec1: &Vec<u64>, vec2: &Vec<u64>) -> u64 {
    vec1
        .iter()
        .zip(vec2.iter())
        .map(|(a, b)| a.abs_diff(*b))
        .sum()
}

fn part2(vec1: &Vec<u64>, vec2: &Vec<u64>) -> u64 {
    let mut map = HashMap::new();

    vec2.iter().for_each(|num| {
        map.entry(num).and_modify(|x| *x += 1).or_insert(1);
    });

    vec1
        .iter()
        .filter(|x| map.contains_key(x))
        .map(|x| x * map.get(x).unwrap())
        .sum()
}

fn parse_input(lines: Vec<String>) -> (Vec<u64>, Vec<u64>) {
    let mut vec1 = Vec::new();
    let mut vec2 = Vec::new();

    for line in lines {
        let result = line.split_whitespace().collect::<Vec<&str>>();
        let num1 = result[0].parse::<u64>().unwrap();
        let num2 = result[1].parse::<u64>().unwrap();
        vec1.push(num1);
        vec2.push(num2);
    }

    vec1.sort();
    vec2.sort();

    (vec1, vec2)
}

fn main() {
    let lines = utils::read_file("input.txt".as_ref()).unwrap();

    let (vec1, vec2) = parse_input(lines);
    let result1 = part1(&vec1, &vec2);
    let result2 = part2(&vec1, &vec2);

    println!("Part 1: {}", result1);
    println!("Part 2: {}", result2);
}