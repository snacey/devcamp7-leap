// use super::anchor::SectionAnchor;
use super::entry::Section;
use super::entry::MAX_TITLE_LEN;
//use super::{
//    anchor::SectionAnchor,
//    entry::Section,
//    entry::{Section, MAX_TITLE_LEN},
//};
//use crate::anchor_trait::AnchorTrait;
use crate::helper;
use hdk::holochain_core_types::chain_header::ChainHeader;
// use hdk::{LinkValidationData, ValidationData};
use hdk::ValidationData;
use holochain_entry_utils::HolochainEntry;

pub fn create(entry: Section, _validation_data: ValidationData) -> Result<(), String> {
    // Can only validate based on MAX_TITLE_LEN unless teacher_address is added to Section (?)
    helper::validate_entity_title(&entry.title, &Section::entry_type(), MAX_TITLE_LEN)
}

pub fn modify(
    new_entry: Section,
    _old_entry: Section,
    _old_entry_header: ChainHeader,
    _validation_data: ValidationData,
) -> Result<(), String> {
    helper::validate_entity_title(&new_entry.title, &Section::entry_type(), MAX_TITLE_LEN)
}

pub fn delete(
    entry: Section,
    _entry_header: ChainHeader,
    _validation_data: ValidationData,
) -> Result<(), String> {
    helper::validate_entity_title(&entry.title, &Section::entry_type(), MAX_TITLE_LEN)
}
