#![no_std]

use gmeta::{In, Metadata, Out};

mod handler;
mod source;

pub use handler::{HandleInput, HandleOutput};
pub use source::{Content, Footer, Header, Profile, Source, State};

/// The contract metadata. Used by frontend apps & for describing the types of messages that can be
/// sent in contract's entry points. See also [`Metadata`].
pub struct ContractMetadata;

/// `()` means the contract doesn't process & reply messages at the above written entry point or
/// doesn't implement it.
impl Metadata for ContractMetadata {
    /// I/O types for the `init()` entry point.
    type Init = ();
    /// I/O types for the `handle()` entry point.
    type Handle = In<HandleInput>;
    /// Types for miscellaneous scenarios.
    type Others = ();
    /// The input type for the `handle_reply()` entry point.
    type Reply = ();
    /// The output type for the `handle_signal()` entry point.
    type Signal = ();
    /// I/O types for the `state()` entry point.
    ///
    /// You can also specify just an output ([`Out`]) or input ([`In`](gmeta::In)) type, if both
    /// ([`InOut`]) are expected like here.
    type State = Out<State>;
}
