use super::validation;
use hdk::{
    entry_definition::ValidatingEntryType,
    holochain_core_types::{dna::entry_types::Sharing, validation::EntryValidationData},
    holochain_json_api::{error::JsonError, json::JsonString},
    holochain_persistence_api::cas::content::Address,
};
use holochain_entry_utils::HolochainEntry;

pub const MAX_TITLE_LEN: usize = 50;

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct Course {
    pub title: String,
    pub sections: Vec<Address>,
    pub teacher_address: Address,
    pub timestamp: u64,
    pub anchor_address: Address,
}

impl HolochainEntry for Course {
    fn entry_type() -> String {
        String::from("course")
    }
}

impl Course {
    pub fn new(
        title: String,
        sections: Vec<Address>,
        teacher_address: Address,
        timestamp: u64,
        anchor_address: Address,
    ) -> Self {
        Course {
            title: title,
            sections: sections,
            teacher_address: teacher_address,
            timestamp: timestamp,
            anchor_address: anchor_address,
        }
    }
}

// Holochain entry definition for Course
pub fn course_entry_def() -> ValidatingEntryType {
    entry!(
        name: Course::entry_type(),
        description: "this is the definition of course",
        sharing: Sharing::Public,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },
        validation: | validation_data: hdk::EntryValidationData<Course>| {
            match validation_data {
                EntryValidationData::Create { entry, validation_data } => {
                    validation::create(entry, validation_data)
                },
                EntryValidationData::Modify { new_entry, old_entry, old_entry_header, validation_data } => {
                    validation::modify(new_entry, old_entry, old_entry_header, validation_data)
                },
                EntryValidationData::Delete { old_entry, old_entry_header, validation_data } => {
                    validation::delete(old_entry, old_entry_header, validation_data)
                }
            }
        },
        // All links that course should have are defined for CoureAnchor and so this entry doesn't have any
        links: []
    )
}
