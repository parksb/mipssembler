pub struct Instruction {
    name: &'static str,
    pub opcode: i32,
    pub funct: i32,
}

impl Instruction {
    pub const fn new(name: &'static str, opcode: i32, funct: i32) -> Self {
        Self {
            name,
            opcode,
            funct,
        }
    }

    pub fn compare_name(&self, name: &str) -> bool {
        self.name == name
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
    rs: i32,
    rt: i32,
    rd: i32,
    shamt: i32,
    funct: i32,
    opcode: i32,
    immediate: i32,
    address: i32,
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

#[derive(Debug, Clone)]
pub enum ArgumentType {
    NUMBER,
    REGISTER,
    LABEL,
    STACK,
}

pub enum InstructionFormat {
    REGISTER,
    IMMEDIATE,
    JUMP,
    PSEUDO,
}
