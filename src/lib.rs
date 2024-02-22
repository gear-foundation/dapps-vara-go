#![no_std]

use gstd::{msg, prelude::*, prog, CodeId};
use template_io::*;

static mut DOMAIN_CODE_ID: Option<CodeId> = None;

// static mut IDENTITY_CODE_ID: Option<CodeId> = None;

/// The resource of the domain.
static mut STATE: Option<State> = None;

// Init handler for the factory program.
#[no_mangle]
extern fn init() {
    // TODO:
    //
    // 1. the code id of the identity program
    // 2. the code id of the domain program
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
            let salt = [
                sender.as_ref().to_vec(),
                config.domain.into_bytes().to_vec(),
            ]
            .concat();

            prog::create_program(code_id, salt, config.source, 0).expect("Failed to create domain");
        }
    }
}

// The `state()` entry point.
#[no_mangle]
extern fn state() {
    let state = unsafe { STATE.take().expect("State isn't initialized") };
    msg::reply(state, 0).expect("Failed to reply from `state()`");
}
