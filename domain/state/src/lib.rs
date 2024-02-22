#![no_std]

use gstd::{collections::BTreeMap, prelude::*, ActorId, Vec};

#[gmeta::metawasm]
pub mod metafns {

    pub type State = template_io::Source;

    /// Returns all domains (pages) that matches the search input.
    pub fn resource(state: State) -> template_io::Source {
        state
    }
}
