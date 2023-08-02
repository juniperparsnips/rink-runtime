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
}
