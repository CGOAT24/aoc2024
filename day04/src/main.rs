trait VecStringExt {
    fn at(&self, i: usize, j: usize) -> String;
}

impl VecStringExt for Vec<String> {
    fn at(&self, i: usize, j: usize) -> String {
        self.get(i).and_then(|s| s.chars().nth(j)).unwrap().to_string()
    }
}

fn get_horizontal(line: &String, start: usize) -> String {
    line.chars().skip(start).take(4).collect()
}

fn get_vertical(lines: &Vec<String>, start_i: usize, j: usize) -> String {
    (start_i..start_i + 4)
        .map(|i| lines.at(i, j))
        .collect()
}

fn get_diagonal_southeast(lines: &Vec<String>, start_i: usize, start_j: usize) -> String {
    (start_i..start_i + 4)
        .map(|i| lines.at(i, start_j + (i - start_i)))
        .collect()
}

fn get_diagonal_northeast(lines: &Vec<String>, start_i: usize, start_j: usize) -> String {
    (0..4)
        .map(|i| lines.at(start_i + 3 - i, start_j + i))
        .collect()
}

fn part1(lines: &Vec<String>) -> usize {
    fn check_word(word: String) -> bool {
        matches!(word.as_str(), "XMAS" | "SAMX")
    }

    let mut count = 0;

    for i in 0..lines.len() {
        for j in 0..lines[i].len() {
            //horizontal
            if check_word(get_horizontal(&lines[i], j)) {
                count += 1;
            }

            //vertical
            if i < lines.len() - 3 && check_word(get_vertical(lines, i, j)){
                count += 1;
            }

            if i >= lines.len() - 3 || j >= lines.get(i).unwrap().len() - 3 {
                continue;
            }

            //diagonal northeast
            if check_word(get_diagonal_northeast(lines, i, j)) {
                count += 1;
            }

            //diagonal southeast
            if check_word(get_diagonal_southeast(lines, i, j)) {
                count += 1;
            }
        }
    }
    count
}

fn get_symbol(lines: &Vec<String>, i: usize, j: usize) -> Vec<char> {
    (0..5)
        .map(|k| match k {
            0 => lines.at(i, j).chars().nth(0).unwrap(),
            1 => lines.at(i, j + 2).chars().nth(0).unwrap(),
            2 => lines.at(i + 1, j + 1).chars().nth(0).unwrap(),
            3 => lines.at(i + 2, j).chars().nth(0).unwrap(),
            _ => lines.at(i + 2, j + 2).chars().nth(0).unwrap(),
        })
        .collect()
}

fn validate_symbol(symbol: Vec<char>) -> bool {
    let line1 = format!("{}{}{}", symbol[0], symbol[2], symbol[4]);
    let line2 = format!("{}{}{}", symbol[1], symbol[2], symbol[3]);

    (line1 == "MAS" || line1 == "SAM") && (line2 == "MAS" || line2 == "SAM")
}

fn part2(lines: &Vec<String>) -> usize {
    (0..lines.len() - 2)
        .flat_map(|i| (0..lines[i].len() - 2).map(move |j| (i, j)))
        .filter(|&(i, j)| validate_symbol(get_symbol(&lines, i, j)))
        .count()
}

fn main() {
    let lines = utils::read_file("input.txt".as_ref()).unwrap();

    let result1 = part1(&lines);
    let result2 = part2(&lines);

    println!("Part 1: {}", result1);
    println!("Part 2: {}", result2);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_data() -> Vec<String> {
        Vec::from([
            String::from("MMMSXXMASM"),
            String::from("MSAMXMSMSA"),
            String::from("AMXSXMAAMM"),
            String::from("MSAMASMSMX"),
            String::from("XMASAMXAMM"),
            String::from("XXAMMXXAMA"),
            String::from("SMSMSASXSS"),
            String::from("SAXAMASAAA"),
            String::from("MAMMMXMMMM"),
            String::from("MXMXAXMASX")
        ])
    }
    #[test]
    fn test_get_horizontal() {
        let line: String = "MMMSXXMASM".parse().unwrap();
        assert_eq!(get_horizontal(&line, 0), "MMMS");
    }

    #[test]
    fn test_get_vertical() {
        let lines = get_test_data();
        assert_eq!(get_vertical(&lines, 0, 0), "MMAM");
    }

    #[test]
    fn test_get_diagonal_southeast() {
        let lines = get_test_data();
        assert_eq!(get_diagonal_southeast(&lines, 0, 0), "MSXM");
    }

    #[test]
    fn test_get_diagonal_northeast() {
        let lines = get_test_data();
        assert_eq!(get_diagonal_northeast(&lines, 0, 0), "MMAS");
    }

    #[test]
    fn test_part1() {
        let lines = get_test_data();
        assert_eq!(part1(&lines), 18);
    }

    #[test]
    fn test_validate_symbol() {
        let lines = get_test_data();
        let symbol = get_symbol(&lines, 0, 1);
        assert_eq!(validate_symbol(symbol), true);
    }

    #[test]
    fn test_at() {
        let lines = get_test_data();
        assert_eq!(lines.at(0, 0), "M");
    }
}