//! Types for the router

use crate::Source;
use gstd::{collections::BTreeMap, ActorId, CodeId, Decode, Encode, String, TypeInfo, Vec};

/// The init input for the router program.
#[derive(Encode, Decode, TypeInfo, PartialEq, Eq, Debug, Clone)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct InitInput {
    /// The code id of the identity program.
    pub identity: CodeId,

    /// The code id of the domain program.
    pub domain: CodeId,
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

/// Available labels
#[derive(Encode, Decode, TypeInfo, PartialEq, Eq, Debug, Clone)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum Label {
    Vara,
    Profile,
}

/// Metadata of domain.
#[derive(Encode, Decode, TypeInfo, PartialEq, Eq, Debug, Clone)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct DomainMeta {
    pub labels: Vec<Label>,
    pub pid: ActorId,
}

/// Router state
#[derive(Encode, Decode, TypeInfo, PartialEq, Eq, Debug, Clone)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct Router(pub BTreeMap<String, DomainMeta>);

impl Router {
    /// Create a dummy domain
    pub fn create_domain(&mut self, domain: String, pid: ActorId) {
        self.0.insert(
            domain,
            DomainMeta {
                pid,
                labels: Default::default(),
            },
        );
    }
}
