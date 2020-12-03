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

pub fn convert_string_to_int(text: &str) -> i32 {
    if text.chars().nth(0).unwrap() == '-' {
        i32::from_str_radix(text[1..text.len()].trim_start_matches("0x"), 16).unwrap() * -1
    } else {
        i32::from_str_radix(text.trim_start_matches("0x"), 16).unwrap()
    }
}
