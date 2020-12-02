pub struct InstructionTable {
    name: &'static str,
    pointer: i32,
    sequence: [i32; 3],
    opcode: i32,
    funct: i32,
    is_occupied: bool,
}

impl InstructionTable {
    pub const fn new(
        name: &'static str,
        pointer: i32,
        sequence: [i32; 3],
        opcode: i32,
        funct: i32,
        is_occupied: bool,
    ) -> Self {
        Self {
            name,
            pointer,
            sequence,
            opcode,
            funct,
            is_occupied,
        }
    }
}

#[derive(Debug)]
pub struct Datum {
    name: String,
    value: i32,
    address: i32,
}

impl Datum {
    pub fn new(name: &str, value: i32, address: i32) -> Self {
        Self {
            name: name.to_string(),
            value,
            address,
        }
    }
}

pub struct Text {
    next: i32,
    instruction: String,
    opcode: i32,
    funct: i32,
    address: i32,
    arguments: [i32; 4],
    is_label: bool,
}

#[derive(Debug)]
pub struct Label {
    name: String,
    address: i32,
    new_address: i32,
}

impl Label {
    pub fn new(name: &str, address: i32, new_address: i32) -> Self {
        Self {
            name: name.to_string(),
            address,
            new_address,
        }
    }

    pub fn set_address_to_negative_one(&mut self) {
        self.address = -1;
    }
}

#[derive(Debug)]
pub enum Section {
    NONE,
    DATA,
    TEXT,
}
