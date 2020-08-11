// allowing for this Rust project to have dead code on a crate level
#![allow(dead_code)]
// unstable Rust feature
// See more at: https://doc.rust-lang.org/nightly/unstable-book/language-features/proc-macro-hygiene.html
#![feature(proc_macro_hygiene)]
// This isn't a mistake that there are multiple #[macro_use] below: each applies to a particular crate that follows it
// specifying that we want to import macros defined in this crate too.
// See more at: https://doc.rust-lang.org/reference/macros-by-example.html
#[macro_use]
extern crate hdk;
extern crate hdk_proc_macros;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
extern crate holochain_json_derive;

use hdk::prelude::*;

use hdk_proc_macros::zome;

mod anchor_trait;
mod content;
mod course;
mod section;
mod helper;

#[zome]
mod courses {

    // Things to be done on an hApp init, we skip this for now
    #[init]
    fn init() {
        Ok(())
    }

    // Things to be done to validate each agent in the network, we skip this for now
    #[validate_agent]
    pub fn validate_agent(validation_data: EntryValidationData<AgentId>) {
        Ok(())
    }

    //  ====================== Course definitions
    #[entry_def]
    fn course_catalog_anchor_entry_definition() -> ValidatingEntryType {
        course::catalog_anchor::catalog_anchor_entry_def()
    }

    #[entry_def]
    fn course_anchor_definition() -> ValidatingEntryType {
        course::anchor::course_anchor_def()
    }

    #[entry_def]
    fn course_entry_definition() -> ValidatingEntryType {
        course::entry::course_entry_def()
    }

    #[zome_fn("hc_public")]
    fn create_course(title: String, timestamp: u64) -> ZomeApiResult<Address> {
        course::handlers::create(title, timestamp)
    }

    #[zome_fn("hc_public")]
    fn get_latest_course_entry(
        course_anchor_address: Address,
    ) -> ZomeApiResult<Option<course::entry::Course>> {
        let latest_course_result = course::handlers::get_latest_course(&course_anchor_address)?;
        match latest_course_result {
            Some((course_entry, _course_entry_address)) => {
                return Ok(Some(course_entry));
            }
            None => return Ok(None),
        }
    }

    #[zome_fn("hc_public")]
    fn update_course(
        title: String,
        sections_addresses: Vec<Address>,
        course_anchor_address: Address,
    ) -> ZomeApiResult<Address> {
        course::handlers::update(title, sections_addresses, &course_anchor_address)
    }

    #[zome_fn("hc_public")]
    fn delete_course(course_anchor_address: Address) -> ZomeApiResult<Address> {
        course::handlers::delete(course_anchor_address)
    }

    #[zome_fn("hc_public")]
    fn get_all_courses() -> ZomeApiResult<Vec<Address>> {
        course::handlers::list_all_courses()
    }

    #[zome_fn("hc_public")]
    fn get_my_courses() -> ZomeApiResult<Vec<Address>> {
        course::handlers::get_my_courses()
    }

    #[zome_fn("hc_public")]
    fn get_my_enrolled_courses() -> ZomeApiResult<Vec<Address>> {
        course::handlers::get_my_enrolled_courses()
    }

    // Section
    // TODO: implement section entry definitions
    #[entry_def]
    fn section_anchor_entry_definition() -> ValidatingEntryType {
        section::section_anchor::section_anchor_entry_def()
    }
    // TODO: implement section CRUD methods

    #[entry_def]
    fn section_entry_definition() -> ValidatingEntryType {
        section::section::section_entry_def()
    }

    // Content
    // TODO: implement content entry definition
    #[entry_def]
    fn content_entry_definition() -> ValidatingEntryType {
        content::content::content_entry_def()
    }
    // TODO: implement content CRUD methods
}
