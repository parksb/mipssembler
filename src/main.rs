use std::env;
use std::fs::File;

mod constants;
mod datum;
mod instruction;
mod label;
mod pseudo_instruction;
mod text;
mod utils;

use crate::constants::{DATA_SECTION_MIN_ADDRESS, TEXT_SECTION_MIN_ADDRESS, WORD};
use crate::datum::{resolve_data, Datum};
use crate::label::{get_addressed_labels, is_label, resolve_labels, Label};
use crate::pseudo_instruction::disassemble_pseudo_instruction;
use crate::text::{get_text_from_code, Text};
use crate::utils::{convert_int_to_binary, read_lines};
use std::io::Write;

pub enum Section {
    NONE,
    DATA,
    TEXT,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_filepath = &args[1];
    let output_filepath = &args[2];
    let mut input_file = File::open(input_filepath).expect("Failed to read input file.");

    let (data, labels) = extract_data_and_labels(&mut input_file);
    let codes = extract_codes(&data, &mut input_file);
    let labels = get_addressed_labels(&codes, &labels);
    let texts = disassemble_instructions(&data, &labels, &codes);

    write_output(output_filepath, &data, &texts);

    println!("Done!");
}

fn extract_data_and_labels(mut input_file: &mut File) -> (Vec<Datum>, Vec<Label>) {
    let mut current_address = DATA_SECTION_MIN_ADDRESS - WORD;
    let mut current_section = Section::NONE;

    let mut data: Vec<Datum> = vec![];
    let mut labels: Vec<Label> = vec![];

    let mut prev_datum_name: Option<String> = None;

    for line in read_lines(&mut input_file) {
        current_section = resolve_section(&line).unwrap_or(current_section);

        match current_section {
            Section::DATA => {
                if resolve_section(&line).is_none() {
                    if let Some(datum) = resolve_data(&line, &prev_datum_name, current_address) {
                        prev_datum_name = Some(datum.name.clone());
                        data.push(datum);
                    }
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

fn extract_codes(data: &[Datum], mut input_file: &mut File) -> Vec<String> {
    let mut codes = vec![];
    let mut current_section = Section::NONE;

    for line in read_lines(&mut input_file) {
        current_section = resolve_section(&line).unwrap_or(current_section);

        if let Section::TEXT = current_section {
            if !line.is_empty() && resolve_section(&line).is_none() {
                if !is_label(&line) {
                    let pseudo_instruction_codes = disassemble_pseudo_instruction(&line, &data);
                    if let Some(pseudo_instruction_codes) = pseudo_instruction_codes {
                        codes.extend(pseudo_instruction_codes);
                    } else {
                        codes.push(line.trim_start().to_string());
                    }
                } else {
                    codes.push(line);
                }
            }
        }
    }

    codes
}

fn disassemble_instructions(data: &[Datum], labels: &[Label], codes: &[String]) -> Vec<Text> {
    let mut current_address = TEXT_SECTION_MIN_ADDRESS;
    codes
        .iter()
        .filter_map(|code| {
            if resolve_labels(&code).is_none() {
                let text = get_text_from_code(&code, current_address, &data, &labels);
                current_address += WORD;
                Some(text)
            } else {
                None
            }
        })
        .collect()
}

fn write_output(filepath: &str, data: &[Datum], texts: &[Text]) {
    let data_section_size = data.len() as i32 * WORD;
    let text_section_size = texts.len() as i32 * WORD;

    let data_section_size_binary = convert_int_to_binary(data_section_size, 32);
    let text_section_size_binary = convert_int_to_binary(text_section_size, 32);

    let mut result = vec![text_section_size_binary, data_section_size_binary];
    result.extend(texts.iter().map(|text| text.to_binary()));
    result.extend(data.iter().map(|datum| datum.to_binary()));

    let mut file = File::create(filepath).expect("Failed to crate output file.");
    write!(file, "{}", result.join("")).expect("Failed to write output file.");
}

fn resolve_section(code: &str) -> Option<Section> {
    match code {
        "\t.data" => Some(Section::DATA),
        "\t.text" => Some(Section::TEXT),
        _ => None,
    }
}
