use colored::*;
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
        hello_world_compiled
    });
    todo!("TODO: Implement the parsing & runtime to make this test pass");
    // This is an integration test, therefore, in the end, it should only call
    // the Story struct, which is one of the only one to be exposed by the
    // library.
}
