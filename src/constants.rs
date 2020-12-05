use phf::{phf_map, Map};

use crate::instruction::Instruction;

pub const INSTRUCTION_TABLE: Map<&'static str, &'static Instruction> = phf_map! {
 "addiu" => &Instruction::new("addiu", 0x9, -1),
 "addu" => &Instruction::new("addu", 0x0, 0x21),
 "and" => &Instruction::new("and", 0x0, 0x24),
 "andi" => &Instruction::new("andi", 0xc, -1),
 "beq" => &Instruction::new("beq", 0x4, -1),
 "bne" => &Instruction::new("bne", 0x5, -1),
 "j" => &Instruction::new("j", 0x2, -1),
 "jal" => &Instruction::new("jal", 0x3, -1),
 "jr" => &Instruction::new("jr", 0, 0x8),
 "lui" => &Instruction::new("lui", 0xf, -1),
 "lw" => &Instruction::new("lw", 0x23, -1),
 "la" => &Instruction::new("la", -1, -1),
 "nor" => &Instruction::new("nor", 0, 0x27),
 "or" => &Instruction::new("or", 0, 0x25),
 "ori" => &Instruction::new("ori", 0xd, -1),
 "sltiu" => &Instruction::new("sltiu", 0xb, -1),
 "sltu" => &Instruction::new("sltu", 0, 0x2b),
 "sll" => &Instruction::new("sll", 0, 0x0),
 "srl" => &Instruction::new("srl", 0, 0x2),
 "sw" => &Instruction::new("sw", 0x2b, -1),
 "subu" => &Instruction::new("subu", 0, 0x23),
};

pub const WORD: i32 = 4;

pub const DATA_SECTION_MIN_ADDRESS: i32 = 0x10000000;
pub const TEXT_SECTION_MIN_ADDRESS: i32 = 0x400000;
