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
    pub fn search(state: State, input: String) -> BTreeMap<String, BTreeMap<String, Source>> {
        let tokens: Vec<&str> = input.split_whitespace().collect();

        // TODO: make this iter human readable.
        state
            .iter()
            .map(|(domain, v)| {
                (
                    domain.clone(),
                    v.paths
                        .iter()
                        .filter_map(|(path, source)| {
                            if tokens.iter().any(|t| {
                                source.labels.iter().any(|l| l.contains(t))
                                    || source.header.title.contains(t)
                                    || source.content.contains(t)
                            }) {
                                return Some((path.clone(), source.clone()));
                            }
                            None
                        })
                        .collect(),
                )
            })
            .collect()
    }

    /// List all labels
    ///
    /// NOTE: This is for completing the search inputs.
    pub fn labels(state: State) -> Vec<String> {
        let mut labels: Vec<String> = state
            .values()
            .map(|s| {
                s.paths
                    .values()
                    .map(|p| p.labels.clone())
                    .flatten()
                    .collect()
            })
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
