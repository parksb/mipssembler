use regex::Regex;
use std::env;
use std::fs::File;

mod constants;
mod models;
mod utils;

use crate::constants::{INSTRUCTION_TABLE, WORD};
use crate::models::{
    ArgumentType, Datum, Instruction, InstructionFormat, Label, Section, Text,
};
use crate::utils::{convert_string_to_int, read_lines};

fn main() {
    let mut data: Vec<Datum> = vec![];
    let mut labels: Vec<Label> = vec![];
    let mut texts: Vec<Text> = vec![];

    let args: Vec<String> = env::args().collect();
    let input_filepath = &args[1];
    let mut input_file = File::open(input_filepath).expect("Failed to read input file.");

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
) {
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
}

fn disassemble_instructions(
    data: &mut Vec<Datum>,
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
    texts: &mut Vec<Text>,
) {
    if let [name, arguments] = text.trim_start().split('\t').collect::<Vec<&str>>()[..] {
        let instruction = INSTRUCTION_TABLE
            .iter()
            .find(|table| table.compare_name(name))
            .unwrap();

        let argument_texts = arguments
            .split(",")
            .map(|arg| arg.trim())
            .collect::<Vec<&str>>();

        let arguments = resolve_arguments(&argument_texts, &data, &labels);

        for text in get_text_by_format(&instruction, &arguments, current_address) {
            texts.push(text);
        }
    }
}

fn get_text_by_format(
    instruction: &Instruction,
    arguments: &Vec<i32>,
    current_address: i32,
) -> Vec<Text> {
    let Instruction { funct, opcode, .. } = instruction;
    match convert_opcode_to_format(instruction.opcode) {
        InstructionFormat::REGISTER => vec![Text::new(
            arguments[1],
            arguments[2],
            arguments[0],
            0,
            *funct,
            *opcode,
            0,
            0,
        )],
        InstructionFormat::JUMP => vec![Text::new(
            0,
            0,
            0,
            0,
            *funct,
            *opcode,
            0,
            get_address_difference(current_address, arguments[0]),
        )],
        InstructionFormat::IMMEDIATE => vec![Text::new(
            arguments[0],
            arguments[1],
            0,
            0,
            *funct,
            *opcode,
            arguments[2],
            0,
        )],
        InstructionFormat::PSEUDO => panic!("A pseudo instruction found.")
    }
}

fn resolve_arguments(
    argument_texts: &Vec<&str>,
    data: &Vec<Datum>,
    labels: &Vec<Label>,
) -> Vec<i32> {
    let mut arguments: Vec<i32> = vec![];

    for argument_text in argument_texts {
        match resolve_argument_type(argument_text) {
            ArgumentType::NUMBER => arguments.push(convert_string_to_int(argument_text)),
            ArgumentType::REGISTER => arguments.push(convert_string_to_int(
                &argument_text[1..argument_text.len()],
            )),
            ArgumentType::LABEL => {
                if let Some(datum) = find_datum(argument_text, &data) {
                    arguments.push(datum.get_address());
                } else {
                    if let Some(label) = find_label(argument_text, &labels) {
                        arguments.push(label.get_address());
                    } else {
                        panic!("Failed to resolve argument value");
                    }
                }
            }
            ArgumentType::STACK => {
                if let [offset, base] = argument_text.split('(').collect::<Vec<&str>>()[..] {
                    let offset = convert_string_to_int(offset);
                    let base = convert_string_to_int(&base[1..(base.len() - 1)]);

                    arguments.push(offset);
                    arguments.push(base);
                } else {
                    panic!("Failed to resolve argument value");
                }
            }
        }
    }

    arguments
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

fn get_address_difference(current_address: i32, target_address: i32) -> i32 {
    (target_address - current_address) / WORD + 1
}

fn convert_opcode_to_format(opcode: i32) -> InstructionFormat {
    match opcode {
        0 => InstructionFormat::REGISTER,
        2 | 3 => InstructionFormat::JUMP,
        -1 => InstructionFormat::PSEUDO,
        _ => InstructionFormat::IMMEDIATE,
    }
}

fn find_datum<'a>(name: &'a str, data: &'a Vec<Datum>) -> Option<&'a Datum> {
    data.iter().find(|datum| datum.compare_name(name))
}

fn find_label<'a>(name: &'a str, labels: &'a Vec<Label>) -> Option<&'a Label> {
    labels.iter().find(|label| label.compare_name(name))
}
