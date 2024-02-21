use crate::Source;

/// The input for the `handle` entry point.
#[derive(Encode, Decode, TypeInfo, PartialEq, Eq, Debug)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct HandleInput {
    pub id: String,
    pub src: Source,
}

/// The output for the `handle` entry point.
#[derive(Encode, Decode, TypeInfo, PartialEq, Eq, Debug)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum HandleOutput {
    Success,
    InvalidPayload,
}