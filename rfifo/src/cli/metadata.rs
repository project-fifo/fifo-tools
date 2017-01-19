use clap::{App, Arg, ArgMatches, SubCommand};
use cmd;
use serde_json;
use serde_json::Value;
use fmt;

pub fn build() -> App<'static, 'static> {
    SubCommand::with_name("metadata")
        .about("Snapshot related commands")
        .subcommand(SubCommand::with_name("get")
                    .about("Reads metadata"))
        .subcommand(SubCommand::with_name("set")
                    .about("Sets metadata")
                    .arg(Arg::with_name("key")
                         .value_name("KEY")
                         .required(true)
                         .index(1))
                    .arg(Arg::with_name("value")
                         .value_name("VALUE")
                         .required(true)
                         .index(2))
                    .arg(Arg::with_name("integer")
                         .short("i")
                         .long("integer")
                         .help("Integer value to set"))
                    .arg(Arg::with_name("string")
                         .short("s")
                         .long("string")
                         .help("String value to set"))
                    .arg(Arg::with_name("json")
                         .short("j")
                         .long("json")
                         .help("JSON value to set"))
                    .arg(Arg::with_name("float")
                         .short("f")
                         .long("float")
                         .help("Float value to set"))
        )
}

pub fn run(matches: &ArgMatches, _opts: &fmt::Opts) {
    match matches.subcommand {
        None =>
            println!("help"),
        Some(ref sub) => {
            let name = sub.name.as_ref();
            match name {
                "get" => {
                    get(&sub.matches)
                },
                "set" => {
                    set(&sub.matches)
                },
                other =>
                    println!("Sub command '{}' not implemented for metadata.", other)
            }
        }
    }
}

fn get(_app: &ArgMatches) {
    let value = cmd::run_generic("metadata-get".to_string());
    println!("deserialized = {:?}", value);
}

fn set(matches: &ArgMatches) {
    let key = value_t!(matches, "key", String).unwrap();
    let mut obj = ::std::collections::HashMap::new();
    let mut data = ::std::collections::BTreeMap::new();
    obj.insert("action", Value::String("metadata-set".to_string()));

    if matches.is_present("integer") {
        let value = value_t_or_exit!(matches, "value", i64);
        data.insert(key, Value::I64(value));
    } else if matches.is_present("float") {
        let value = value_t_or_exit!(matches, "value", f64);
        data.insert(key, Value::F64(value));
    } else if matches.is_present("json") {
        let json = value_t_or_exit!(matches, "value", String);
        let value: Value = serde_json::from_str(&json).unwrap();
        data.insert(key, value);
    } else {
        let value = value_t_or_exit!(matches, "value", String);
        data.insert(key, Value::String(value));
    }
    obj.insert("data", Value::Object(data));
    let str = serde_json::to_string(&obj).unwrap();
    let res = cmd::run_str(str);
    println!("res = {:?}", res);
}
