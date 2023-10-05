use colored::*;
use rink_runtime::Story;
use rstest::*;
use std::fs;

#[fixture]
pub fn hello_world_compiled() -> String {
    fs::read_to_string("tests/data/hello_world.json")
        .expect("Should have been able to read the file")
}

#[rstest]
fn run_hello_world(hello_world_compiled: String) {
    println!("{} {} {:?}", { "➤".blue() }, { "JSON content:".blue() }, {
        &hello_world_compiled
    });
    let mut output = String::new();
    let mut story = Story::new_from_json(&hello_world_compiled, &mut output).unwrap();
    while let Ok(()) = story.step() {}
    assert_eq!(output, "Hello, world!\nHello?\nHello, are you there?\n");
    // This is an integration test, therefore, in the end, it should only call
    // the Story struct, which is one of the only one to be exposed by the
    // library.
}
