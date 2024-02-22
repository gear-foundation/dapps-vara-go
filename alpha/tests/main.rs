use alpha_io::{Label, Resource};
use gtest::{state_args, Program, System};
use std::{fs, path::PathBuf};

fn get_state_binary() -> Vec<u8> {
    let bin = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../target/wasm32-unknown-unknown/debug/alpha_state.meta.wasm");
    fs::read(bin).unwrap()
}

#[test]
fn test() {
    let system = System::new();
    system.init_logger();

    let program = Program::current(&system);
    let mut result = program.send_bytes(2, []);
    assert!(!result.main_failed());

    let input = Resource {
        labels: Label::list(),
        domain: "vara-go".into(),
        description: "test test".into(),
        html: "<div>hello, world<div>".into(),
        styles: "".into(),
    };
    result = program.send(2, input);
    assert!(!result.main_failed());

    let state_bin = get_state_binary();
    let search = Label::Vara;
    let output: Vec<Resource> = program
        .read_state_using_wasm(b"", "search", state_bin, state_args!(search))
        .expect("Failed to search source");
    // assert_eq!(output.get(&input.domain), Some(input.src).as_ref());
    println!("{output:?}");
}
