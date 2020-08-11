use hdk::prelude::*;
use hdk::{entry_definition::ValidatingEntryType, holochain_core_types::dna::entry_types::Sharing};

use crate::anchor_trait::AnchorTrait;
use crate::content::content::Content;
use crate::section::section::Section;
use holochain_entry_utils::HolochainEntry;

pub const SECTION_TO_CONTENT_LINK: &str = "section_anchor->content";

#[derive(Serialize, Deserialize, Debug, self::DefaultJson, Clone)]
pub struct SectionAnchor {
    title: String,
    course_anchor_address: Address,
    pub timestamp: u64,
}

impl AnchorTrait for SectionAnchor {
    fn entry_type() -> String {
        String::from("section_anchor")
    }
    fn link_to() -> String {
        Section::entry_type()
    }
    fn link_type() -> String {
        "section_anchor->section".to_string()
    }
}

impl SectionAnchor {
    pub fn new(title: String, course_anchor_address: Address, timestamp: u64) -> Self {
        SectionAnchor {
            title: title,
            course_anchor_address: course_anchor_address,
            timestamp: timestamp,
        }
    }
}

pub fn section_anchor_entry_def() -> ValidatingEntryType {
    entry!(
        name: SectionAnchor::entry_type(),
        description:"Anchor to a valid course section",
        sharing: Sharing::Public,
        validation_package:||{
            hdk::ValidationPackageDefinition::Entry
        },
        validation:|_validation_data: hdk::EntryValidationData<SectionAnchor>|{
            Ok(())
        },
        links:[
            // link that connects SectionAnchor to the latest Section entry
            // This is a necessary link that allows access to section data
            to!(
                SectionAnchor::link_to(),
                link_type: SectionAnchor::link_type(),
                validation_package:||{
                    hdk::ValidationPackageDefinition::Entry
                },
                validation:|_validation_data: hdk::LinkValidationData|{
                    Ok(())
                }
            ),
            to!(
                Content::entry_type(),
                link_type: SECTION_TO_CONTENT_LINK,
                validation_package:||{
                    hdk::ValidationPackageDefinition::Entry
                },
                validation:|_validation_data: hdk::LinkValidationData|{
                    Ok(())
                }
            )
        ]
    )
}
