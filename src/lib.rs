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
    let payload = msg::load::<HandleInput>().expect("Invalid payload");
    let state = unsafe { STATE.as_mut().expect("State isn't initialized") };

    // TODO:
    //
    // 1) format checks.
    // 2) use domain instead of simple data source.
    // 3) sub paths for this domain.
    // 4) integration with identity interface.
    state.insert(payload.domain, payload.src);
}

// The `state()` entry point.
#[no_mangle]
extern fn state() {
    let state = unsafe { STATE.take().expect("State isn't initialized") };
    msg::reply(state, 0).expect("Failed to reply from `state()`");
}
