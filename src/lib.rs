#![no_std]

use gstd::{collections::BTreeMap, msg, prelude::*};
use template_io::*;

static mut STATE: Option<BTreeMap<String, Source>> = None;

// The `init()` entry point.
#[no_mangle]
extern fn init() {
    unsafe { STATE = Some(Default::default()) }
}

// The `handle()` entry point.
#[no_mangle]
extern fn handle() {
    let Ok(payload) = msg::load::<HandleInput>() else {
        msg::reply(HandleOutput::InvalidPayload, 1).expect("Failed to reply from `handle()`");
        return;
    };

    let state = unsafe { STATE.as_mut().expect("State isn't initialized") };

    // TODO: duplicated domain checks.
    state.insert(payload.id, payload.src);

    msg::reply(HandleOutput::Success, 0).expect("Failed to reply from `handle()`");
}

// The `state()` entry point.
#[no_mangle]
extern fn state() {
    let state = unsafe { STATE.take().expect("State isn't initialized") };
    msg::reply(state, 0).expect("Failed to reply from `state()`");
}
