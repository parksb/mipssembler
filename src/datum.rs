use crate::utils::{convert_int_to_binary, convert_string_to_int};

pub struct Datum {
    name: String,
    value: i32,
    pub address: i32,
}

impl Datum {
    pub fn new(name: &str, value: i32, address: i32) -> Self {
        Self {
            name: name.to_string(),
            value,
            address,
        }
    }

    pub fn to_binary(&self) -> String {
        convert_int_to_binary(self.value, 32)
    }
}

pub fn find_datum<'a>(name: &'a str, data: &'a [Datum]) -> Option<&'a Datum> {
    data.iter().find(|datum| datum.name == name)
}

pub fn resolve_data(code: &str, current_address: i32) -> Option<Datum> {
    if let [name, meta] = code.split(":\t").collect::<Vec<&str>>()[..] {
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
