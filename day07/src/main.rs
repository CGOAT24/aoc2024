use std::fmt::Debug;
use std::time::Instant;

struct Equation {
    result: u64,
    numbers: Vec<u64>
}

impl Debug for Equation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {:?}", self.result, self.numbers)
    }
}

enum Operator {
    Add,
    Multiply,
    Concat
}

fn calc_total_calibration(equations: &Vec<Equation>, base: usize) -> u64 {
    equations.iter().fold(0, |acc, eq| {
        if cfg!(debug_assertions) {
            println!("Checking in base {} for: {:?}", base, eq);
        }

        let combinations = get_total_combinations(eq.numbers.len() - 1, base);
        acc + find_calibration_result(eq, &combinations)
    })
}

fn get_equations(lines: &Vec<String>) -> Vec<Equation> {
    lines.iter().map(|line| {
        let parts: Vec<&str> = line.split(":").collect();
        let result = parts[0].parse::<u64>().unwrap();
        let numbers = parts[1].split_whitespace().filter_map(|part| {
            match part.parse::<u64>() {
                Ok(num) => Some(num),
                Err(_) => None
            }
        }).collect::<Vec<u64>>();
        Equation { result, numbers }
    }).collect()
}

fn get_total_combinations(size: usize, base: usize) -> Vec<Vec<Operator>> {
    let mut vec = Vec::new();
    let num_combinations = base.pow(size as u32);

    for i in 0..num_combinations {
        let mut combination = Vec::new();
        let mut n = i;

        for _ in 0..size {
            combination.push((n % base) as u8);
            n /= base;
        }
        vec.push(combination.iter().map(|x| match x {
            0 => Operator::Add,
            1 => Operator::Multiply,
            2 => Operator::Concat,
            _ => panic!("Invalid operator")
        }).collect());
    }
    vec
}

fn find_calibration_result(eq: &Equation, total_combinations: &Vec<Vec<Operator>>) -> u64 {
    if total_combinations.iter().any(|c| is_calibrated(&eq.numbers, c, eq.result)) {
        eq.result
    }
    else {
        0
    }
}

fn is_calibrated(numbers: &Vec<u64>, operators: &Vec<Operator>, expected: u64) -> bool {
    let mut result = numbers[0];
    let mut op_index = 0;
    numbers.iter().skip(1).for_each(|num| {
        match operators[op_index] {
            Operator::Add => result += num,
            Operator::Multiply => result *= num,
            Operator::Concat => result = (result.to_string() + num.to_string().as_str()).parse::<u64>().unwrap()
        }
        op_index += 1;
    });
    result == expected
}

fn main() {
    let now = Instant::now();
    let lines = utils::read_file("input.txt".as_ref()).unwrap();
    let equations = get_equations(&lines);

    let result1: u64 = calc_total_calibration(&equations, 2);
    let result2: u64 = calc_total_calibration(&equations, 3);

    println!("Part 1: {}", result1);
    println!("Part 2: {}", result2);
    println!("Time elapsed: {}ms", now.elapsed().as_millis());
}