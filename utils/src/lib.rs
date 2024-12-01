use std::fs::File;
use std::io::BufRead;

pub fn read_file(path: &std::path::Path) -> std::io::Result<Vec<String>> {
    let file = File::open(path)?;
    let reader = std::io::BufReader::new(file);

    let data = reader
        .lines()
        .map(|line_result| {
            line_result.map(|line| line.to_string())
        })
        .collect::<Result<Vec<String>, std::io::Error>>()?;
    Ok(data)
}