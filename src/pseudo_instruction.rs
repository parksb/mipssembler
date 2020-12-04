use crate::constants::DATA_SECTION_MIN_ADDRESS;
use crate::datum::{find_datum, Datum};

pub fn disassemble_pseudo_instruction(code: &str, data: &[Datum]) -> Option<Vec<String>> {
    if let [name, arguments] = code.trim_start().split('\t').collect::<Vec<&str>>()[..] {
        match name {
            "la" => Some(la(arguments, &data)),
            _ => None,
        }
    } else {
        panic!("Invalid instruction.")
    }
}

fn la(arguments: &str, data: &[Datum]) -> Vec<String> {
    let mut result: Vec<String> = vec![];
    let argument_text = arguments
        .split(',')
        .map(|arg| arg.trim())
        .collect::<Vec<&str>>();

    if let [register, datum_name] = argument_text[..] {
        if let Some(datum) = find_datum(datum_name, &data) {
            let shifted_datum_address = datum.address >> 16;
            result.push(format!("lui\t{}, {}", register, shifted_datum_address));

            if (datum.address << 16) > 0 {
                let address = datum.address - DATA_SECTION_MIN_ADDRESS;
                result.push(format!("ori\t{}, {}, {}", register, register, address));
            }
        } else {
            panic!("Use of undeclared data.");
        }
    } else {
        panic!("Failed to parse arguments.");
    }

    result
}
