#![no_std]

use gstd::prelude::*;

#[gmeta::metawasm]
pub mod metafns {

    pub type State = template_io::Source;

    /// Returns all domains (pages) that matches the search input.
    pub fn resource(state: State) -> template_io::Source {
        state
    }
}
