use crate::Source;
use gstd::{Decode, Encode, String, TypeInfo};

/// The input for the `handle` entry point.
#[derive(Encode, Decode, TypeInfo, PartialEq, Eq, Debug, Clone)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct HandleInput {
    pub domain: String,
    pub src: Source,
}
