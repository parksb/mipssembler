use std::fs::File;
use std::io::{BufRead, BufReader, Lines, Seek, SeekFrom};
use std::iter::Map;

use crate::constants::WORD;

pub fn read_lines(file: &mut File) -> Map<Lines<BufReader<&mut File>>, fn(std::io::Result<String>) -> String> {
    file.seek(SeekFrom::Start(0));
    BufReader::new(file).lines().map(|line| line.unwrap())
}

pub fn convert_string_to_int(code: &str) -> i32 {
    if code.chars().nth(0).unwrap() == '-' {
        i32::from_str_radix(code[1..code.len()].trim_start_matches("0x"), 16).unwrap() * -1
    } else {
        i32::from_str_radix(code.trim_start_matches("0x"), 16).unwrap()
    }
}

pub fn get_address_difference(current_address: i32, target_address: i32) -> i32 {
    (target_address - current_address) / WORD + 1
}
