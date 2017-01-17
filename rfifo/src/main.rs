extern crate rustc_serialize;
extern crate clap;
extern crate serde_json;

use rustc_serialize::json;
use serde_json::Value;
use std::process::Command;

mod cli;

#[derive(RustcEncodable)]
struct GenericAction {
    action: String
}


fn run(obj: String) -> Value {
    let output = Command::new("echo")
        .arg(obj)
        .output()
        .expect("failed to execute process");
    let o = String::from_utf8_lossy(&output.stdout);
    let result: Value = serde_json::from_str(&o).unwrap();
    return result;
}

fn main() {
    // Generate a command
    let a = GenericAction{action: "zfs-list".to_string()};
    // Run a shell command with it
    let value: Value = run(json::encode(&a).unwrap());

    println!("deserialized = {:?}", value);
    println!("action = {:?}", value.pointer("/action").unwrap());

    let matches = cli::build_cli().get_matches();

    println!("matches = {:?}", matches);

}
