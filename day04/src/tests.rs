#[cfg(test)]
mod tests {
    use crate::*;

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
    fn test_part2() {
        let lines = get_test_data();
        assert_eq!(part2(&lines), 9);
    }

    #[test]
    fn test_at() {
        let lines = get_test_data();
        assert_eq!(lines.at(0, 0), "M");
    }
}