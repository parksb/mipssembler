use regex::Regex;
use std::env;
use std::fs::File;

mod constants;
mod models;
mod utils;

use crate::constants::WORD;
use crate::models::{Datum, Label, Section};
use crate::utils::read_lines;

fn main() {
    let mut data: Vec<Datum> = vec![];
    let mut labels: Vec<Label> = vec![];

    let args: Vec<String> = env::args().collect();
    let input_filepath = &args[1];
    let mut input_file = File::open(input_filepath).expect("Failed to read input file.");

    let (data_section_size, text_section_size) =
        set_initial_values(&mut data, &mut labels, &mut input_file);

    println!("DATA: {:?}", data);
    println!("LABELS: {:?}", labels);
}

fn set_initial_values(
    mut data: &mut Vec<Datum>,
    mut labels: &mut Vec<Label>,
    mut input_file: &mut File,
) -> (i32, i32) {
    let mut current_address = 0x10000000 - WORD;
    let mut current_section = Section::NONE;

    let mut data_section_size = 0;
    let mut text_section_size = 0;

    for line in read_lines(&mut input_file, 0) {
        current_section = resolve_section(&line).unwrap_or(current_section);

        match current_section {
            Section::DATA => {
                data_section_size += 1;
                resolve_data(&line, current_address, &mut data);
                current_address += WORD;
            },
            Section::TEXT => {
                text_section_size += 1;
                if !resolve_labels(&line, current_address, &mut labels) {
                    current_address += WORD;
                }
            },
            Section::NONE => (),
        }
    }

    (data_section_size * WORD, text_section_size * WORD)
}

fn resolve_section(text: &str) -> Result<Section, Section> {
    match text {
        "\t.data" => Ok(Section::DATA),
        "\t.text" => Ok(Section::TEXT),
        _ => Err(Section::NONE),
    }
}

fn resolve_data(text: &str, current_address: i32, data: &mut Vec<Datum>) -> bool {
    let is_data = true;
    if let [name, meta] = text.split(":\t").collect::<Vec<&str>>()[..] {
        if let [_, value] = meta.split('\t').collect::<Vec<&str>>()[..] {
            let parsed_value = i32::from_str_radix(value.trim_start_matches("0x"), 16).unwrap();
            data.push(Datum::new(name, parsed_value, current_address));
            return is_data;
        }
    }

    !is_data
}

fn resolve_labels(text: &str, current_address: i32, labels: &mut Vec<Label>) -> bool {
    let is_label = true;
    let label_regex = Regex::new(r"^.*:").unwrap();

    if let Some(cap) = label_regex.captures_iter(&text).next() {
        let name = cap[0].trim_end_matches(':');
        labels.push(Label::new(name, current_address, 0));
        is_label
    } else {
        !is_label
    }
}
