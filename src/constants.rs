use crate::models::Instruction;

pub const INSTRUCTION_NUM: usize = 21;

pub const INSTRUCTION_TABLE: [Instruction; INSTRUCTION_NUM] = [
    Instruction::new("addiu", 0x9, -1),
    Instruction::new("addu", 0x0, 0x21),
    Instruction::new("and", 0x0, 0x24),
    Instruction::new("andi", 0xc, -1),
    Instruction::new("beq", 0x4, -1),
    Instruction::new("bne", 0x5, -1),
    Instruction::new("j", 0x2, -1),
    Instruction::new("jal", 0x3, -1),
    Instruction::new("jr", 0, 0x8),
    Instruction::new("lui", 0xf, -1),
    Instruction::new("lw", 0x23, -1),
    Instruction::new("la", -1, -1),
    Instruction::new("nor", 0, 0x27),
    Instruction::new("or", 0, 0x25),
    Instruction::new("ori", 0xd, -1),
    Instruction::new("sltiu", 0xb, -1),
    Instruction::new("sltu", 0, 0x2b),
    Instruction::new("sll", 0, 0x0),
    Instruction::new("srl", 0, 0x2),
    Instruction::new("sw", 0x2b, -1),
    Instruction::new("subu", 0, 0x23),
];

pub const WORD: i32 = 4;
