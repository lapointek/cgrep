use anyhow::{Context, Result};
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn contains_pattern(line: &str, pattern: &str) -> bool {
    line.contains(pattern)
}

pub fn search_file(path: &std::path::Path, pattern: &str) -> Result<Vec<String>> {
    // Open file and propagate errors automatically
    let file =
        File::open(path).with_context(|| format!("could not open file `{}`", path.display()))?;

    // Wrap file in BufReader
    let reader = BufReader::new(file);

    let mut results = Vec::new();
    for line in reader.lines() {
        let line = line?;
        if contains_pattern(&line, pattern) {
            results.push(line);
        }
    }
    Ok(results)
}
