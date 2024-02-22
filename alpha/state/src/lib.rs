#![no_std]

use alpha_io::{Resource, State};
use gstd::prelude::*;

#[gmeta::metawasm]
pub mod metafns {
    pub type State = State;

    /// Returns all domains (pages) that matches the search input.
    pub fn labels(state: State) -> Vec<Label> {
        Label::list()
    }

    /// Search all resouces matching labels
    pub fn search(state: State, label: Label) -> Vec<Resource> {
        state.iter().filter(|s| s.labels.contains(label)).collect()
    }
}
