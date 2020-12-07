use crate::text::Text;

pub enum InstructionFormat {
    REGISTER,
    IMMEDIATE,
    JUMP,
    PSEUDO,
}

pub struct Instruction {
    pub name: &'static str,
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

    pub fn is_branch(&self) -> bool {
        self.opcode == 4 || self.opcode == 5
    }

    pub fn is_shift(&self) -> bool {
        self.funct == 0 || self.funct == 2
    }

    pub fn is_register_jump(&self) -> bool {
       self.funct == 8
    }

    pub fn to_register_format_text(&self, rs: i32, rt: i32, rd: i32, shamt: i32) -> Text {
        Text::new(rs, rt, rd, shamt, self.funct, self.opcode, 0, 0)
    }

    pub fn to_jump_format_text(&self, address: i32) -> Text {
        Text::new(0, 0, 0, 0, self.funct, self.opcode, 0, address)
    }

    pub fn to_immediate_format_text(&self, rs: i32, rt: i32, immediate: i32) -> Text {
        Text::new(rs, rt, 0, 0, self.funct, self.opcode, immediate, 0)
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
