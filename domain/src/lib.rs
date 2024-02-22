#![no_std]

use gstd::{msg, prelude::*, ActorId};
use template_io::*;

/// Admin of this domain.
static mut ADMIN: ActorId = ActorId::zero();

/// The resource of the domain.
static mut STATE: Option<Source> = None;

// The `init()` entry point.
#[no_mangle]
extern fn init() {
    unsafe { ADMIN = msg::source() }
    unsafe { STATE = None }

    // TODO:
    //
    // register domain to the router.
    //
    // let domain = msg::load::<String>().expect("Invalid payload");
}

// The `handle()` entry point.
#[no_mangle]
extern fn handle() {
    let source = msg::load::<Source>().expect("Invalid payload");
    let state = unsafe { STATE.as_mut().expect("State isn't initialized") };
    *state = source;

    // TODO:
    //
    // register labels to the router
}

// The `state()` entry point.
#[no_mangle]
extern fn state() {
    let state = unsafe { STATE.take().expect("State isn't initialized") };
    msg::reply(state, 0).expect("Failed to reply from `state()`");
}
