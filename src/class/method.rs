use super::access_flags::AccessFlags;
use super::attribute::AttributeInfo;
use super::common_type::*;

#[derive(Clone, Debug)]
pub struct MethodInfo {
    pub access_flags: AccessFlags,
    pub name_index: u2,
    pub descriptor_index: u2,
    pub attributes_count: u2,
    pub attributes: Vec<AttributeInfo>,
}
