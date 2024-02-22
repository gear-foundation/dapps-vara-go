#![no_std]

use gstd::{collections::BTreeMap, prelude::*, ActorId, Vec};

#[gmeta::metawasm]
pub mod metafns {

    pub type State = template_io::Router;

    /// Returns all domains (pages) that matches the search input.
    pub fn search(state: State, input: String) -> BTreeMap<String, ActorId> {
        let tokens: Vec<&str> = input.split_whitespace().collect();

        state
            .0
            .iter()
            .filter_map(|(domain, meta)| {
                if tokens.iter().any(|t| meta.labels().contains(t)) {
                    return Some((domain.clone(), meta.pid));
                }
                None
            })
            .collect()
    }

    // TODO:
    //
    // 1) list sub paths of a domain.
    // 2)
}
