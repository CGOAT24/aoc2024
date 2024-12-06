fn part1(rules: &Vec<(u32, u32)>, updates: &Vec<Vec<u32>>) -> u32 {
    updates
        .iter()
        .filter(|update| is_update_valid(&update, &rules))
        .fold(0, |acc, update| acc + get_middle_entry(&update))
}

fn part2(rules: &Vec<(u32, u32)>, updates: &Vec<Vec<u32>>) -> u32 {
    updates
        .iter()
        .filter(|update| !is_update_valid(&update, &rules))
        .map(|update| fix_ordering(update, rules))
        .fold(0, |acc, update| acc + get_middle_entry(&update))
}

fn fix_ordering(update: &Vec<u32>, rules: &Vec<(u32, u32)>) -> Vec<u32> {
    let mut fixed = update.clone();
    fixed.sort_by(|a, b| {
        if rules.contains(&(*a, *b)) {
            std::cmp::Ordering::Less
        }
        else if rules.contains(&(*b, *a)) {
            std::cmp::Ordering::Greater
        }
        else {
            std::cmp::Ordering::Equal

        }
    });
    fixed
}

fn get_rules(lines: &Vec<String>) -> Vec<(u32, u32)> {
    lines
        .iter()
        .map(|line| {
            let words: Vec<&str> = line.split("|").collect();
            return if words.len() == 2 {
                let first = words[0].parse::<u32>().unwrap();
                let second = words[1].parse::<u32>().unwrap();
                Ok((first, second))
            }
            else {
                Err(())
            }
        })
        .filter(|x| x.is_ok())
        .map(|x| x.unwrap())
        .collect()
}

fn get_updates(lines: &Vec<String>) -> Vec<Vec<u32>> {
    lines
        .iter()
        .map(|line| {
            let words: Vec<&str> = line.split(",").collect();
            return if words.len() > 1 {
                Ok(words.iter().map(|word| word.parse::<u32>().unwrap()).collect())
            }
            else {
                Err(())
            }
        })
        .filter(|x| x.is_ok())
        .map(|x| x.unwrap())
        .collect()
}

fn is_update_valid(update: &Vec<u32>, rules: &Vec<(u32, u32)>) -> bool {
    for i in 0..update.len() - 1 {
        let first = update[i];
        let second = update[i + 1];

        if !rules.contains(&(first, second)) {
            return false;
        }
    }
    true
}

fn get_middle_entry(update: &Vec<u32>) -> u32 {
    update[update.len() / 2]
}

fn main() {
    let lines = utils::read_file("input.txt".as_ref()).unwrap();

    let rules = get_rules(&lines);
    let updates = get_updates(&lines);

    let result1 = part1(&rules, &updates);
    let result2 = part2(&rules, &updates);

    println!("Part 1: {}", result1);
    println!("Part 2: {}", result2);
}