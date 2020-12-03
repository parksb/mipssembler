use regex::Regex;
use std::env;
use std::fs::File;

mod constants;
mod models;
mod utils;

use crate::constants::{INSTRUCTION_TABLES, WORD};
use crate::models::{Argument, ArgumentType, Datum, Label, Section, Text};
use crate::utils::read_lines;

fn main() {
    let mut data: Vec<Datum> = vec![];
    let mut labels: Vec<Label> = vec![];
    let mut texts: Vec<Text> = vec![];

    let args: Vec<String> = env::args().collect();
    let input_filepath = &args[1];
    let mut input_file = File::open(input_filepath).expect("Failed to read input file.");

    let (data_section_size, text_section_size) =
        set_initial_values(&mut data, &mut labels, &mut input_file);
    disassemble_instructions(&mut data, &mut labels, &mut texts, &mut input_file);

    println!("DATA: {:?}", data);
    println!("LABELS: {:?}", labels);
    println!("TEXTS: {:?}", texts);
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
            }
            Section::TEXT => {
                text_section_size += 1;
                if !resolve_labels(&line, current_address, &mut labels) {
                    current_address += WORD;
                }
            }
            Section::NONE => (),
        }
    }

    (data_section_size * WORD, text_section_size * WORD)
}

fn disassemble_instructions(
    mut data: &mut Vec<Datum>,
    mut labels: &mut Vec<Label>,
    mut texts: &mut Vec<Text>,
    mut input_file: &mut File,
) {
    let mut current_address = 0x10000000 - WORD;
    let mut current_section = Section::NONE;
    let mut text_section_size = 0;

    for line in read_lines(&mut input_file, 0) {
        current_section = resolve_section(&line).unwrap_or(current_section);

        match current_section {
            Section::TEXT => {
                text_section_size += 1;
                if !resolve_labels(&line, current_address, &mut labels) {
                    if text_section_size > 1 {
                        append_instructions_to_text(
                            &line,
                            current_address,
                            &data,
                            &labels,
                            &mut texts,
                        );
                    }
                    current_address += WORD;
                }
            }
            _ => (),
        }
    }
}

fn append_instructions_to_text(
    text: &str,
    current_address: i32,
    data: &Vec<Datum>,
    labels: &Vec<Label>,
    mut texts: &mut Vec<Text>,
) {
    if let [name, arguments] = text.trim_start().split('\t').collect::<Vec<&str>>()[..] {
        let instruction_table = INSTRUCTION_TABLES
            .iter()
            .find(|table| table.compare_name(name))
            .unwrap();

        let mut new_text = instruction_table.to_text(0, current_address, vec![], false);

        let argument_texts = arguments
            .split(",")
            .map(|arg| arg.trim())
            .collect::<Vec<&str>>();

        let mut stack_arguments: Vec<i32> = vec![];
        let mut argument_values = argument_texts
            .iter()
            .filter_map(|argument_text| {
                let argument_type = resolve_argument_type(argument_text);

                match argument_type {
                    ArgumentType::NUMBER => Some(convert_string_to_int(argument_text)),
                    ArgumentType::REGISTER => Some(convert_string_to_int(
                        &argument_text[1..argument_text.len()],
                    )),
                    ArgumentType::LABEL => {
                        if let Some(datum) =
                            data.iter().find(|datum| datum.compare_name(argument_text))
                        {
                            new_text.set_is_label(false);
                            Some(datum.get_address())
                        } else {
                            if let Some(label) = labels
                                .iter()
                                .find(|label| label.compare_name(argument_text))
                            {
                                new_text.set_is_label(true);
                                Some(label.get_address())
                            } else {
                                panic!("Failed to resolve argument value");
                            }
                        }
                    }
                    ArgumentType::STACK => {
                        if let [offset, base] = argument_text.split('(').collect::<Vec<&str>>()[..]
                        {
                            let offset = convert_string_to_int(offset);
                            let base = convert_string_to_int(&base[1..(base.len() - 1)]);

                            stack_arguments = vec![offset, base];
                            None
                        } else {
                            panic!("Failed to resolve argument value");
                        }
                    }
                }
            })
            .collect::<Vec<i32>>();

        if stack_arguments.len() > 0 {
            argument_values.append(&mut stack_arguments);
        }

        new_text.set_arguments(argument_values);

        texts.push(new_text);
    }
}

fn resolve_argument_type(text: &str) -> ArgumentType {
    let arguments = [
        (Regex::new(r"^\$\d*").unwrap(), ArgumentType::REGISTER),
        (Regex::new(r"^[a-z]\w*").unwrap(), ArgumentType::LABEL),
        (Regex::new(r"^-?\d+\(\$\d*\)").unwrap(), ArgumentType::STACK),
        (Regex::new(r"^(0x)?\d*").unwrap(), ArgumentType::NUMBER),
    ];

    arguments
        .iter()
        .find(|arg| arg.0.is_match(text))
        .expect("Failed to resolve argument.")
        .clone()
        .1
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
            let parsed_value = convert_string_to_int(value);
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

fn convert_string_to_int(text: &str) -> i32 {
    if text.chars().nth(0).unwrap() == '-' {
        i32::from_str_radix(text[1..text.len()].trim_start_matches("0x"), 16).unwrap() * -1
    } else {
        i32::from_str_radix(text.trim_start_matches("0x"), 16).unwrap()
    }
}
