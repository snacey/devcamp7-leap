use hdk::{
    entry_definition::ValidatingEntryType,
    holochain_core_types::{dna::entry_types::Sharing, validation::EntryValidationData},
    holochain_json_api::{error::JsonError, json::JsonString},
    holochain_persistence_api::cas::content::Address,
};
use holochain_entry_utils::HolochainEntry;

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct Section {
    pub title: String,
    pub section_anchor: Address,
    pub timestamp: u64,
}

impl HolochainEntry for Section {
    fn entry_type() -> String {
        String::from("section")
    }
}

impl Section {
    pub fn new(title: String, section_anchor: Address, timestamp: u64) -> Self {
        Section {
            title: title,
            section_anchor: section_anchor,
            timestamp: timestamp,
        }
    }
}

// Holochain entry definition for Section
pub fn section_entry_def() -> ValidatingEntryType {
    entry!(
        name: Section::entry_type(),
        description: "this is the definition of section",
        sharing: Sharing::Public,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },
        validation: | validation_data: hdk::EntryValidationData<Section>| {
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
        // All links that course should have are defined for SectionAnchor and so this entry doesn't have any
        links: []
    )
}
