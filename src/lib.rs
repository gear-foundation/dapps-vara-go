#![no_std]

use gstd::{msg, prelude::*};
use template_io::*;

static mut STATE: Option<State> = None;

// The `init()` entry point.
#[no_mangle]
extern fn init() {
    unsafe { STATE = Some(Default::default()) }
}

// The `handle()` entry point.
#[no_mangle]
extern fn handle() {
    let payload = msg::load::<HandleInput>().expect("Invalid payload");
    let state = unsafe { STATE.as_mut().expect("State isn't initialized") };
    let sender = msg::source();

    // TODO:
    //
    // 1) format checks.
    // 4) integration with identity interface.

    // Just overriding the content atm.
    let domain = state
        .entry(payload.domain.clone())
        .or_insert_with(|| Domain {
            owner: sender,
            paths: Default::default(),
        });

    if domain.owner != sender {
        // Do nothing atm.
        //
        // TODO: Returns the error to the client.
        return;
    }

    domain.paths.insert(payload.path, payload.src);
}

// The `state()` entry point.
#[no_mangle]
extern fn state() {
    let state = unsafe { STATE.take().expect("State isn't initialized") };
    msg::reply(state, 0).expect("Failed to reply from `state()`");
}
