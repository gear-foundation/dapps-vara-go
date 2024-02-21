#![no_std]

use gstd::{collections::BTreeMap, msg, prelude::*, ActorId};
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
    let Some(payload) = msg::load() else {
        msg::reply(HandleOutput::InvalidPayload, 1).expect("Failed to reply from `handle()`");
        return;
    };

    if let HandleInput { id, src } = payload {
        let state = unsafe { STATE.as_mut().expect("State isn't initialized") };

        // TODO: duplicated domain checks.
        state.insert(id, src);

        msg::reply(HandleOutput::Success, 0).expect("Failed to reply from `handle()`");
    }
}

// The `state()` entry point.
#[no_mangle]
extern fn state() {
    let state = unsafe { STATE.take().expect("State isn't initialized") };
    msg::reply(State::from_iter(state), 0).expect("Failed to reply from `state()`");
}
