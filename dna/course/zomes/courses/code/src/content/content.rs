use hdk::{
    entry_definition::ValidatingEntryType,
    holochain_core_types::{dna::entry_types::Sharing, validation::EntryValidationData},
    holochain_json_api::{error::JsonError, json::JsonString},
    //   holochain_persistence_api::cas::content::Address,
};
use holochain_entry_utils::HolochainEntry;

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct Content {
    pub title: String,
    pub timestamp: u64,
}

impl HolochainEntry for Content {
    fn entry_type() -> String {
        String::from("content")
    }
}

impl Content {
    pub fn new(title: String, timestamp: u64) -> Self {
        Content {
            title: title,
            timestamp: timestamp,
        }
    }
}

// Holochain entry definition for Content
pub fn content_entry_def() -> ValidatingEntryType {
    entry!(
        name: Content::entry_type(),
        description: "this is the definition of section content",
        sharing: Sharing::Public,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },
        validation: | validation_data: hdk::EntryValidationData<Content>| {
            match validation_data {
                EntryValidationData::Create { .. } => {
                    Ok(())
                },
                EntryValidationData::Modify { .. } => {
                    Ok(())
                },
                EntryValidationData::Delete { .. } => {
                    Ok(())
                }
            }
        },
        // All links that content should have are defined for SectionAnchor and so this entry doesn't have any
        links: []
    )
}
