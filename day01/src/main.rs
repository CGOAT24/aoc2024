use std::collections::HashMap;
use utils;

fn part1(vec1: Vec<u64>, vec2: Vec<u64>) -> u64 {
    let mut acc = 0;
    for i in 0..vec1.len() {
        acc += vec1[i].abs_diff(vec2[i]);
    }
    acc
}

fn part2(vec1: Vec<u64>, vec2: Vec<u64>) -> u64 {
    let mut map = HashMap::new();

    for num in vec2 {
        map.entry(num).and_modify(|x| *x += 1).or_insert(1);
    }

    let mut acc = 0;
    for num in vec1 {
        if map.contains_key(&num) {
            acc += num * map.get(&num).unwrap();
        }
    }
    acc
}

fn to_sorted_u64_vecs(lines: Vec<String>) -> (Vec<u64>, Vec<u64>) {
    let mut vec1 = Vec::new();
    let mut vec2 = Vec::new();

    for line in lines {
        let result = line.split_whitespace().collect::<Vec<&str>>();
        let num1 = result[0].parse::<u64>();
        let num2 = result[1].parse::<u64>();

        if let (Ok(num1), Ok(num2)) = (num1, num2) {
            vec1.push(num1);
            vec2.push(num2);
        }
    }

    vec1.sort();
    vec2.sort();

    (vec1, vec2)
}

fn main() {
    let lines = utils::read_file("input.txt".as_ref()).unwrap();

    let (vec1, vec2) = to_sorted_u64_vecs(lines);
    let result1 = part1(vec1.clone(), vec2.clone());
    let result2 = part2(vec1, vec2);

    println!("Part 1: {}", result1);
    println!("Part 2: {}", result2);
}