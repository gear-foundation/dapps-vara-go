#![no_std]

use gstd::{msg, prelude::*, prog, ActorId, CodeId};
use template_io::*;

/// The code id of the domain program.
static mut DOMAIN_CODE_ID: Option<CodeId> = None;

/// The code id of the identity program.
static mut IDENTITY_CODE_ID: Option<CodeId> = None;

/// The resource of the domain.
static mut STATE: Option<Router> = None;

// Init handler for the factory program.
#[no_mangle]
extern fn init() {
    let payload: InitInput = msg::load().expect("Failed to load handle input");

    unsafe {
        *DOMAIN_CODE_ID
            .as_mut()
            .expect("Failed to get domain storage") = payload.domain;
        *IDENTITY_CODE_ID
            .as_mut()
            .expect("Failed to get identity storage") = payload.identity;
    }
}

fn domain_salt(sender: ActorId, domain: String) -> Vec<u8> {
    [sender.as_ref().to_vec(), domain.into_bytes().to_vec()].concat()
}

// TODO:
//
// Write methods:
//
// 1. create a new domain
// 2. create an identity
// 3. update a domain
// 4. update the identity
//
// Read methods:
//
// 1. Search domains
#[no_mangle]
extern fn handle() {
    let payload: Command = msg::load().expect("Failed to load handle input");
    let sender = msg::source();

    match payload {
        Command::CreateDomain(config) => {
            let code_id = unsafe { DOMAIN_CODE_ID.clone().expect("code id not exist") };

            let (_, pid) = prog::create_program(
                code_id,
                domain_salt(sender, config.domain.clone()),
                config.source,
                0,
            )
            .expect("Failed to create domain");

            unsafe {
                STATE
                    .as_mut()
                    .expect("Failed to load program state")
                    .create_domain(config.domain, pid)
            }
        }
        Command::AddLabels(config) => unsafe {
            // TODO: admin checks

            STATE
                .as_mut()
                .expect("Failed to load program state")
                .add_labels(config.domain, config.labels)
        },
    }
}

// The `state()` entry point.
#[no_mangle]
extern fn state() {
    let state = unsafe { STATE.take().expect("State isn't initialized") };
    msg::reply(state, 0).expect("Failed to reply from `state()`");
}
