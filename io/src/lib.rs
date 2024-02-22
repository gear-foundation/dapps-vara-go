#![no_std]

use gmeta::{In, Metadata, Out};
pub use router::{Command, InitInput, Router};
pub use source::{Footer, Header, Source, State};

mod router;
mod source;

/// The contract metadata. Used by frontend apps & for describing the types of messages that can be
/// sent in contract's entry points. See also [`Metadata`].
pub struct ContractMetadata;

/// `()` means the contract doesn't process & reply messages at the above written entry point or
/// doesn't implement it.
impl Metadata for ContractMetadata {
    /// I/O types for the `init()` entry point.
    type Init = In<InitInput>;
    /// I/O types for the `handle()` entry point.
    type Handle = In<Command>;
    /// Types for miscellaneous scenarios.
    type Others = ();
    /// The input type for the `handle_reply()` entry point.
    type Reply = ();
    /// The output type for the `handle_signal()` entry point.
    type Signal = ();
    /// I/O types for the `state()` entry point.
    ///
    /// You can also specify just an output ([`Out`]) or input ([`In`]) type, if both
    /// ([`In`]) are expected like here.
    type State = Out<Router>;
}
