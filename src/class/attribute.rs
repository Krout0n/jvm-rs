use super::common_type::*;
use super::instruction::Instruction;

#[derive(Debug, PartialEq)]
pub enum AttributeInfoKind {
    Code {
        max_stack: u2,
        max_locals: u2,
        code_length: u4,
        code: Vec<Instruction>,
        exception_table_length: u2,
        exception_table: Vec<ExceptionTable>,
        attributes_count: u2,
        attributes: Vec<AttributeInfo>,
    },
    LineNumberTable {
        line_number_table_length: u2,
        line_number_table: Vec<LineNumberTable>,
    },
    // u2 sourcefile_index;
    SourceFile(u2),
}

#[derive(Debug, PartialEq)]
pub struct LineNumberTable {
    pub start_pc: u2,
    pub line_number: u2,
}

#[derive(Debug, PartialEq)]
pub struct ExceptionTable {
    pub start_pc: u2,
    pub end_pc: u2,
    pub handler_pc: u2,
    pub catch_type: u2,
}

#[derive(Debug, PartialEq)]
pub struct AttributeInfo {
    pub attribute_name_index: u2,
    pub attribute_length: u4,
    // u1 info[attribute_length];
    pub info: AttributeInfoKind,
}
