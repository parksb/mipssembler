use std::fs::File;
use std::io::{BufRead, BufReader, Lines, Seek, SeekFrom};
use std::iter::Map;

pub fn read_lines(
    file: &mut File,
    start_location: u64,
) -> Map<Lines<BufReader<&mut File>>, fn(std::io::Result<String>) -> String> {
    file.seek(SeekFrom::Start(start_location));
    BufReader::new(file).lines().map(|line| line.unwrap())
}
