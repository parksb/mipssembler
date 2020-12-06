use std::fs::File;
use std::io::{BufRead, BufReader, Lines, Seek, SeekFrom};
use std::iter::Map;

use crate::constants::WORD;
use std::ops::Neg;

type LinesMap<'a> = Map<Lines<BufReader<&'a mut File>>, fn(std::io::Result<String>) -> String>;

pub fn read_lines(file: &mut File) -> LinesMap {
    if file.seek(SeekFrom::Start(0)).is_ok() {
        BufReader::new(file).lines().map(|line| line.unwrap())
    } else {
        panic!("Failed to set position to zero.");
    }
}

pub fn convert_string_to_int(code: &str) -> i32 {
    if code.starts_with("0x") {
        i32::from_str_radix(code.trim_start_matches("0x"), 16).unwrap()
    } else if code.starts_with('-') {
        i32::from_str_radix(code.trim_start_matches('-'), 10)
            .unwrap()
            .neg()
    } else {
        i32::from_str_radix(code, 10).unwrap()
    }
}

pub fn get_address_difference(current_address: i32, target_address: i32) -> i32 {
    (target_address - current_address) / WORD - 1
}

pub fn convert_int_to_binary(number: i32, bit: i32) -> String {
    let binary = format!("{:032b}", number);
    binary[(binary.len() - (bit as usize))..].to_string()
}
