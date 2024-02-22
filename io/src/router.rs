//! Types for the router

use crate::Source;
use gstd::{ActorId, Decode, Encode, String, TypeInfo};

/// The init input for the router program.
#[derive(Encode, Decode, TypeInfo, PartialEq, Eq, Debug, Clone)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct InitInput {
    /// The code id of the identity program.
    pub identity: ActorId,

    /// The code id of the domain program.
    pub domain: ActorId,
}

/// Avaiable commands for the router program.
#[derive(Encode, Decode, TypeInfo, PartialEq, Eq, Debug, Clone)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum Command {
    CreateDomain(CommandCreateDomain),
    // TODO:
    //
    // CreateIdentity,
}

/// The command to create a new domain.
#[derive(Encode, Decode, TypeInfo, PartialEq, Eq, Debug, Clone)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct CommandCreateDomain {
    /// The domain name.
    pub domain: String,
    /// The source of the domain.
    pub source: Source,
}

// TODO:
//
// /// The command to create a new domain.
// pub struct CommandCreateIdentity {
//
// }
