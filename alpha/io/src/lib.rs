#![no_std]

use gmeta::{In, Metadata, Out};
use gstd::*;

/// The contract metadata. Used by frontend apps & for describing the types of messages that can be
/// sent in contract's entry points. See also [`Metadata`].
pub struct ContractMetadata;

pub type State = Vec<Resource>;

#[derive(Encode, Decode, TypeInfo, PartialEq, Eq, Debug, Clone)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum Label {
    Apple,
    Banana,
    Football,
    Vara,
}

impl AsRef<str> for Label {
    fn as_ref(&self) -> &str {
        match self {
            Label::Apple => "apple",
            Label::Banana => "banana",
            Label::Football => "football",
            Label::Vara => "vara",
        }
    }
}

impl Label {
    pub fn list() -> Vec<Label> {
        vec![Label::Apple, Label::Banana, Label::Football, Label::Vara]
    }
}

#[derive(Encode, Decode, TypeInfo, PartialEq, Eq, Debug, Clone)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct Resource {
    pub labels: Vec<Label>,
    pub domain: String,
    pub description: String,
    pub html: String,
    pub styles: String,
}

/// `()` means the contract doesn't process & reply messages at the above written entry point or
/// doesn't implement it.
impl Metadata for ContractMetadata {
    /// I/O types for the `init()` entry point.
    type Init = In<()>;
    /// I/O types for the `handle()` entry point.
    type Handle = In<Resource>;
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
    type State = Out<State>;
}
