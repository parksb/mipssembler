use crate::constants::{DATA_SECTION_MIN_ADDRESS, WORD};
use crate::section::{resolve_section, Section};
use crate::utils::read_lines;
use std::fs::File;

pub struct Line {
    pub section: Section,
    pub address: i32,
    pub text: Option<String>,
}

impl Line {
    fn new(section: Section, address: i32, text: Option<String>) -> Self {
        Self {
            section,
            address,
            text,
        }
    }
}

pub fn compose_lines(mut input_file: &mut File) -> Vec<Line> {
    let lines = read_lines(&mut input_file);

    let mut current_address = DATA_SECTION_MIN_ADDRESS - WORD;
    let mut current_section = Section::NONE;

    lines
        .map(|line| {
            current_section = resolve_section(&line).unwrap_or_else(|| current_section.clone());
            match current_section {
                Section::DATA => {
                    let result = if resolve_section(&line).is_none() {
                        Line::new(Section::DATA, current_address, Some(line))
                    } else {
                        Line::new(Section::NONE, current_address, None)
                    };
                    current_address += WORD;
                    result
                }
                Section::TEXT => Line::new(Section::TEXT, current_address, Some(line)),
                Section::NONE => Line::new(Section::NONE, current_address, None),
            }
        })
        .collect::<Vec<Line>>()
}
