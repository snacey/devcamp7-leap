use hdk::{
    entry_definition::ValidatingEntryType,
    holochain_core_types::{dna::entry_types::Sharing, validation::EntryValidationData},
    holochain_json_api::{error::JsonError, json::JsonString},
    holochain_persistence_api::cas::content::Address,
};
use holochain_entry_utils::HolochainEntry;

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct Content {
    pub title: String,
    pub url: String,
    pub description: String,
    pub timestamp: u64,
    pub section_anchor_address: Address,
}

impl Content {
    pub fn new(
        title: String,
        url: String,
        description: String,
        timestamp: u64,
        section_anchor_address: Address,
    ) -> Self {
        Content {
            title: title,
            url: url,
            description: description,
            timestamp: timestamp,
            section_anchor_address: section_anchor_address,
        }
    }
}

impl HolochainEntry for Content {
    fn entry_type() -> String {
        String::from("content")
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
        links: []
    )
}
