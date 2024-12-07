use std::fmt::Debug;

#[derive(Debug)]
struct Equation {
    result: u64,
    numbers: Vec<u64>
}

enum Operator {
    Add,
    Multiply,
    Concat
}

impl Debug for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operator::Add => write!(f, "+"),
            Operator::Multiply => write!(f, "*"),
            Operator::Concat => write!(f, "||")
        }
    }
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

fn part1(eq: &Equation) -> u64 {
    let num_combinations = 1 << eq.numbers.len() - 1;
    let mut total_combinations: Vec<Vec<Operator>> = (0..num_combinations).map(|i| (0..eq.numbers.len() - 1).map(|bit| (i & (1 << bit)) != 0).map(|x| if x { Operator::Add } else { Operator::Multiply }).collect()).collect();

    let mut count = 0;
    for combination in total_combinations {
        if is_calibrated(&eq.numbers, &combination, eq.result) {
            count += eq.result;
            break;
        }
    }
    count
}

fn part2(eq: &Equation) -> u64 {
    todo!()
}

fn is_calibrated(numbers: &Vec<u64>, operators: &Vec<Operator>, expected: u64) -> bool {
    let mut result = numbers[0];
    let mut op_index = 0;
    for i in 1..numbers.len() {
        match operators[op_index] {
            Operator::Add => result += numbers[i],
            Operator::Multiply => result *= numbers[i],
            Operator::Concat => result = result * 10 + numbers[i]
        }
        op_index += 1;
    }
    result == expected
}

fn main() {
    let lines = utils::read_file("input.txt".as_ref()).unwrap();
    let equations = get_equations(&lines);

    let result1: u64 = equations.iter().map(|eq| part1(eq)).sum();
    //let result2: u64 = equations.iter().map(|eq| part2(eq)).sum();
    println!("Part 1: {}", result1);
}
