use std::env;
use std::fs::File;
use std::io::Write;

mod constants;
mod datum;
mod instruction;
mod label;
mod line;
mod pseudo_instruction;
mod section;
mod text;
mod utils;

use crate::constants::{TEXT_SECTION_MIN_ADDRESS, WORD};
use crate::datum::{extract_data_from_lines, Datum};
use crate::label::{get_addressed_labels, is_label, resolve_labels, Label};
use crate::line::{compose_lines, Line};
use crate::pseudo_instruction::disassemble_pseudo_instruction;
use crate::section::{resolve_section, Section};
use crate::text::{get_text_from_code, Text};
use crate::utils::convert_int_to_binary;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_filepath = &args[1];
    let output_filepath = &args[2];
    let mut input_file = File::open(input_filepath).expect("Failed to read input file.");

    let lines = compose_lines(&mut input_file);

    let data = extract_data_from_lines(&lines);
    let codes = extract_codes(&lines, &data);
    let labels = get_addressed_labels(&lines, &codes);
    let texts = disassemble_instructions(&data, &labels, &codes);

    write_output(output_filepath, &data, &texts);

    println!("Done!");
}

fn extract_codes(lines: &[Line], data: &[Datum]) -> Vec<String> {
    lines
        .iter()
        .filter(|line| {
            line.section == Section::TEXT && resolve_section(line.text.as_ref().unwrap()).is_none()
        })
        .flat_map(|line| {
            if !is_label(&line.text.as_ref().unwrap()) {
                if let Some(pseudo_instruction_codes) =
                    disassemble_pseudo_instruction(&line.text.as_ref().unwrap(), &data)
                {
                    pseudo_instruction_codes
                } else {
                    vec![line.text.clone().unwrap().trim_start().to_string()]
                }
            } else {
                vec![line.text.clone().unwrap()]
            }
        })
        .collect()
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
