use std::process::Command;
use serde_json;
use serde_json::Value;
use rustc_serialize::json;
use rustc_serialize::Encodable;

#[derive(RustcEncodable)]
struct GenericReq {
    action: String
}

pub fn run_generic(action: String) -> Value {
    let action = GenericReq{action: action};
    run(action)
}

// run is a generic function that takes any
// encoable, this will make it easyer to allow
// for calling it with `RustcEncodable` structs
// from the outside
pub fn run<E: Encodable>(obj: E) -> Value {
    let str = json::encode(&obj).unwrap();
    let output = Command::new("echo")
        .arg(str)
        .output()
        .expect("failed to execute process");
    let o = String::from_utf8_lossy(&output.stdout);
    let result: Value = serde_json::from_str(&o).unwrap();
    return result;
}
