use crate::constants::INSTRUCTION_TABLE;
use crate::datum::{find_datum, Datum};
use crate::instruction::{convert_opcode_to_format, Instruction, InstructionFormat};
use crate::label::{find_label, Label};
use crate::utils::{convert_int_to_binary, convert_string_to_int, get_address_height};
use regex::Regex;

#[derive(Debug, Clone)]
enum ArgumentType {
    NUMBER,
    REGISTER,
    LABEL,
    STACK,
}

#[derive(Debug)]
pub struct Text {
    pub rs: i32,
    pub rt: i32,
    pub rd: i32,
    pub shamt: i32,
    pub funct: i32,
    pub opcode: i32,
    pub immediate: i32,
    pub address: i32,
}

impl Text {
    pub fn new(
        rs: i32,
        rt: i32,
        rd: i32,
        shamt: i32,
        funct: i32,
        opcode: i32,
        immediate: i32,
        address: i32,
    ) -> Self {
        Self {
            rs,
            rt,
            rd,
            shamt,
            funct,
            opcode,
            immediate,
            address,
        }
    }

    pub fn to_binary(&self) -> String {
        match convert_opcode_to_format(self.opcode) {
            InstructionFormat::REGISTER => format!(
                "{}{}{}{}{}{}",
                convert_int_to_binary(self.opcode, 6),
                convert_int_to_binary(self.rs, 5),
                convert_int_to_binary(self.rt, 5),
                convert_int_to_binary(self.rd, 5),
                convert_int_to_binary(self.shamt, 5),
                convert_int_to_binary(self.funct, 6),
            ),
            InstructionFormat::IMMEDIATE => format!(
                "{}{}{}{}",
                convert_int_to_binary(self.opcode, 6),
                convert_int_to_binary(self.rs, 5),
                convert_int_to_binary(self.rt, 5),
                convert_int_to_binary(self.immediate, 16),
            ),
            InstructionFormat::JUMP => format!(
                "{}{}",
                convert_int_to_binary(self.opcode, 6),
                convert_int_to_binary(self.address, 26),
            ),
            InstructionFormat::PSEUDO => panic!("A pseudo instruction found."),
        }
    }
}

pub fn get_text_from_code(
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
        panic!("Invalid instruction.");
    }
}

fn get_text_by_format(
    instruction: &Instruction,
    arguments: &Vec<i32>,
    current_address: i32,
) -> Text {
    let Instruction { funct, opcode, .. } = instruction;
    match convert_opcode_to_format(instruction.opcode) {
        InstructionFormat::REGISTER => match instruction.funct {
            0 | 2 => Text::new(
                0,
                arguments[1],
                arguments[0],
                arguments[2],
                *funct,
                *opcode,
                0,
                0,
            ),
            _ => Text::new(
                arguments[1],
                arguments[2],
                arguments[0],
                0,
                *funct,
                *opcode,
                0,
                0,
            ),
        },
        InstructionFormat::JUMP => Text::new(0, 0, 0, 0, *funct, *opcode, 0, arguments[0] >> 2),
        InstructionFormat::IMMEDIATE => {
            if arguments.len() < 3 {
                Text::new(0, arguments[0], 0, 0, *funct, *opcode, arguments[1], 0)
            } else {
                match instruction.opcode {
                    4 | 5 => Text::new(
                        arguments[0],
                        arguments[1],
                        0,
                        0,
                        *funct,
                        *opcode,
                        get_address_height(current_address, arguments[2]),
                        0,
                    ),
                    _ => Text::new(
                        arguments[0],
                        arguments[1],
                        0,
                        0,
                        *funct,
                        *opcode,
                        arguments[2],
                        0,
                    ),
                }
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
                        panic!("Failed to resolve argument value.");
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
                    panic!("Failed to resolve argument value.");
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
