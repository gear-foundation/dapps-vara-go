use gstd::{collections::BTreeMap, prelude::*};
use gtest::{Program, System};
use std::fs;
use template_io::*;

fn test_input() -> HandleInput {
    let mut links = BTreeMap::new();
    links.insert(
        "website".to_string(),
        "https://vara-network.io/".to_string(),
    );

    HandleInput {
        domain: "vara-go".to_string(),
        src: Source {
            labels: vec!["vara-network".to_string()],
            header: Header {
                title: "title".to_string(),
                logo: None,
            },
            content: Content::Profile(Profile {
                title: "title".to_string(),
                links,
            }),
            footer: Footer {
                info: "info".to_string(),
            },
        },
    }
}

#[test]
fn test() {
    let system = System::new();
    system.init_logger();

    let program = Program::current(&system);
    let input = test_input();

    // Init program.
    let mut result = program.send_bytes(2, []);
    assert!(!result.main_failed());

    // Send the test handle input.
    result = program.send(0, input.clone());
    assert!(!result.main_failed());

    // State reading
    let state: State = program.read_state(b"").unwrap();
    assert_eq!(state.get(&input.domain), Some(input.src).as_ref());
}

#[allow(unused)]
fn get_state_binary() -> Vec<u8> {
    fs::read("target/wasm32-unknown-unknown/release/template_state.meta.wasm").unwrap()
}
