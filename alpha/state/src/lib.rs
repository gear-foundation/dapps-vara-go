#![no_std]

use alpha_io::{Label, Resource};
use gstd::prelude::*;

#[gmeta::metawasm]
pub mod metafns {
    pub type State = alpha_io::State;

    /// Returns all domains (pages) that matches the search input.
    pub fn labels(_state: State) -> Vec<Label> {
        Label::list()
    }

    /// Search all resouces matching the input label.
    pub fn search(state: State, label: Label) -> Vec<Resource> {
        state
            .into_iter()
            .filter(|s| s.labels.contains(&label))
            .collect()
    }
}
