#![no_std]

use gstd::{collections::BTreeMap, prelude::*};
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

    // TODO:
    //
    // 1) list subpages of a domain.
    // 2)

    // pub fn query(state: State, query: StateQuery) -> StateQueryReply {
    //     match query {
    //         StateQuery::Pingers => StateQueryReply::Pingers(pingers(state)),
    //         StateQuery::PingCount(actor) => StateQueryReply::PingCount(ping_count(state, actor)),
    //     }
    // }
    //
    // pub fn pingers(state: State) -> Vec<ActorId> {
    //     state.iter().map(|(pinger, _)| *pinger).collect()
    // }
    //
    // pub fn ping_count(state: State, actor: ActorId) -> u128 {
    //     state
    //         .iter()
    //         .find_map(|(some_actor, count)| (some_actor == &actor).then_some(count))
    //         .copied()
    //         .unwrap_or_default()
    // }
}
