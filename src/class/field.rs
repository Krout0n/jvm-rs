use super::attribute::AttributeInfo;
use super::common_type::*;

#[derive(Debug)]
pub struct FieldInfo {
    pub access_flags: u2,
    pub name_index: u2,
    pub descriptor_index: u2,
    pub attributes_count: u2,
    pub attributes: Vec<AttributeInfo>,
}
