use std::fs::File;
use std::io::BufRead;

pub fn read_file<T, F>(path: &std::path::Path, transform: F) -> std::io::Result<Vec<T>> where F: Fn(&str) -> T {
    let file = File::open(path)?;
    let reader = std::io::BufReader::new(file);

    let data = reader
        .lines()
        .map(|line_result| {
            line_result.map(|line| transform(&line))
        })
        .collect::<Result<Vec<T>, std::io::Error>>()?;
    Ok(data)
}