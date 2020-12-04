use regex::Regex;
use std::env;
use std::fs::File;

mod constants;
mod models;
mod utils;

use crate::constants::{INSTRUCTION_TABLE, WORD};
use crate::models::{ArgumentType, Datum, Instruction, InstructionFormat, Label, Section, Text};
use crate::utils::{convert_string_to_int, read_lines};

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_filepath = &args[1];
    let mut input_file = File::open(input_filepath).expect("Failed to read input file.");

    let (data, labels) = set_initial_values(&mut input_file);
    let codes = extract_text_from_file(&data, &mut input_file);
    let texts = disassemble_instructions(&data, &labels, &codes);

    println!("CODE: {:?}", codes);
    println!("DATA: {:?}", data);
    println!("LABELS: {:?}", labels);
    println!("TEXTS: {:?}", texts);
}

fn extract_text_from_file(data: &Vec<Datum>, mut input_file: &mut File) -> Vec<String> {
    let mut codes = vec![];
    let mut current_section = Section::NONE;
    let mut text_section_size = 0;

    for line in read_lines(&mut input_file, 0) {
        current_section = resolve_section(&line).unwrap_or(current_section);

        match current_section {
            Section::TEXT => {
                text_section_size += 1;
                if text_section_size > 1 && !line.is_empty() {
                    if !is_label(&line) {
                        if let Some(pseudo_instructions) =
                            disassemble_pseudo_instruction(&line, &data)
                        {
                            codes.extend(pseudo_instructions);
                        } else {
                            codes.push(line.trim_start().to_string());
                        }
                    } else {
                        codes.push(line);
                    }
                }
            }
            _ => (),
        }
    }

    codes
}

fn disassemble_pseudo_instruction(text: &str, data: &Vec<Datum>) -> Option<Vec<String>> {
    if let [name, arguments] = text.trim_start().split('\t').collect::<Vec<&str>>()[..] {
        match name {
            "la" => Some(la(arguments, &data)),
            _ => None,
        }
    } else {
        panic!("Invalid instruction")
    }
}

fn la(arguments: &str, data: &Vec<Datum>) -> Vec<String> {
    let mut result: Vec<String> = vec![];
    let argument_text = arguments
        .split(",")
        .map(|arg| arg.trim())
        .collect::<Vec<&str>>();

    if let [register, datum_name] = argument_text[..] {
        if let Some(datum) = find_datum(datum_name, &data) {
            let shifted_datum_address = datum.get_address() >> 16;
            result.push(format!("lui\t{}, {}", register, shifted_datum_address));

            if (datum.get_address() << 16) > 0 {
                let ad = datum.get_address() - 0x10000000;
                result.push(format!("ori\t{}, {}, {}", register, register, ad));
            }
        } else {
            panic!("Unknown data.");
        }
    } else {
        panic!("Failed.");
    }

    result
}

fn set_initial_values(mut input_file: &mut File) -> (Vec<Datum>, Vec<Label>) {
    let mut current_address = 0x10000000 - WORD;
    let mut current_section = Section::NONE;

    let mut data: Vec<Datum> = vec![];
    let mut labels: Vec<Label> = vec![];

    for line in read_lines(&mut input_file, 0) {
        current_section = resolve_section(&line).unwrap_or(current_section);

        match current_section {
            Section::DATA => {
                if let Some(datum) = resolve_data(&line, current_address) {
                    data.push(datum);
                }
                current_address += WORD;
            }
            Section::TEXT => {
                if let Some(label) = resolve_labels(&line) {
                    labels.push(label);
                }
            }
            Section::NONE => (),
        }
    }

    (data, labels)
}

fn disassemble_instructions(
    data: &Vec<Datum>,
    labels: &Vec<Label>,
    codes: &Vec<String>,
) -> Vec<Text> {
    let mut current_address = 0x10000000 - WORD;
    codes
        .iter()
        .filter_map(|line| {
            if let None = resolve_labels(&line) {
                current_address += WORD;
                Some(append_instructions_to_text(
                    &line,
                    current_address,
                    &data,
                    &labels,
                ))
            } else {
                None
            }
        })
        .collect()
}

fn append_instructions_to_text(
    text: &str,
    current_address: i32,
    data: &Vec<Datum>,
    labels: &Vec<Label>,
) -> Text {
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

        get_text_by_format(&instruction, &arguments, current_address)
    } else {
        panic!("No");
    }
}

fn get_text_by_format(
    instruction: &Instruction,
    arguments: &Vec<i32>,
    current_address: i32,
) -> Text {
    let Instruction { funct, opcode, .. } = instruction;
    match convert_opcode_to_format(instruction.opcode) {
        InstructionFormat::REGISTER => Text::new(
            arguments[1],
            arguments[2],
            arguments[0],
            0,
            *funct,
            *opcode,
            0,
            0,
        ),
        InstructionFormat::JUMP => Text::new(
            0,
            0,
            0,
            0,
            *funct,
            *opcode,
            0,
            get_address_difference(current_address, arguments[0]),
        ),
        InstructionFormat::IMMEDIATE => {
            if arguments.len() < 3 {
                Text::new(0, arguments[0], 0, 0, *funct, *opcode, arguments[1], 0)
            } else {
                Text::new(
                    arguments[0],
                    arguments[1],
                    0,
                    0,
                    *funct,
                    *opcode,
                    arguments[2],
                    0,
                )
            }
        }
        InstructionFormat::PSEUDO => panic!("A pseudo instruction found."),
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

fn resolve_data(text: &str, current_address: i32) -> Option<Datum> {
    if let [name, meta] = text.split(":\t").collect::<Vec<&str>>()[..] {
        if let [_, value] = meta.split('\t').collect::<Vec<&str>>()[..] {
            let parsed_value = convert_string_to_int(value);
            Some(Datum::new(name, parsed_value, current_address))
        } else {
            None
        }
    } else {
        None
    }
}

fn resolve_labels(text: &str) -> Option<Label> {
    let label_regex = Regex::new(r"^.*:").unwrap();
    if let Some(cap) = label_regex.captures_iter(&text).next() {
        let name = cap[0].trim_end_matches(':');
        Some(Label::new(name, 0, 0))
    } else {
        None
    }
}

fn is_label(text: &str) -> bool {
    let label_regex = Regex::new(r"^.*:").unwrap();
    label_regex.is_match(text)
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
