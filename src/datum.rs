use crate::utils::{convert_int_to_binary, convert_string_to_int};

pub struct Datum {
    pub name: String,
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

pub fn resolve_data(code: &str, prev_datum_name: &Option<String>, address: i32) -> Option<Datum> {
    if let [name, _, value] = code.split('\t').collect::<Vec<&str>>()[..] {
        let value = convert_string_to_int(value);
        let name = name.trim_end_matches(':');
        if name.is_empty() {
            if let Some(prev_datum_name) = prev_datum_name {
                let name = format!("{}_{}", prev_datum_name, address);
                Some(Datum::new(&name, value, address))
            } else {
                panic!("Data name not found.")
            }
        } else {
            Some(Datum::new(name, value, address))
        }
    } else {
        None
    }
}
