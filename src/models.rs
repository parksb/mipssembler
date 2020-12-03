use regex::Regex;

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

    pub fn compare_name(&self, name: &str) -> bool {
        self.name == name
    }

    pub fn to_text(&self, next: i32, address: i32, arguments: Vec<i32>, is_label: bool) -> Text {
        Text {
            next,
            instruction: self.name.to_string(),
            opcode: self.opcode,
            funct: self.funct,
            address,
            arguments,
            is_label,
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

    pub fn compare_name(&self, name: &str) -> bool {
        self.name == name
    }

    pub fn get_address(&self) -> i32 {
        self.address
    }
}

#[derive(Debug)]
pub struct Text {
    next: i32,
    instruction: String,
    opcode: i32,
    funct: i32,
    address: i32,
    arguments: Vec<i32>,
    is_label: bool,
}

impl Text {
    pub fn new(instruction: &str, opcode: i32, funct: i32, address: i32) -> Self {
        Self {
            next: 0,
            instruction: instruction.to_string(),
            opcode,
            funct,
            address,
            arguments: vec![],
            is_label: false,
        }
    }

    pub fn set_is_label(&mut self, is_label: bool) {
        self.is_label = is_label;
    }

    pub fn set_arguments(&mut self, arguments: Vec<i32>) {
        self.arguments = arguments;
    }
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

    pub fn compare_name(&self, name: &str) -> bool {
        self.name == name.to_string()
    }

    pub fn get_address(&self) -> i32 {
        self.address
    }
}

#[derive(Debug)]
pub enum Section {
    NONE,
    DATA,
    TEXT,
}

#[derive(Clone)]
pub struct Argument {
    pub regex: Regex,
    pub data_type: ArgumentType,
}

impl Argument {
    pub fn new(regex: Regex, data_type: ArgumentType) -> Self {
        Argument { regex, data_type }
    }
}

#[derive(Debug, Clone)]
pub enum ArgumentType {
    NUMBER,
    REGISTER,
    LABEL,
    STACK,
}

#[derive(Debug)]
pub enum InstructionFormat {
    R,
    I,
    J,
}
