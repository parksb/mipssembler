use std::fs::File;
use std::io::{BufRead, BufReader, Lines, Seek, SeekFrom};
use std::iter::Map;

use crate::constants::WORD;

pub fn read_lines(
    file: &mut File,
) -> Map<Lines<BufReader<&mut File>>, fn(std::io::Result<String>) -> String> {
    if let Ok(_) = file.seek(SeekFrom::Start(0)) {
        BufReader::new(file).lines().map(|line| line.unwrap())
    } else {
        panic!("Failed to set position to zero.");
    }
}

pub fn convert_string_to_int(code: &str) -> i32 {
    if code.chars().nth(0).unwrap() == '-' {
        i32::from_str_radix(&code[1..code.len()], 16).unwrap() * -1
    } else {
        if code.starts_with("0x") {
            i32::from_str_radix(code.trim_start_matches("0x"), 16).unwrap()
        } else {
            i32::from_str_radix(code, 10).unwrap()
        }
    }
}

pub fn get_address_height(current_address: i32, target_address: i32) -> i32 {
    (target_address - current_address) / WORD - 1
}

pub fn convert_int_to_binary(number: i32, bit: i32) -> String {
    match bit {
        32 => format!("{:032b}", number),
        26 => format!("{:026b}", number),
        16 => {
            if number < 0 {
                let result = format!("{:016b}", number);
                result[16..result.len()].to_string()
            } else {
                format!("{:016b}", number)
            }
        }
        6 => format!("{:06b}", number),
        5 => format!("{:05b}", number),
        _ => format!("{:b}", number),
    }
}
