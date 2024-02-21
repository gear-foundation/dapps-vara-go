#![no_std]

use gstd::{collections::BTreeMap, prelude::*, Vec};
use template_io::*;

#[gmeta::metawasm]
pub mod metafns {

    pub type State = template_io::State;

    /// Returns all domains (pages) that matches the search input.
    ///
    /// For the source of the competition:
    /// - domain name
    /// - labels
    /// - header title
    /// - content
    pub fn search(state: State, input: String) -> BTreeMap<String, Source> {
        let tokens: Vec<&str> = input.split_whitespace().collect();

        state
            .iter()
            .filter_map(|(domain, source)| {
                if tokens.iter().any(|t| {
                    domain.contains(t)
                        || source.labels.iter().any(|l| l.contains(t))
                        || source.header.title.contains(t)
                        || source.content.contains(t)
                }) {
                    return Some((domain.clone(), source.clone()));
                }
                None
            })
            .collect()
    }

    /// List all labels
    pub fn labels(state: State) -> Vec<String> {
        let mut labels: Vec<String> = state
            .values()
            .map(|s| s.labels.clone())
            .collect::<Vec<Vec<String>>>()
            .into_iter()
            .flatten()
            .collect();
        labels.sort();
        labels.dedup();
        labels
    }

    // TODO:
    //
    // 1) list sub paths of a domain.
    // 2)
}
