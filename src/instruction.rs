pub enum InstructionFormat {
    REGISTER,
    IMMEDIATE,
    JUMP,
    PSEUDO,
}

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

pub fn convert_opcode_to_format(opcode: i32) -> InstructionFormat {
    match opcode {
        0 => InstructionFormat::REGISTER,
        2 | 3 => InstructionFormat::JUMP,
        -1 => InstructionFormat::PSEUDO,
        _ => InstructionFormat::IMMEDIATE,
    }
}
