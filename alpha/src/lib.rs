#![no_std]

use alpha_io::{Resource, State};
use gstd::{msg, prelude::*};

/// The resource of the domain.
static mut STATE: Option<State> = None;

// do nothing atm.
#[no_mangle]
extern fn init() {
    unsafe {
        STATE = Some(vec![]);
    }
}

// The `handle()` entry point.
#[no_mangle]
extern fn handle() {
    let res = msg::load::<Resource>().expect("Failed to load handle input");
    let state = unsafe { STATE.as_mut().expect("State isn't initialized") };
    state.push(res);
}

// The `state()` entry point.
#[no_mangle]
extern fn state() {
    let state = unsafe { STATE.take().expect("State isn't initialized") };
    msg::reply(state, 0).expect("Failed to reply from `state()`");
}
